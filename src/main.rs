#[derive(Debug)]
struct A<'a>(&'a i32, i32 );

fn foo<'a>(el : &'a mut A<'a>) {

}

fn main() {
    let a = 1;
    let mut b = A(&a, 2);
    foo(&mut b);
    let c = &mut b;
    println!("{:?}", b);
}