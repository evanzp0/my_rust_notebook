### Enum 的内存分配和对齐

- `enum` 如果没有关联值时：
    - 如果只有一个 `variant`，则为 `0` 字节长度
    - 如果大于一个 `variant`，则每一个 `variant` 用一个`bit`来对应，所以一个字节可以存放 `2^8 = 256` 个`variant`
```rust
/// 0 byte
enum A {
    s1
} 

/// 1 byte
enum A {
    s1, s2
}

/// 2 bytes
enum B {
    s1, s2, ..., s257
}
```
- `enum` 如果有关联值时：
    - 如果一个 `variant`没关联值，另一个关联值为引用`(&)` ，则为 `8` 个字节长度（引用的长度, `64` 位）, 没关联值的 `variant`可以用`null`表示
    - 如果一个 `variant`没关联值，另一个关联值非引用`(&)`，则按 `variant`数据类型的对齐规则，计算大小
```rust
/// 4 bytes
enum C {
    Hello, // 1 byte 
    // padding, 1 byte
    World(u16) // 2 bytes
}

enum D<'a> {
    Hello, // 0 byte
    World(&'a u16) // 8 bytes
}
```
在 Rust 中，有许多类型会包含不可为空的指针，如Box<T>、Vec<T>、String、&T和&mut T。同样地，我们可以想象嵌套的枚举将它们的标记集中到一个单一的字段中，因为根据定义，它们的有效值范围有限。

```rust
    println!("{}", core::mem::size_of::<Option<Vec<u8>>>()); // 24 bytes
    println!("{}", core::mem::size_of::<Vec<u8>>()); // 24 bytes
    println!("{}", core::mem::size_of::<Option<Box<String>>>()); // 8 bytes
    println!("{}", core::mem::size_of::<Box<String>>()); // 8 bytes
```

- `enum` 都有关联值时：
```rust
enum Foo {
    A(u32),
    B(u64),
    C(u8),
}
```
可能会被布局成：
```rust
struct FooRepr {
    data: u64, // 根据 tag 的不同，这一项可以为 u64，u32，或者 u8
    tag: u8,   // 0 = A，1 = B， 2 = C
}
```