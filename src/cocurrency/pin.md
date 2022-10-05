
Pin这个智能指针的作用，其实就是如果被包装的指针指向的对象是 !Upin的，那么通过Pin后的智能指针无法获得 该对象的可变引用。如果被包装的指针指向的对象是 Upin的，那么通过Pin后的智能指针可以获得 该对象的可变引用。实际上那个对象本身如果是自引用结构，直接转移这个对象的所有权，还是会出现自引用指针指向失效内存的问题对吗？

pin你只需要搞明白，pin的new函数，box的pin函数为什么是safe的; pin的new unchecked为什么是unsafe的就行了
get_mut/as_mut/deref_mut/new 几个方法实际上都是针对 Target: Unpin 实现的，new() 里面 unsafe包了下 new unchecked

```rust
use std::{marker::PhantomPinned, pin::Pin, ops::Deref};

fn main() {

    struct Ms {
        a: Box<i32>,
        b: * const Box<i32>,
        _marker: PhantomPinned,
    }

    let mut m1 = Ms {
        a: Box::new(1),
        b: std::ptr::null(),
        _marker: PhantomPinned,
    };

    let mut pin_m1;
    unsafe {
        pin_m1 = Pin::new_unchecked(&m1);
    }

    let mut m2 = Ms {
        a: Box::new(2),
        b: std::ptr::null(),
        _marker: PhantomPinned,
    };

    m2.b = &m2.a as *const Box<i32>;
    
    let mut pin_m2;
    unsafe {
        pin_m2 = Pin::new_unchecked(&m2);
    }
    // pin_m2.a =  Box::new(3); // error

    struct Mt {
        a: Box<i32>,
    }

    let mut m3 = Mt {
        a: Box::new(2),
    };
    
    let mut pin_m3 = Pin::new(&mut m3);
    pin_m3.a =  Box::new(3);


    impl Deref for Mt {
        type Target = i32;

        fn deref(&self) -> &Self::Target {
            todo!()
        }
    }


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
        println!("{:p}, {:p}", &m1.a, m1.a);
        println!("{:p}, {:p}, {:p}", &m1.b, m1.b, *m1.b);
    }

    let mut m2 = Ms {
        a: Box::new(2),
        b: std::ptr::null(),
    };

    m2.b = &m2.a as *const Box<i32>;
    unsafe {
        println!("{}", *m2.b);
        println!("{:p}, {:p}", &m2.a, m2.a);
        println!("{:p}, {:p}, {:p}", &m2.b, m2.b, *m2.b);
    }

    m2 = m1;
    m2.a = Box::new(3);
    unsafe {
        println!("{}", *m2.b);
        println!("{:p}, {:p}", &m2.a, m2.a);
        println!("{:p}, {:p}, {:p}", &m2.b, m2.b, *m2.b);
    }

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
```