### 型变

可写类型，对应“不变”
只读类型，对应“协变”
函数指针，对应“逆变”

这是规定，违背就会出现问题。

假设 `let p: Animal = b: Cat; // 表示 Cat 是 Animal 的子类型`，
如果你需要一个喂动物的函数：`fn feed_animal(t: Animal)`，但是你提供了一个喂猫的函数：`fn feed_animal(t: Cat)`，这显然是不对的，猫吃猫粮但如果你拿猫粮喂马，那马就要爆炸了。但是反过来是可以的，你需要喂猫的函数，但是给你提供了一个喂动物的函数，喂动物的话就是要给它喝水吃东西，这操作套用到喂猫这件事上猫表示可以接受。所以，就形成了逆变：

```rust
let p: Animal = b: Cat // 后者是前者的子类型
let fn feed_animal(t: Cat) = fn feed_animal(t: Animal) // 后者是前者的子类型
```

套用到生命周期范围上

```rust
let p: 'a = b: 'static // 后者是前者的子类型
let fn feed_animal(t: &'static T) = fn feed_animal(t: &'a T) // 后者是前者的子类型
```
它的意思是能处理一个生命周期范围更大的引用，也应该能处理生命周期小的引用