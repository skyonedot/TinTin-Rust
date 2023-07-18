// use std::io::{self, BufRead};

// fn main() {
//     println!("请输入一个数字：");

//     let stdin = io::stdin();
//     let input = stdin.lock().lines().next().unwrap().unwrap();
//     let number: i32 = input.parse().unwrap();

//     let output = match number {
//         0 => "你输入了0",
//         1 => "你输入了1",
//         2 => "你输入了2",
//         _ => "你输入了其他数字",
//     };

//     println!("{}", output);
// }
use std::collections::HashMap;

// 学生结构体
struct Student {
    id: u32,
    name: String,
    class_id: u32,
}

// 班级结构体
#[allow(dead_code)]
struct Class {
    id: u32,
    name: String,
}

// 课程结构体
#[allow(dead_code)]
struct Course {
    id: u32,
    name: String,
}

// 学生管理系统结构体
struct StudentManagementSystem {
    students: HashMap<u32, Student>,
    classes: HashMap<u32, Class>,
    courses: HashMap<u32, Course>,
}

impl StudentManagementSystem {
    // 添加学生
    fn add_student(&mut self, id: u32, name: String, class_id: u32) {
        
        let student = Student {
            id,
            name: name.clone(),
            class_id,
        };
        self.students.insert(id, student);
        println!("学生 {} 添加成功！", name);
        
    }


    // 获取学生
    fn get_student(&self, id: u32) -> Option<&Student> {
        
        self.students.get(&id)
        
    }

    // 更新学生
    fn update_student(&mut self, id: u32, name: String, class_id: u32) {
        
        if let Some(student) = self.students.get_mut(&id) {
            student.name = name;
            student.class_id = class_id;
            println!("学生 {} 更新成功！", student.name);
        } else {
            println!("找不到ID为 {} 的学生！", id);
        }
        
    }

    // 删除学生
    fn delete_student(&mut self, id: u32) {
        
        if let Some(student) = self.students.remove(&id) {
            println!("========任务四：学生 {} 删除成功！========", student.name);
        } else {
            println!("找不到ID为 {} 的学生！", id);
        }
        
    }
}

fn main() {
    let mut system = StudentManagementSystem {
        students: HashMap::new(),
        classes: HashMap::new(),
        courses: HashMap::new(),
    };

    // 添加班级
    let class_id = 1;
    let class_name = "Class A".to_string();
    let class = Class {
        id: class_id,
        name: class_name.clone(),
    };
    system.classes.insert(class_id, class);

    // 添加课程
    let course_id = 1;
    let course_name = "Math".to_string();
    let course = Course {
        id: course_id,
        name: course_name.clone(),
    };
    system.courses.insert(course_id, course);

    // 添加学生
    print!("========任务一：添加学生开始========\n");
    let student_id = 1;
    let student_name = "John".to_string();
    system.add_student(student_id, student_name.clone(), class_id);
    print!("========任务一：添加学生完成========\n");

    // 获取学生信息
    print!("========任务二：查询学生开始========\n");
    if let Some(student) = system.get_student(student_id) {
        println!("学生ID：{}", student.id);
        println!("学生姓名：{}", student.name);
        if let Some(class) = system.classes.get(&student.class_id) {
            println!("学生班级：{}", class.name);
        }
    }
    print!("========任务二：查询学生完成========\n");

    // 更新学生信息
    print!("========任务三：更新学生开始========\n");
    let new_name = "Jane".to_string();
    let new_class_id = 2;
    system.update_student(student_id, new_name.clone(), new_class_id);
    print!("========任务三：更新学生完成========\n");

    // 获取更新后的学生信息
    print!("========任务四：删除学生开始========\n");
    if let Some(student) = system.get_student(student_id) {
        println!("学生ID：{}", student.id);
        println!("学生姓名：{}", student.name);
        if let Some(class) = system.classes.get(&student.class_id) {
            println!("学生班级：{}", class.name);
        }
    }
    

    // 删除学生
    system.delete_student(student_id);
}
