
use std::sync::mpsc::{channel, Sender, Receiver};
use std::sync::{Arc, Mutex, Condvar};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

/// 思路就是：
/// 1、生成一个channel，将发送端放入 thread_pool，thread_pool在join方法中通过 condvar.wait() 等待唤醒；
/// 2、Build里生成多个线程，和多个持有接收端的 thread_pool_shared_data, 在每个下线程里用闭包获取thread_pool_shared_data的所有权，然后通过channel.recv block线程
/// 3、通过 thread_pool.execute(fn) 将fn任务发送到 channel中，被获取管道控制权的线程 recv() 并处理。
/// 4、线程处理完任务后notify_all()，然后继续 recv() 等待。
/// 5、thread_pool在 condvar.wait()中被唤醒，检查有没有正在执行的线程和待执行任务，如果没有，则退出join()，主线程结束，同时子线程也结束了。
/// 6、如果子线程在任务处理中有发送恐慌，此线程被结束了，这是通过守卫对象 sentinel 的 drop 方法进行检测，如果发生恐慌会再次创建一个线程进行补偿
/// 
/// 

// 定义任务：FnBox ===========================
trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<Self>) {
        (*self)()
    }
}

type Thunk<'a> = Box<dyn FnBox + Send + 'a>;

// 定义各线程分享的数据 =======================
struct ThreadPoolSharedData {
    name: Option<String>,
    job_receiver: Mutex<Receiver<Thunk<'static>>>,
    empty_trigger: Mutex<()>,
    empty_condvar: Condvar,
    queued_count: AtomicUsize,  // 任务队列的待分配任务数
    active_count: AtomicUsize,  // 正在运行任务的线程数
    max_thread_count: AtomicUsize, // 可容纳的最大线程数
    panic_count: AtomicUsize, // 发生恐慌的线程数
    stack_size: Option<usize>, // 线程中栈的大小
}

impl ThreadPoolSharedData {
    fn has_work(&self) -> bool {
        self.queued_count.load(Ordering::SeqCst) > 0 
        || 
        self.active_count.load(Ordering::SeqCst) > 0
    }

    fn no_work_notify_all(&self) {
        if !self.has_work() {
            // 此处加锁得目的是为了在别的地方调用 condvar.wait() 前，condvar.notify_all() 不会被提前调用，不然 condvar.wait() 就一直阻塞了
            *self.empty_trigger.lock().expect("Unable to notify all threads"); 
            self.empty_condvar.notify_all();
        }
    }
}

// 定义线程池 =================================
pub struct ThreadPool {
    jobs: Sender<Thunk<'static>>,
    shared_data: Arc<ThreadPoolSharedData>,
} 

impl ThreadPool {
    pub fn new(num_theads: usize) -> ThreadPool {
        PoolBuilder::new().num_threads(num_theads).build()
    }

    pub fn execute<F>(&self, job: F)
        where F: FnOnce() + Send + 'static 
    {
        // 待执行任务数 + 1
        self.shared_data.queued_count.fetch_add(1, Ordering::SeqCst); // 用了 SeqCst 是为了保证 job.send 的内存顺序吧？
        //将任务发送给 channel
        self.jobs.send(Box::new(job)).expect("unable to send job into queue ");

    }

    pub fn join(&self) {
        if self.shared_data.has_work() == false { return (); }
        // 此处加锁得目的是在 ThreadPoolSharedData 中的 empty_condvar.notify_all() 被调用前，调用empty_condvar.wait() 阻塞当前线程，等待 empty_condvar.notify_all() 唤醒
        let mut lock = self.shared_data.empty_trigger.lock().unwrap();
        while self.shared_data.has_work() {
            lock = self.shared_data.empty_condvar.wait(lock).unwrap();
            lock = self.shared_data.empty_trigger.lock().unwrap();
            println!("wait");
        }
    }
}

// 添加 PoolBuilder ===============================
pub struct PoolBuilder {
    num_threads: Option<usize>,
    thread_name: Option<String>,
    thread_stack_size: Option<usize>,
}

