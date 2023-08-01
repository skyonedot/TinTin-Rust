// 定义一个简单的声明宏，名称是 `sum`
macro_rules! sum {
    // 定义宏的模式匹配，这里用到两个参数，用逗号隔开
    // $x:expr 表示匹配任何 Rust 表达式，并将其绑定到变量 $x
    // $y:expr 表示匹配另一个 Rust 表达式，并将其绑定到变量 $y
    ($x:expr, $y:expr) => {
        // 定义宏的展开部分，这里是将 $x 和 $y 相加并打印结果
        println!("{} + {} = {}", $x, $y, $x + $y);
    };
}

fn main() {
    let a = 10;
    let b = 20;

    // 使用我们定义的宏 `sum!` 来打印 a 和 b 的和
    sum!(a, b);
}


// 在这个例子中，我们定义了一个名为 sum 的宏，它接受两个参数并在宏展开时打印这两个参数的和。在 main 函数中，我们调用宏 sum! 来输出 10 和 20 的和。在 Rust 中，宏的调用以感叹号 ! 结尾。

// 编译过程如下：

// 1. 当编译器遇到宏调用 sum!(a, b) 时，它会寻找宏 sum 的定义。
// 2. 编译器在代码中找到了我们定义的宏 sum，然后将其展开。
// 3. 在展开时，编译器将 (a, b) 替换为 10, 20，并将展开后的代码插入到原来的位置。
// 4. 最终，编译器会生成展开后的代码，并继续编译整个程序。

// 展开后的代码将类似于：
// fn main() {
//     let a = 10;
//     let b = 20;

//     // 宏展开后的代码
//     println!("{} + {} = {}", a, b, a + b);
// }

// 然后，编译器会继续处理展开后的代码，并最终生成可执行程序。这就是 Rust 中声明宏的基本结构和编译过程。