### Rust的原子(Atomic)类型与内存顺序(Memory Ordering)

Rust编程语言在1.34之后的版本中开始正式提供完整的原子(Atomic)类型了。所谓的原子是指一系列不可被上下文交换(Context Switch)的机器指令，这些机器指令组成的操作又称为原子操作(Atomic Operation)。在多CPU内核的环境下，当某个CPU内核开始运行原子操作时，就会先暂停其它CPU内核对内存的操作，以保证在原子操作运行的过程中，内存内容不会受到其它CPU内核干扰。所以原子操作若用得好，就不需要去使用会拖累程序性能的互斥锁(Mutex)或是消息传递(message passing)机制。只不过依靠原子操作来解决同步问题的话，会牵扯到编译器优化以及CPU架构的问题，这篇文章会针对Rust编程语言提供的原子类型来探讨原子操作。

先来看看以下这个程序吧！
```rust
use std::thread::{self, JoinHandle};
 
const N_TIMES: u64 = 1000;
const N_THREADS: usize = 10;
 
static mut R: u64 = 0;
 
fn reset() {
    unsafe {
        R = 0;
    }
}
 
fn add_n_times(n: u64) -> JoinHandle<()> {
    thread::spawn(move || {
        for _ in 0..n {
            unsafe {
                R += 1;
            }
        }
    })
}
 
fn main() {
    loop {
        reset();
 
        let mut threads = Vec::with_capacity(N_THREADS);
 
        for _ in 0..N_THREADS {
            threads.push(add_n_times(N_TIMES));
        }
 
        for thread in threads {
            thread.join().unwrap();
        }
 
        assert_eq!(N_TIMES * N_THREADS as u64, unsafe { R });
    }
}
```
全域静态变量R是可变的。在main函数的无穷循环中，每次迭代会将R设为0，并且创建出N_THREADS个线程，每个线程都会运行N_TIMES次的R += 1。也就是说，在理想状态下，我们可以预估无穷循环的每次迭代，R += 1都会被运行N_TIMES * N_THREADS次，所以在该次迭代的最后，R的值也应为N_TIMES * N_THREADS才对。

然而现实并非如此，当N_THREADS大于1时，R的值在每次迭代的最后，有可能会小于N_TIMES * N_THREADS，这就是所谓的竞跑现象(Data Race)。竞跑之所以会发生，是由于R += 1这行叙述，虽然它只有一行，但它在实际运行时却可能是由好几个步骤组成，例如：第一步，将R读取进CPU寄存器；第二步，将该寄存器的值加一；第三步，将该寄存器的值回存给R。此时若使用超过一个线程来运行R += 1这行叙述的话，有可能会变成这样：当第一个线程运行完第二步，还没到第三步时，第二个线程就先运行第一步了，所以会造成第二个线程读取到原本应该已经被第一个线程加一却还没回存好的R。也有可能会变成这样：当第一个线程运行完第一步后，第二个线程却运行完两轮R += 1了，此时第一个线程再继续把第二步和第三步做完反而会让R的值比先前的更小！

使用互斥锁可以很轻易地解决这个问题，程序如下：
```rust
use std::thread::{self, JoinHandle};
use std::sync::{Mutex, Arc}; // 1
 
const N_TIMES: u64 = 1000;
const N_THREADS: usize = 10;
 
static mut R: u64 = 0;
 
fn reset() {
    unsafe {
        R = 0;
    }
}
 
fn add_n_times(n: u64, mutex: Arc<Mutex<()>>) -> JoinHandle<()> {
    thread::spawn(move || {
        for _ in 0..n {
            let lock = mutex.lock().unwrap(); // 2
            // critical section START
            unsafe {
                R += 1;
            }
            // critical section END
            drop(lock); // 3
        }
    })
}
 
fn main() {
    let mutex = Arc::new(Mutex::new(())); // 4
 
    loop {
        reset();
 
        let mut threads = Vec::with_capacity(N_THREADS);
 
        for _ in 0..N_THREADS {
            threads.push(add_n_times(N_TIMES, mutex.clone())); // 5
        }
 
        for thread in threads {
            thread.join().unwrap();
        }
 
        assert_eq!(N_TIMES * N_THREADS as u64, unsafe { R });
    }
}
```
利用互斥锁形成的临界区段(Critical Section)，来使R += 1程序叙述在同一时间只能够被一个线程来运行。不过功能强大的互斥锁会需要耗费不少额外的运算资源，在这个例子中，我们其实只需利用Rust编程语言的原子类型就能达到相同的结果，性能也会好很多。

