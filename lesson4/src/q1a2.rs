// 定义三个不同类型，并为它们定义Trait
pub trait MyTrait {
    fn my_method(&self);
}

pub struct Type1;
pub struct Type2;
pub struct Type3;

impl MyTrait for Type1 {
    fn my_method(&self) {
        println!("Q1A2 Trait Object Type1 In Vec.");
    }
}

impl MyTrait for Type2 {
    fn my_method(&self) {
        println!("Q1A2 Trait Object Type2 In Vec.");
    }
}

impl MyTrait for Type3 {
    fn my_method(&self) {
        println!("Q1A2 Trait Object Type3 In Vec.");
    }
}
