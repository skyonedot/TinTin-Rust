// 定义三个不同类型
pub struct Type1;
pub struct Type2;
pub struct Type3;

// 为每个类型实现各自的方法
impl Type1 {
    pub fn method1(&self) {
        println!("Q1A1 Type1 In Vec.");
    }
}

impl Type2 {
    pub fn method2(&self) {
        println!("Q1A1 Type2 In Vec.");
    }
}

impl Type3 {
    pub fn method3(&self) {
        println!("Q1A1 Type3 In Vec.");
    }
}

// 枚举包裹三个不同的类型
pub enum EnumType {
    T1(Type1),
    T2(Type2),
    T3(Type3),
}