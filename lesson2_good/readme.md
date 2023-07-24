# Week02 <!-- omit in toc -->

## Table of content <!-- omit in toc -->
- [Homework](#homework)
  - [Description](#description)
  - [Ownership](#ownership)
  - [Reference](#reference)
  - [Resource](#resource)


## Homework

### Description

模仿老师的思路，自己对所有权、不可变引用、可变引用这三者的规则或特性做一个集中的总结，写一个笔记列表。

### Ownership

从一个简单的 String 例子出发

```rust
fn main() {
    let a = String::from("Hello");
    println!("{a}");
}
```

但是想组出一个 `Hello World` 的时候会出现编译错误(borrow of moved value: `a`)

```rust
fn main() {
    let a = String::from("Hello");
    let b = a + " World"; // Error
    // let b = a.clone() + " World"; // Correct
    println!("{a}");
    println!("{b}");
}
```

即使将例子简化也会出现一样的错误代码

```rust
fn main() {
    let a = String::from("Hello");
    let b = a; // Error
    // let b = a.clone(); // Correct
    println!("{a}");
    println!("{b}");
}
```

这时，会发现换个类型的值则符合预期的正常执行。

```rust
fn main() {
    let a = 1;
    let b = a;
    println!("{a}");
    println!("{b}");
}
```

为什么呢？  
在深入研读 [The Rust Programming Language][The Book]，了解下列几点后
1. Rust 借鉴并改进 C++ 的 RAII([Resource Acquisition Is Initialization][RAII])
   1. 不需要像 C 一样完全手动管理内存（由程序员呼叫 free）。
   2. 值会在离开 scope 的时候释放其内存。
2. Memory(内存) 中 Heap(堆) 和 Stack(栈) 相关知识
   1. 固定大小(or 编译期间可以确定大小)类型的值，编译器一般默认将值放在 Stack，如 i32。
   2. 动态大小(or 编译期间不可确定大小)类型的值，编译器一般默认将值放在 Heap，由一个 Stack 上的变数指向其地址，如 Vec。
3. Rust 中许多类型默认实现 `Copy` trait，是一种 bitwise 的复制方式(i.e. 浅拷贝)，如 i32, bool, char，但 String 类型不属于这个范畴。
   1. String 内部是 Vec\<u8\>，浅拷贝会产生 2 变数指向同地址，需要 clone (i.e. 深拷贝) 才能连同 Heap 的值复制。

这时我们会发现，Stack 上的值都由编译器管理好了，运行过程中会随着函数呼叫一同增减，困难的在 Heap 上，也就是动态大小的值。  
当然，可以默认使用深拷贝，但这非常损耗效能；再来是默认浅拷贝的话，依照 RAII，离开 scope 时又该哪个变数负责回收那同一个内存呢？ 毕竟 double-free 也不是我们想要的...  

如果我们从 RAII 出发，默认浅拷贝，加上唯一性的概念，有机会得出和 Rust 所有权规则相同的方式。  
这里，在不使用 clone 的方式，我们给出了一个可运行的例子，并标注相对应所有权的概念。

```rust
fn main() {
    let a = String::from("Hello"); // a has the ownership of the String value.
    let b = a; // String value of a is moved to b here. Now b has the ownership and a becomes invalid.
    // println!("{a}"); // Should not use an invalid variable.
    println!("{b}");
} // String value of b is dropped here.
```

最终，我们可以得到如下数条所有权规则，规则 1 & 2 保证了值被唯一一个变数所拥有；规则 3 保证拥有者离开 scope 时释放值的内存，完成内存管理。

所有权规则
1. Each value in Rust has an owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.

### Reference

如果 Rust 只有所有权，想必不是很方便，特别是作为函数的参数时...毕竟很多时候我们会接着函数呼叫后继续使用该变数，不应该因为所有权机制而被强迫将该值作为返回值的一部分。
因此，借用(Borrwing) 机制，即创建引用(Reference) 因应而生。引用的概念也是借鉴 C++ 而来，但 Rust 默认引用和值都是 immutable。  
虽然上课时老师使用的词是「引用」，本文尽量按照编译器的讯息解释，所以「借用」和「引用」两个词会穿插使用，但不影响理解。

从一个小例子开始

```rust
fn main() {
    let a;
    let b = 10;
    a = &b;
    println!("{a}");
    println!("{b}");
}
```

如果我们修改一下，则会出现编译错误。  
因为 Rust 编译器会确认 reference 是否有效，编译时发现引用在值被回收后被使用会报错，这也保证了引用是有效的(valid)。

```rust
fn main() {
    let a;
    {
        let b = 10;
        a = &b; // Error: `b` does not live long enough
        println!("{b}");
    } // b dropped here while still borrowed
    println!("{a}"); // borrow later used here
}
```

如果我们将最开始的例子，换成 mutable，则同样出现编译错误(cannot borrow `a` as immutable because it is also borrowed as mutable)。

```rust
fn main() {
    let mut a = 10;
    let b = &mut a; // mutable borrow occurs here
    *b = 20;
    println!("{a}"); // immutable borrow occurs here
    println!("{b}"); // mutable borrow later used here
}
```
上述错误的意思是同时只能有可变或不可变的借用，我们试着只留下其中一个使用，则编译通过。

```rust
fn main() {
    let mut a = 10;
    let b = &mut a;
    *b = 20;
    // println!("{a}");
    println!("{b}");
}
```

```rust
fn main() {
    let mut a = 10;
    let b = &mut a;
    *b = 20;
    println!("{a}");
    // println!("{b}");
}
```

但其实如果我们观察 println!，发现其是透过不可变借用的方式取得变数，会觉得奇怪，不是不可以同时有两种借用吗？这个借用的范围是如何判断的呢？  
我们试着同时保留两者，只微调一下顺序，发现编译通过。

```rust
fn main() {
    let mut a = 10;
    let b = &mut a; // mutable borrow here
    *b = 20; // mutable borrow used here
    println!("{b}"); // immutable borrow occurs here
    println!("{a}"); // immutable borrow occurs here
}
```

可以推断，借用的发生只会持续到最后一次使用，所以只要两种借用不同时重叠即可。  
接下来，同时观察多个可变或多个不可变借用的例子，前者编译失败，后者则编译成功。

```rust
fn main() {
    let mut a = 10;
    let b = &mut a; // first mutable borrow occurs here
    let c = &mut a; // second mutable borrow occurs here
    *b = 20; // first borrow later used here
}
```

```rust
fn main() {
    let a = 10;
    let b = &a;
    let c = &a;
    println!("{a}");
    println!("{b}");
    println!("{c}");
}
```

整理过后可以得知
1. 同时可存在多个不可变借用（引用）。
2. 同时只能有一个可变借用（引用）。
3. 同时只能存在一种借用。

如果熟悉 Data Race，可以发现一件有趣的事情，这里使用 [The Rust Programming Language][The Book] 提到的 Data Race 的定义
- Two or more pointers access the same data at the same time.
- At least one of the pointers is being used to write to the data.
- There’s no mechanism being used to synchronize access to the data.

其实 Rust 对引用的限制，恰恰是在编译时防止 Data Race 的可能性。按照目前个人的理解，这可能是日后 fearless concurrency 的根基。

结束前，实验一下可变引用和不可变引用在 assignment 的时候会发生什么事。  
可以看到 Rust 为不可变引用实现了 Copy trait，使其便于使用，也符合规则。

```rust
fn main() {
    let a = 10;
    let b = &a;
    let c = b;
    println!("{b}");
    println!("{c}");
}
```

由于可变引用具有唯一性，不实现 Copy trait，移动后不可再被使用，也因此保证了唯一性。

```rust
fn main() {
    let mut a = 10;
    let b = &mut a; // move occurs because `b` has type `&mut i32`, which does not implement the `Copy` trait
    let c = b; // value moved here
    println!("{b}"); // value borrowed here after move
    println!("{c}");
}
```

最后，以引用规则做收尾。

引用规则
1. At any given time, you can have either one mutable reference or any number of immutable references.
2. References must always be valid.

### Resource

- [The Rust Programming Language][The Book]
- [Resource Acquisition Is Initialization][RAII]

<!-- Links -->
[The Book]: https://doc.rust-lang.org/book/title-page.html
[RAII]: https://en.cppreference.com/w/cpp/language/raii