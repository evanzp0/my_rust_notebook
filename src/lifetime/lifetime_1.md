# 关于 Rust 生命周期范围（1 / 3）

### 代码示例
```rust{.line-numbers}
fn the_longest<'a: 'b, 'b>(s1: &'a str, s2:&'b str) -> &'b str {
    if s1.len() > s2.len() {s1} else {s2}
}
fn main() {
     let s1 = String::from("Rust");
     let s1_r = &s1;
     {
        let mut s2 = String::from("C");
        let res = the_longest(s1_r, &s2);
        let s3 = &mut s2;
        println!("{}", res);
    }
}
```

**对 main 代码中的生命周期进行标注**
* 分析 main 中的生命周期标注，可以不用考虑 `the_longest()` 中的生命周期标注

```rust{.line-numbers}
'a {
    let s1 = String::from("Rust");
    'b {
        let s1_r = &'b s1;
        'c {
            let s2 = String::from("C");
            'd {
                'e {
                    'f {
                        // &'e s2, 可以看成在实参传入函数时生成了一个匿名对象，并指明在 'e 范围内该匿名对象有权安全引用 s2
                        // 根据函数输入输出的生命周期标注规则（输出生命周期<=输入生命周期中最小的那个），'e >= 'f
                        let res: &'f str = the_longest(s1_r, &'e s2);  
                        'g {
                            // 借用检查器发现 'e 内（匿名对象对s2的有不可变借用权的生命周期范围内）出现了对 s2 的可变借用，因此判定违背了借用规则
                            let s3 = &'g mut s2; // error!
                            'h {
                                // println!() 宏会调用 &res，此时会创建一个匿名的生命周期范围 'h
                                // 因为之前已经判定 'e >= 'f ，使用了 &res 对应生命周期 'f，所以 'e 的生命周期也被延展到此 print() 结束后才结束
                                println!("{}", &'h res);
                            }
                        }
                    }
                }
            }
        }
    }
}
```

**生命周期标注说明：**
- `'a` : 是 `let s1` 开出的生命周期范围
- `'b` : 是 `let s1_r` 开出的生命周期范围
- `'c` : 是 `let s2` 开出的生命周期范围
- `'d` : 是 代码第7行的 `{` 开出的生命周期范围，依据《Rust 编程之道》中的 128 页中关于"可以创建新词法作用域的场景"的描述
- `'e` ：是 `the_longest(s1_r, &'e s2)` 中，`&s2` 作为参数传入时开出的生命周期范围，依据《Rust 死灵书》在 "3.3生命周期"中关于“匿名生命周期”的描述
- `'f` : 是 `let res` 开出的生命周期范围, `'f 在 println!("{}", res)` 这段代码之后才结束，是因为 `'f` 对应的 `res` 变量在 `println!()` 中被使用, 引用的生命周期范围结束的地方是它最后一次被使用的地方
- `'g` : 是 `let s3` 开出的生命周期范围 
- `'h` : `println!()` 宏会调用 `&res`，此时会创建一个匿名的生命周期范围 `'h`

### 什么是生命周期范围呢？
就是在该范围内对象可以被有效使用，对于自有对象(`owned object`)而言它没被析构，对于借用对象而言它没有被悬垂