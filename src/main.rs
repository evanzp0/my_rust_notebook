#![allow(unused)]
// #![feature(generators, generator_trait)]
// #![feature(negative_impls)]

// use std::collections::HashMap;
// use std::marker::{PhantomData, self};
// use std::panic;
// use std::any::Any;
// use std::fmt::Display;
// use std::ptr::NonNull;
// use std::sync::mpsc::{channel, sync_channel};
// use std::sync::{Arc, Mutex, Barrier, Condvar};
// use std::thread::{Builder, current, sleep};
// use std::time::Duration;
// use std::{mem::size_of, borrow::Cow};
// use std::ffi::CStr;
// use std::os::raw::c_char;
// use std::cell::{Cell, RefCell};
// use std::thread;

// static B: [u8; 10] = [99, 97, 114, 114, 121, 116, 111, 119,  101, 108];
// static C: [u8; 11] = [116, 104, 97, 110, 107, 115, 102, 105,  115, 104, 0];

// #[derive(Debug)]
// struct Sa {
//     // a: *const i32
//     _data: PhantomData<NonNull<u8>>
// }

// struct Sb <Slayer>
// where Slayer : Display
// {
//     nm: Slayer,
// }

// impl !Sync for Sa {
    
// }
// unsafe impl Send for Sa{}

// use std::ops::Sub;
// use std::sync::atomic::{AtomicU64, Ordering};
// use std::thread::{ JoinHandle};
// use std::time::Instant;

// const N_TIMES: u64 = 10000000;
// const N_THREADS: usize = 10;

// static R: AtomicU64 = AtomicU64::new(0);

// fn add_n_times(n: u64) -> JoinHandle<()> {
//     thread::spawn(move || {
//         for _ in 0..n {
//             R.fetch_add(1, Ordering::Relaxed);
//         }
//     })
// }

use std::{marker::PhantomPinned, pin::Pin, ops::Deref, future::Future, any::Any};

struct A;
impl Drop for A {
    fn drop(&mut self) {
        println!("drop A");
    }
}

impl A {
    fn get_b(self) -> B {
        println!("get_b");
        B
    }
}

struct B;
impl Drop for B {
    fn drop(&mut self) {
        println!("drop B");
    }

}

impl B {
    fn get_c(self) -> C {
        println!("get_c");
        C
    }
}

struct C;
impl Drop for C {
    fn drop(&mut self) {
        println!("drop C");
    }
}

impl C {
    fn end_c(&self) -> bool {
        true
    }
}

fn main() {
    let a = A;
    match a.get_b().get_c().end_c() {
        true => println!("end C"),
        false => println!("end C?"),
    }
    println!("main end");
}

fn meth() {
    

    #[derive(Debug)]
    struct Ms {
        a: Box<i32>,
        b: * const Box<i32>,
    }

    let mut m1 = Ms {
        a: Box::new(1),
        b: std::ptr::null(),
    };

    m1.b = &m1.a as *const Box<i32>;
    unsafe {
        println!("{}", *m1.b);
        println!("{:p}, {:p}, {}", &m1.a, m1.a, m1.a);
        println!("{:p}, {:p}, {:p}", &m1.b, m1.b, *m1.b);
    }

    let mut m2 = Ms {
        a: Box::new(2),
        b: std::ptr::null(),
    };

    m2.b = &m2.a as *const Box<i32>;
    unsafe {
        println!("{}", *m2.b);
        println!("{:p}, {:p}, {}", &m2.a, m2.a, m2.a);
        println!("{:p}, {:p}, {:p}", &m2.b, m2.b, *m2.b);
    }

    m2 = m1;
    // m2.a = Box::new(3);
    unsafe {
        println!("{}", *m2.b);
        println!("{:p}, {:p}, {}", &m2.a, m2.a, m2.a);
        println!("{:p}, {:p}, {:p}", &m2.b, m2.b, *m2.b);
    }

    // 1
    // 0xbf3bbbf0e8, 0x1c511a3ddc0, 1
    // 0xbf3bbbf0f0, 0xbf3bbbf0e8, 0x1c511a3ddc0
    // 2
    // 0xbf3bbbf210, 0x1c511a3dde0, 2
    // 0xbf3bbbf218, 0xbf3bbbf210, 0x1c511a3dde0
    // 1
    // 0xbf3bbbf210, 0x1c511a3ddc0, 1
    // 0xbf3bbbf218, 0xbf3bbbf0e8, 0x1c511a3ddc0



// m2.a = Box::new(3); 加这行后

// 1
// 0xb73b2ff170, 0x22943d2dd90
// 0xb73b2ff178, 0xb73b2ff170, 0x22943d2dd90
// 2
// 0xb73b2ff288, 0x22943d2ddb0
// 0xb73b2ff290, 0xb73b2ff288, 0x22943d2ddb0
// 1137892816
// 0xb73b2ff288, 0x22943d2ddb0
// 0xb73b2ff290, 0xb73b2ff170, 0x22943d2dd90

}