程序如下：
```rust
use std::thread::{self, JoinHandle};
use std::sync::atomic::{Ordering, AtomicU64}; // 1
 
const N_TIMES: u64 = 1000;
const N_THREADS: usize = 10;
 
static R: AtomicU64 = AtomicU64::new(0); // 2
 
fn reset() {
    R.store(0, Ordering::Relaxed); // 3
}
 
fn add_n_times(n: u64) -> JoinHandle<()> {
    thread::spawn(move || {
        for _ in 0..n {
            R.fetch_add(1, Ordering::Relaxed); // 4
        }
    })
}
 
fn main() {
    loop {
        reset();
 
        let mut threads = Vec::with_capacity(N_THREADS);
 
        for _ in 0..N_THREADS {
            threads.push(add_n_times(N_TIMES));
        }
 
        for thread in threads {
            thread.join().unwrap();
        }
 
        assert_eq!(N_TIMES * N_THREADS as u64, R.load(Ordering::Relaxed)); // 5
    }
}
```
以上程序，使用AtomicU64来替换原本的u64，我们甚至还可以把全域静态变量R的mut关键字拿掉，因为原子类型的值就算不使用mut也还是可变的，这点和被Mutex类型包裹的值一样。

例如以下程序可以通过编译：
```rust
use std::sync::Mutex;
use std::sync::atomic::{Ordering, AtomicU64};
 
struct Counter {
    count: u64
}
 
fn main() {
    let n = Mutex::new(Counter {
        count: 0
    });
 
    n.lock().unwrap().count += 1;
 
    let n = AtomicU64::new(0);
 
    n.fetch_add(0, Ordering::Relaxed);
}
```
在使用原子类型提供的原子操作时，需要额外传入一个Ordering枚举的变体实体。这个Ordering枚举可不是std::cmp这个模块下用来比大小的Ordering哦！而是位于std::sync::atomic模块下的Ordering枚举。

这边的Ordering枚举是用来控制原子操作时所使用的「内存顺序」(Memory Ordering)的限制，共有Relaxed、Acquire、Release、AcqRel、SeqCst五种变体。

### 内存顺序

内存顺序是指CPU在访问内存时的顺序，这个顺序不单纯是程序叙述的撰写顺序，可能还会因编译器优化，在编译阶段发生改变(reordering)，也可能在运行阶段时，因CPU的缓存机制而被打乱顺序。

举个例子，在编译以下程序叙述时：
```rust
static mut X: u64 = 0;
static mut Y: u64 = 1;

fn main() {
    ...     // A

    unsafe {
        ... // B
        X = 1;
        ... // C
        Y = 3;
        ... // D
        X = 2;
        ... // E
    }
}
```
如果C、D段落根本没有用到X = 1，那么编译器很可能会直接将X = 1和X = 2合并在一起，变成：
```rust
static mut X: u64 = 0;
static mut Y: u64 = 1;
 
fn main() {
    ...     // A
 
    unsafe {
        ... // B
        X = 2;  // 1
        ... // C
        Y = 3;
        ... // D  // 2
        ... // E  // 3
    }
}
```
此时若段落A中有使用新的线程来读取全域静态变量X，则不可能会读取到当X的值为1时的结果，因为在编译阶段时就被编译器给省略掉了！

另一方面，假设X = 1并没有被编译器省略掉好了，并且在段落A中有一个新线程，主线程和段落A的线程对于全域变量的运行顺序关系如下：
```rust
initial state: X = 0, Y = 1
 
THREAD Main     THREAD A
X = 1;          if X == 1 {
Y = 3;              Y *= 2;
X = 2;          }
```
- Y = 3：THREAD A运行完后才运行THREAD Main。或是THREAD Main运行完后才运行THREAD A。
- Y = 6：THREAD Main运行完Y = 3后，运行THREAD A。THREAD A运行完后，THREAD Main才继续运行完。

而实际上我们却有可能会得到以下这种状态：

- Y = 2：THREAD Main正在运行Y = 3，THREAD A此时也开始运行Y *= 2。3这个值来不及回存到Y，Y就被Y *= 2先行取用了(此时取到的Y为1)，而当3这个值终于回存到Y后，Y *= 2才计算完成，所以Y的值变成2。

上述只是一般的竞跑，更极端一点由CPU缓存引起的内存顺序问题还有以下这个：

- Y = 2：THREAD Main虽然已经确实运行完Y = 3了，但是该CPU缓存中的Y = 3还没同步到其它CPU的缓存中，此时THREAD A的Y *= 2就开始读取Y，因此它读到的Y值为1，计算之后就出现Y = 2的结果。

甚至即便改成：

```rust
initial state: X = 0, Y = 1
 
THREAD Main     THREAD A
X = 1;          if X == 2 {
Y = 3;              Y *= 2;
X = 2;          }
````

### 原子操作与内存顺序
#### Relaxed
Relaxed只会进行单纯的原子操作，并不会对内存顺序进行任何限制。换句话说，它可以最大幅度地保留编译器优化的程度，不过如果想要在多个原子操作间实现跨线程的同步机制，就得采用其它的内存顺序的限制方式了。

例如以下这个范例：
```rust
use std::thread::{self, JoinHandle};
 
static mut DATA: u64 = 0;
static mut READY: bool = false;
 
