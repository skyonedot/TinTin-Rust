
use std::ops::Add;

#[derive(Clone)] // Derive the Clone trait to enable cloning MyNumber instances
pub struct MyNumber(pub i32);

impl Add for MyNumber {
    type Output = MyNumber;

    fn add(self, other: MyNumber) -> MyNumber {
        MyNumber(self.0 + other.0)
    }
}

pub trait MyTrait {
    fn display(&self);
}

impl MyTrait for MyNumber {
    fn display(&self) {
        println!("MyNumber: {}", self.0);
    }
}

pub fn perform_addition(obj: &dyn MyTrait, num1: &MyNumber, num2: &MyNumber) {
    let result = num1.clone().add(num2.clone()); // Clone the MyNumber instances before calling add()
    obj.display();
    println!("Result of addition: {}", result.0);
}
