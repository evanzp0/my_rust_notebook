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