fn reset() {
    unsafe {
        DATA = 0;
        READY = false;
    }
}
 
fn producer() -> JoinHandle<()> {
    thread::spawn(move || {
        unsafe {
            DATA = 100;                // A
            READY = true;              // B
        }
    })
}
 
fn consumer() -> JoinHandle<()> {
    thread::spawn(move || {
        unsafe {
            while !READY {}           // C
 
            assert_eq!(100, DATA);    // D
        }
    })
}
 
fn main() {
    loop {
        reset();
 
        let t_producer = producer();
        let t_consumer = consumer();
 
        t_producer.join().unwrap();
        t_consumer.join().unwrap();
    }
}
```

单就程序逻辑来看，这样的程序似乎是挺安全的：当READY为true时，DATA一定是100。但是，实际上，这个程序在经过编译器优化或是CPU缓存的影响后，可能会让C行在读取到READY是true后，D行读取到的DATA却还是0。

这东西很反直觉的原因是我们可能比较常在x86或是x86_64架构上的CPU开发程序，而x86或是x86_64架构的CPU属于「强有序」(strongly-ordered)的内存模型，不太会发生内存顺序的问题。但是在如ARM架构等使用「弱有序」(weakly-ordered)的内存模型下，内存顺序就很有可能会被打乱。

即便我们将以上程序的READY，使用原子类型搭配Relaxed内存顺序限制来修改，问题也是依旧存在的。
```rust
use std::thread::{self, JoinHandle};
use std::sync::atomic::{Ordering, AtomicBool}; // 1
 
static mut DATA: u64 = 0;
static READY: AtomicBool = AtomicBool::new(false); // 2
 
fn reset() {
    unsafe {
        DATA = 0;
    }
    READY.store(false, Ordering::Relaxed);
}
 
fn producer() -> JoinHandle<()> {
    thread::spawn(move || {
        unsafe {
            DATA = 100;                                 // A
        }
        READY.store(true, Ordering::Relaxed);           // B  // 3
    })
}
 
fn consumer() -> JoinHandle<()> {
    thread::spawn(move || {
        while !READY.load(Ordering::Relaxed) {}         // C  // 4
 
        assert_eq!(100, unsafe { DATA });               // D
    })
}
 
 
fn main() {
    loop {
        reset();
 
        let t_producer = producer();
        let t_consumer = consumer();
 
        t_producer.join().unwrap();
        t_consumer.join().unwrap();
    }
}
```
#### AcqRel
借由Acquire和Release这两个内存顺序的限制，可以构筑出一对内存屏障(Memory Barrier)，或称内存栅栏(Memory Fence)，防止编译器和CPU将屏障前(Release)和屏障后(Acquire)中的数据操作重新排在屏障围成的范围之外。

如下：

```rust
use std::thread::{self, JoinHandle};
use std::sync::atomic::{Ordering, AtomicBool};
 
static mut DATA: u64 = 0;
static READY: AtomicBool = AtomicBool::new(false);
 
fn reset() {
    unsafe {
        DATA = 0;
    }
    READY.store(false, Ordering::Relaxed);
}
 
fn producer() -> JoinHandle<()> {
    thread::spawn(move || {
        unsafe {
            DATA = 100;                                 // A
        }
        READY.store(true, Ordering::Release);           // B: memory fence ↑ // 1
    })
}
 
fn consumer() -> JoinHandle<()> {
    thread::spawn(move || {
        while !READY.load(Ordering::Acquire) {}         // C: memory fence ↓ // 2
 
        assert_eq!(100, unsafe { DATA });               // D
    })
}
 
 
fn main() {
    loop {
        reset();
 
        let t_producer = producer();
        let t_consumer = consumer();
 
        t_producer.join().unwrap();
        t_consumer.join().unwrap();
    }
}
```
原则上，Acquire用于读取，而Release用于写入。但是由于有些原子操作同时拥有读取和写入的功能，此时就需要使用AcqRel来设置内存顺序了。在内存屏障中被写入的数据，都可以被其它线程读取到，不会有CPU缓存的问题。

#### SeqCst
SeqCst就像是AcqRel的加强版，它不管原子操作是属于读取还是写入的操作，只要某个线程有用到SeqCst的原子操作，线程中该SeqCst操作前的数据操作绝对不会被重新排在该SeqCst操作之后，且该SeqCst操作后的数据操作也绝对不会被重新排在SeqCst操作前。

另外，Acquire、Release和AcqRel等也可以与SeqCst搭配使用，来构筑出一对内存屏障。

> 问：一个常见的结构体，里面有mutex，和被它保护的数据成员，mutex加锁本身是原子操作的，那么怎么保证，mutex这个变量被锁住之后，结构体里其他变量能读到最新值呢？

以parking_lot 这个库的Mutex为例，可见lock时，使用acquire，其后的数据访问不会被重排到其之前；unlock使用release，其前的访问不会被排到其后。就保证lock之后能acquire到之前的写入，unlock之后能release之前的写入。