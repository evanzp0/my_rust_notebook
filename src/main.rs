
fn check_size<T>(val: T) -> usize {
    core::mem::size_of::<T>()
}

struct A {
    i: u8,
    v: u32
}

enum B<'a> {
    Hello,
    World(&'a u16)
}

enum C {
    Hello,
    World(u16)
}

enum D {
    Hello, world
}

fn main() {
    // let a = "hello你好".as_bytes();
    // println!("{}",a.len());
    // println!("{:?}", check_size(["hello你好";69]));

    println!("{}", core::mem::size_of::<Option<Vec<u8>>>()); // 24 bytes
    println!("{}", core::mem::size_of::<Vec<u8>>()); // 24 bytes
    println!("{}", core::mem::size_of::<Option<Box<String>>>()); // 8 bytes
    println!("{}", core::mem::size_of::<Box<String>>()); // 8 bytes

    // println!("{}", core::mem::size_of::<C>());
    // println!("{}", core::mem::size_of::<D>());

    // println!("{}", core::mem::size_of::<&str>());

}

// #[derive(Debug)]
// struct A<'a>(&'a i32, i32 );

// fn foo<'a>(el : &'a mut A<'a>) {

// }

// fn main1() {
//     let a = 1;
//     let mut b = A(&a, 2);
//     foo(&mut b);
//     let c = &mut b;
//     println!("{:?}", b);
// }

