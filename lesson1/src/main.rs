mod first_layer {
    // 添加一个一层子模块,循环打印从’a’~’Z’ 之间的所有字符
    pub fn print_chars() {
        for c in 'a'..='z' {
            println!("{c}");
        }
        for c in 'A'..='Z' {
            println!("{c}");
        }
    }


    // 添加一个二层子模块,循环打印从’A’~’z’ 之间的所有字符
    pub mod second_layer {
        pub fn print_chars(){
            for c in 'A'..='z' {
                if !is_special_character(c) {
                    println!("{}", c);
                }
            }
        }
    
        fn is_special_character(c: char) -> bool {
            match c {
                '[' | '\\' | ']' | '^' | '_' | '`' => true,
                _ => false,
            }
        }
    }
}




fn main() {
    println!("作业一");
    first_layer::print_chars();
    println!("\n作业二");
    first_layer::second_layer::print_chars();
}