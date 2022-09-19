# 关于 Rust 生命周期范围（3 / 3）
### 代码示例 1
```rust{.line-numbers}
fn main() {
    let mut s1 = String::from("Rust");
    let s2 = &mut s1;
    let s3 = &s2;
    println!("{}", s3);
    println!("{}", s2);
    let s4 = &mut s1;
    println!("{}", s2);
}
```
**系统提示：**

```console
error[E0499]: cannot borrow `s1` as mutable more than once at a time
 --> src/lifetime_3.rs:8:18
  |
4 |         let s2 = &mut s1;
  |                  ------- first mutable borrow occurs here
...
8 |         let s4 = &mut s1;
  |                  ^^^^^^^ second mutable borrow occurs here
9 |         println!("{}", s2);
  |                        -- first borrow later used here
```

**对 main 代码中的生命周期进行标注**

```rust{.line-numbers}
'a {
    let mut s1 = String::from("Rust");
    'b {
        let s2 = &'b mut s1;
        'c {
            /// s3 是对 s2 的借用，而不是对 s1 的借用，所以并没有破坏借用规则
            let s3 = &'c s2;
            'c1 { 
                /// println!() 宏会调用 &s3，此时会创建一个匿名的生命周期范围 'c1
                println!("{}", &'c1 s3);
            }
            
        }
        println!("{}", s2);
        'd {
            /// s2 有权在 'b 的范围内对 s1 进行可变引用，
            /// 此时 s4 又对 s1 进行借用，破坏了借用规则，导致编译器报错
            let s4 = &'d mut s1; // error
        }
        'e {
            /// println!() 宏会调用 &s2，此时会创建一个匿名的生命周期范围 'e
            println!("{}", &'e s2);
        }
    }
}
```

### 代码示例 2

```rust{.line-numbers}
fn main() {
    let mut s1 = String::from("Rust");
    let mut s2 = &mut s1;
    let s3 = &mut s2;
    let s4 = &s2; // error
    let s5 = &s3;
}
```

**系统提示：**

```console
  --> src/lifetime_3.rs:16:14
   |
15 |     let s3 = &mut s2;
   |              ------- mutable borrow occurs here
16 |     let s4 = &s2;
   |              ^^^ immutable borrow occurs here
17 |     let s5 = &s3;
   |              --- mutable borrow later used here
```

**对上面 main 代码中的生命周期标注**

```rust{.line-numbers}
'a {
    let mut s1 = String::from("Rust");
    'b {
        let mut s2 = &'b mut s1;
        'c {
            let s3 = &'c mut s2;
            'd {
                let s4 = &'d s2; } //'b end 
            } // 'd end
            'e {
                let s5 = &'e s3;
            } // 'e end
        } // 'c end
} // 'a end
```

**说明**

- 上面的生命周期范围 `'b` 和 `'c` 出现了互相交错的现象
- 在 `'c` 的生命周期范围里，`s3` 对应 `s2` 有权使用可变借用，所以当 `s4` 在该范围中对 `s2` 再次进行不可变借用时，借用规则被破坏


### 代码示例 3 (重引用 reborrow)

```rust{.line-numbers}
fn main() {
    let mut s1 = String::from("Rust");
    let s2 = &mut s1;
    // let s3 = &mut s1; // 这样违反借用规则会报错
    let s3 = &mut* s2; // 对 s2 重引用，注意不是 &mut (*s2) , 因为 (*s2) 会转移 s2 所有权  
    let s5 = &s2; // 报错，如果这行和下面的 let 4 那一行换下位置就不报错了
    let s4 = &s3;
}
```

**系统提示：**

```console
error[E0502]: cannot borrow `s2` as immutable because it is also borrowed as mutable
 --> src/lifetime_3.rs:5:14
  |
4 |     let s3 = &mut* s2;
  |              -------- mutable borrow occurs here
5 |     let s5 = &s2;
  |              ^^^ immutable borrow occurs here
6 |     let s4 = &s3;
  |              --- mutable borrow later used here
```

**对上面 main 代码中的生命周期标注**

```rust{.line-numbers}
'a {
    let mut s1 = String::from("Rust");
    'b {
        let s2 = &'b mut s1;
        'c {
            let s3 = &'c mut* s2;
            'd {
                let s5 = &'d s2; } // 'b end
            } // 'd end
            'e {
                let s4 = &'e s3;
            } // 'e end
        }
} // 'a end
```

**说明**

- 上面的生命周期范围 `'b` 和 `'c` 出现了互相交错的现象
- 在 'c 的生命周期范围里，`s3` 对 `s2` 进行了 reborrow，所以编译器认为 `s3` 在该范围内对应 `s2` 有权使用可变借用，因此当 `s5` 在该范围 `'c` 中对 `s2` 再次进行不可变借用时，借用规则被破坏


### 代码示例 4 (这是我实际开发中遇到的问题)

```rust
impl<'a> TaskQuery<'a> {

    fn build_sql(&'a self, params: &'a mut Vec<&'a (dyn ToSql + Sync)>) -> String { 
        ...
    }
    
    pub async fn fetch_count(&self) -> Result<i64> {
        let mut params: Vec<&(dyn ToSql + Sync)> = Vec::new();
        let sql= self.build_sql(&mut params);
        let rst = self.base_dao.fetch_i64(&sql, &params).await?;
    
        Ok(rst)
    }
    
}

```

**系统提示:**

```console
error[E0502]: cannot borrow `params` as immutable because it is also borrowed as mutable
   --> src/manager/engine/query/task_query.rs:109:49
    |
108 |         let sql= self.build_sql(&mut params);
    |                                 ----------- mutable borrow occurs here
109 |         let rst = self.base_dao.fetch_i64(&sql, &params).await?;
    |                                                 ^^^^^^^
    |                                                 |
    |                                                 immutable borrow occurs here
    |                                                 mutable borrow later used here
```

**标注下生命周期**

```
'a == 'self? {
    let mut params: Vec<&(dyn ToSql + Sync)> = Vec::new();
    'b {
        // sql 这个变量不用看，主要是 build_sql() 中声明了 param 的生命周期 ‘b 和 self 一样长，虽说这样本来就是不合理的
        let sql= self.build_sql(&'b mut params); 
        'c {
            // &'b mut params 的匿名对象在 'b 生命周期范围内对 params 有权进行可变借用，所以，再进行生成 &param 对应的匿名对象进行不可变借用时就出错了
            let rst = self.base_dao.fetch_i64(&'_ sql, &'b params).await?;
        }
        
    }
}
```

**改成下面这样, 去掉`build_sql()`中`params`的生命周期标注`'a`，就能通过了**

```rust
impl<'a> TaskQuery<'a> {

    fn build_sql(&'a self, params: &mut Vec<&'a (dyn ToSql + Sync)>) -> String { 
    
    }
    
    pub async fn fetch_count(&self) -> Result<i64> {
        let mut params: Vec<&(dyn ToSql + Sync)> = Vec::new();
        let sql= self.build_sql(&mut params);
        let rst = self.base_dao.fetch_i64(&sql, &params).await?;
    
        Ok(rst)
    }
    
}
```

**标注下生命周期**

```
'a == 'self? {
    let mut params: Vec<&(dyn ToSql + Sync)> = Vec::new();
    'b {
        let sql= self.build_sql(&'b mut params); 
    }
    'c {
        let rst = self.base_dao.fetch_i64(&'_ sql, &'c params).await?;
    }
}
```

