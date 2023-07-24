mod q1a1;
mod q1a2;
mod q2;

fn main() {
    // Question1 Answer1
    // 使用枚举包裹三个不同的类型，并放入一个Vec中，对Vec进行遍历，调用三种不同类型的各自的方法。
    println!("Question1 Method1:");
    let vec: Vec<q1a1::EnumType> = vec![
        q1a1::EnumType::T1(q1a1::Type1),
        q1a1::EnumType::T2(q1a1::Type2),
        q1a1::EnumType::T3(q1a1::Type3),
    ];
    for item in vec {
        match item {
            q1a1::EnumType::T1(t1) => t1.method1(),
            q1a1::EnumType::T2(t2) => t2.method2(),
            q1a1::EnumType::T3(t3) => t3.method3(),
        }
    }

    // Question1 Answer2
    // 定义三个不同的类型，使用Trait Object，将其放入一个Vec中，对Vec进行遍历，调用三种不同类型的各自的方法。
    println!("\nQuestion1 Method2:");
    let vec: Vec<Box<dyn q1a2::MyTrait>> = vec![
        Box::new(q1a2::Type1),
        Box::new(q1a2::Type2),
        Box::new(q1a2::Type3),
    ];
    for item in vec {
        item.my_method();
    };

    //Question2 Answer
    println!("\nQuestion2");
    let num1 = q2::MyNumber(5);
    let num2 = q2::MyNumber(10);
    q2::perform_addition(&num1, &num1, &num2);
}
