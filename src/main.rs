use std::{mem::size_of, borrow::Cow};
use std::ffi::CStr;
use std::os::raw::c_char;

static B: [u8; 10] = [99, 97, 114, 114, 121, 116, 111, 119,  101, 108];
static C: [u8; 11] = [116, 104, 97, 110, 107, 115, 102, 105,  115, 104, 0];

fn main() {
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
}

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