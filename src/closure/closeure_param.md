
```rust
fn main() {
    let a = |x: i32| {1};
    meth(a)
}

fn meth<F>(f: F) where F: FnOnce(i32) -> i32 {
    println!("{}", f(12))
}
```