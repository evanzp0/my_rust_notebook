# 关于 Rust 生命周期范围（2 / 3）
### 代码示例 1
```rust{.line-numbers}
fn main() {
    let mut s1 = String::from("Rust");
    let s2 = &mut s1;
    let s3 = &s1;
}
```

**对 main 代码中的生命周期进行标注**

```rust{.line-numbers}
'a {
    let mut s1 = String::from("Rust");
    'b {
        /// s2 是对应范围 'b 的生命周期，因为 s2 之后没被使用，所以它对应的生命周期范围 'b 被编译器认为提前结束，
        /// 虽然技术上来说变量 s2 在 main() 函数结束的时候才被 drop
        let s2 = &'b mut s1;
    }
    'c {
        /// 由于 'b 生命周期提前结束，所以 s3 能够借用到 s1 的只读引用 
        let s3 = &'c s1;
    }
}
```

### 代码示例 2
```rust{.line-numbers}
fn main() {
    let mut s1 = String::from("Rust");
    let s2 = &mut s1;
    let s3 = &s1;
    println!("{}", s2);
}
```

**系统提示：**

```
error[E0502]: cannot borrow `s1` as immutable because it is also borrowed as mutable
 --> src/lifetime_2.rs:4:14
  |
3 |     let s2 = &mut s1;
  |              ------- mutable borrow occurs here
4 |     let s3 = &s1;
  |              ^^^ immutable borrow occurs here
5 |     println!("{}", s2);
  |                    -- mutable borrow later used here
  ```

**对上面 main 代码中的生命周期标注**

```rust{.line-numbers}
'a {
    let mut s1 = String::from("Rust");
    'b {
        /// s2 是对应范围 'b 的生命周期，因为 s2 在第 5 行中被 println!()使用，所以它对应的生命周期范围 'b 被编译器认为到了 println!() 结束后的位置才结束
        let s2 = &'b mut s1;
        'c {
            /// 由于 'b 生命周期尚未结束，s2 有权在 'b 的范围内借用 s1 的可变引用，所以 s3 借用 s1 的只读引用时，破坏了借用规则，编译器报错
            let s3 = &'c s1; // error!
        }
        'd {
            /// println!() 宏会调用 &s2，此时会创建一个匿名的生命周期范围 'd
            println!("{}", &'d s2);
        }
    }
}
```

