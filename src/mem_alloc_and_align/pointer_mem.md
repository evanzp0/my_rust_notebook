### 指针和内存

```rust
use std::mem::size_of;

static B: [u8; 10] = [1, 2, 3, 4, 5, 6, 7, 8,  9, 10];
static C: [u8; 11] = [11, 12, 13, 14, 15, 16, 17, 18,  19, 20, 21];

fn main() {
    let a: usize = 42;
    let b: Box<[u8]> = Box::new(B);
    let c: &[u8; 11] = &C;

    println!("a 指针的大小 {:?}", size_of::<&usize>());
    println!("c 胖指针的大小 {:?}", size_of::<&[u8]>());
    println!("c raw指针的大小 {:?}", size_of::<&[u8;11]>());
    println!("c 指针的所在地址 {:p}", &c);
    println!("c 指向 C 的地址 {:p}", c);

    println!("C 的大小 {:?}", size_of::<[u8;11]>());
    println!("C 所在地址 {:p}", &C);
}
```
Box智能指针的内存分配，Box::leak() 后返回的堆上的对象是'static 的，无法被回收
```rust
    let s = "abcde".to_owned();
    println!("s addr: {:p}", &s);
    let s1 = s.as_ptr();
    println!("s raw addr: {:p}", s1);
    let a = Box::new(s);
    println!("a point to addr: {:p}", a);
    println!("a point to addr: {:p}", &*a);
    println!("a raw addr: {:p}", (*a).as_ptr());
    let b = Box::leak(a);
    println!("b point to addr: {:p}", b);
    println!("b raw addr: {:p}", b.as_ptr());
```