// use std::{sync::{Mutex, Arc, Condvar, atomic::{AtomicUsize, Ordering}}, thread, time::Duration};
// use crossbeam::select;
// use std::thread;
// use std::time::Duration;
// use crossbeam_channel::unbounded;

// let (s1, r1) = unbounded();
// let (s2, r2) = unbounded();

// thread::spawn(move || {s1.send(10).unwrap(); Duration::from_secs(2);});
// thread::spawn(move || {s2.send(20).unwrap(); Duration::from_secs(2); });
// select! {
//     recv(r1) -> msg =>{ assert_eq!(msg, Ok(10)); println!("{:?}", msg)},
//     recv(r2) -> msg => { assert_eq!(msg, Ok(20)); println!("{:?}", msg) },
//     default(Duration::from_secs(1)) => println!("timed out"),
// }

// let pair = Arc::new((Mutex::new(()), Condvar::new()));
// let count = Arc::new(AtomicUsize::new(2));
// let t1 = {
//     let pair = pair.clone();
//     let count = count.clone();
//     thread::spawn(move || {
//         // thread::sleep(Duration::from_secs(3));
//         let mut start = pair.0.lock().unwrap();
//         count.fetch_sub(1, Ordering::Relaxed);
//         pair.1.notify_all();
//         println!("11");
//     });
// };

// let mut start = pair.0.lock().unwrap();
// while count.load(Ordering::Relaxed) > 0 {
//     println!("== {}", count.load(Ordering::Relaxed));
//     start = pair.1.wait(start).unwrap();
//     // start = pair.0.lock().unwrap();   // block 
//     println!("..");
// }


    // let t2 = {
    //     let pair = pair.clone();
    //     let count = count.clone();
    //     thread::spawn(move || {
    //         thread::sleep(Duration::from_secs(3));
    //         let mut start = pair.0.lock().unwrap();
    //         count.fetch_sub(1, Ordering::Relaxed);
    //         pair.1.notify_all();
    //         println!("22");
    //     });
    // };

// thread_local!{static FOO: Cell<i32>  = Cell::new(1)};

// FOO.with(|f| {
//     assert_eq!(1, (*f).get());
//     (*f).set(2);
// });

// thread::spawn(|| {
//     FOO.with(|f| {
//         assert_eq!(1, (*f).get());
//         (*f).set(3);
//     });
// });

// FOO.with(|f| {
//     assert_eq!(2, (*f).get());
// });

// let mut v = vec![];
// let size: usize = 1024 * 1024;
// for id in 1..10 {
//     let thread_name = format!("t_{}", id);
//     let bd = Builder::new().name(thread_name).stack_size(size);
//     let child = bd.spawn(move || {
//         println!("{}", id);

//         if id == 3 {
//             panic::catch_unwind(|| {
//                 panic!("oh no!");
//             }).ok();
//         }
//         println!("{}", current().name().unwrap());
//     }).unwrap();
//     v.push(child);
// }

// println!("before join");
// for c in v {
//     c.join();
// }

// println!("main end");

// fn leakit() -> &'static mut String {
//     let s = "abcde".to_owned();
//     println!("s addr: {:p}", &s);
//     let s1 = s.as_ptr();
//     println!("s raw addr: {:p}", s1);
//     let a = Box::new(s);
//     println!("a point to addr: {:p}", a);
//     let b = Box::leak(a);
//     println!("b point to addr: {:p}", b);
//     println!("b raw addr: {:p}", b.as_ptr());

//     b
// }


//    println!("*a address {:p}", &*a);
// let a: usize = 42;
// let b: String;
// let c: Cow<str>;

// unsafe {
//     let B_ptr: *mut u8 = &B as *const u8 as *mut u8;
//     b = String::from_raw_parts(B_ptr, 10, 10);
//     println!("{}", b);

//     let C_ptr: *const i8 = &C as *const u8 as *const c_char;
//     c = CStr::from_ptr(C_ptr).to_string_lossy();
//     println!("{}", c);

//     let a_ptr = &a as *const usize;
//     println!("{:p} - {:p}", &a, a_ptr);

//     let a: i64 = 42;
//     let a_ptr = &a as *const i64;
//     let a_addr : usize = unsafe { std::mem::transmute(a_ptr) };

//     println!("a:{} {:p} ... 0x{:x}", a, a_ptr, a_addr + 7);
// }

// println!("a 指针的大小 {:?}", size_of::<&usize>());
// println!("c 胖指针的大小 {:?}", size_of::<&[u8]>());
// println!("c raw指针的大小 {:?}", size_of::<&[u8;11]>());
// println!("c 指针的所在地址 {:p}", &c);
// println!("c 指向 C 的地址 {:p}", c);

// println!("C 的大小 {:?}", size_of::<[u8;11]>());
// println!("C 所在地址 {:p}", &C);