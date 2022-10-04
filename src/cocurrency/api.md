
### Struct std::sync::Condvar

- 一个条件变量
条件变量代表其具有阻塞一个线程的能力，且在阻塞期间不消耗`CPU`时间。条件变量通常和一个持有布尔值的可变锁`mutex`一起使用，布尔值的作用在于检测是否达到了条件。这个布尔检测值通常在可变锁`mutex`锁定后被进行验证，验证结果决定是否需要使用条件变量对线程进行`block`。

此模块的函数将阻塞当前线程。注意用同一个条件变量中的方法取调用多个互斥锁的话将会引起恐慌。
```rust{.line-numbers}
use std::sync::{Arc, Mutex, Condvar};
use std::thread;

let pair = Arc::new((Mutex::new(false), Condvar::new()));
let pair2 = Arc::clone(&pair);

// Inside of our lock, spawn a new thread, and then wait for it to start.
thread::spawn(move|| {
    let (lock, cvar) = &*pair2;
    let mut started = lock.lock().unwrap();
    *started = true;
    // We notify the condvar that the value has changed.
    cvar.notify_one();
});

// Wait for the thread to start up.
let (lock, cvar) = &*pair;
 // 不锁 start 的话，当前线程 start == false 然后被block，而同时正好另一个线程将start改成了 true，这样搞就变成死锁了
let mut started = lock.lock().unwrap(); 
while !*started {
    started = cvar.wait(started).unwrap();
}
```

- source
pub fn wait<'a, T>(
    &self,
    guard: MutexGuard<'a, T>
) -> LockResult<MutexGuard<'a, T>>

> 阻塞当前线程直到该条件变量接收到一个通知。
该方法会自动解锁由`guard`参数表示的`mutex`锁并阻塞当前线程。这意味着任何在`mutex`解锁后发生的 notify_one 和 notify_all 方法的调用，都会唤醒该线程，当wake() 方法返回时，该`mutex`锁被再次锁定。

#### Errors
如果`Mutex`锁是中毒状态的话，wait 该锁会返回 error

#### Panic
当本方法同时用于多个 Mutex 锁时，会发生恐慌. ?