impl PoolBuilder {
    pub fn new() -> Self {
        PoolBuilder {
            num_threads : None,
            thread_name: None,
            thread_stack_size: None,
        }
    }

    pub fn num_threads(mut self, num_threads: usize) -> PoolBuilder {
        assert!(num_threads > 0);
        self.num_threads = Some(num_threads);
        self
    }

    pub fn build(self) -> ThreadPool {
        let (tx, rx) = channel::<Thunk<'static>>();
        let num_threads = self.num_threads.unwrap_or_else(num_cpus::get);
        let shared_data = Arc::new(ThreadPoolSharedData {
            name: self.thread_name,
            job_receiver: Mutex::new(rx),
            empty_trigger: Mutex::new(()),
            empty_condvar: Condvar::new(),
            queued_count: AtomicUsize::new(0),
            active_count: AtomicUsize::new(0),
            max_thread_count: AtomicUsize::new(num_threads),
            panic_count: AtomicUsize::new(0),
            stack_size: self.thread_stack_size,
        });

        for _ in 0..num_threads {
            spawn_in_pool(shared_data.clone());
        }

        ThreadPool {
            jobs: tx,
            shared_data,
        }
    }
}

fn spawn_in_pool(shared_data: Arc<ThreadPoolSharedData>) {
    let mut builder = thread::Builder::new();
    if let Some(ref name) = shared_data.name {
        builder = builder.name(name.clone());
    }
    if let Some(ref stack_size) = shared_data.stack_size {
        builder = builder.stack_size(stack_size.to_owned());
    }

    builder.spawn(move || {
        let sentinel = Sentinel::new(&shared_data);
        loop {
            let thread_counter_val = shared_data
                .active_count.load(Ordering::Acquire);
            let max_thread_count_val = shared_data
                .max_thread_count.load(Ordering::Relaxed);
            if thread_counter_val >= max_thread_count_val {
                break;
            }
            let message = {
                let lock = shared_data.job_receiver.lock().expect("unable to lock job_receiver"); // 锁定channel
                let a = lock.recv(); // 阻塞进程
                a
            }; // 离开时解锁channel

            let job = match message {
                Ok(job) => job,
                Err(..) => break,
            };

            shared_data.queued_count.fetch_sub(1, Ordering::SeqCst);
            shared_data.active_count.fetch_add(1, Ordering::SeqCst);
            // 线程执行任务时，任务内部可能发生恐慌
            job.call_box();  
            shared_data.active_count.fetch_sub(1, Ordering::SeqCst);
            shared_data.no_work_notify_all();
            println!("notify");
        }
        sentinel.cancel();
    }).unwrap();
}

struct Sentinel<'a> {
    shared_data: &'a Arc<ThreadPoolSharedData>,
    active: bool,
}

impl<'a> Sentinel<'a> {
    fn new(shared_data:&'a Arc<ThreadPoolSharedData>) -> Sentinel<'a> {
        Sentinel {
            shared_data,
            active: true,
        }
    }

    fn cancel(mut self) {
        self.active = false;
    }
}

impl<'a> Drop for Sentinel<'a> {
    fn drop(&mut self) {
        // 只有当线程内部执行任务发生恐慌时，才有可能 self.active == true
        if self.active {
            self.shared_data.active_count.fetch_sub(1, Ordering::SeqCst);
            if thread::panicking() {
                self.shared_data.panic_count.fetch_add(1, Ordering::SeqCst);
            }
            // 任务内部发生恐慌，则取消唤醒wait的条件变量，并成一个新的线程，并调用线程中channel.recv()让其阻塞，直到接收到任务。
            self.shared_data.no_work_notify_all();
            spawn_in_pool(self.shared_data.clone());
        }
    }
}

fn main() {
    let pool = ThreadPool::new(8);
    let test_count = Arc::new(AtomicUsize::new(0));
    for _ in 0..1 {
        let test_count = test_count.clone();
        pool.execute(move || {
            test_count.fetch_add(1, Ordering::Relaxed);
        });
    }
    pool.join();
    println!("{}", test_count.load(Ordering::Relaxed));
    assert_eq!(1, test_count.load(Ordering::Relaxed));
}