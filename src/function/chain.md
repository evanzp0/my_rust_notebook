### 链式调用

中间过程，如果对象`self`被消费了，则中间函数返回时`drop`该`self`；
最后一个函数的返回值在分号后`match`等`block`结束时被`drop`

```rust
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
    fn get_c(&self) -> C {
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
    a.get_b().get_c().end_c()
    // match a.get_b().get_c().end_c() {
    //     true => println!("end C"),
    //     false => println!("end C?"),
    // }
    println!("main end");
}
```
结果显示:
```
get_b
drop A
get_c
drop B
end C
drop C
main end
```