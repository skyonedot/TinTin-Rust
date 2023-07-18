作业

创建一个Rust工程，

（1）添加一个一层子模块，循环打印从’a’~’Z’ 之间的所有字符

（2）添加一个二层子模块，循环打印从’A’~’z’ 之间的所有字符

（3）使用Cargo编译运行此工程

（需要自行发现其中的细节，一个考点是：ascii码字符的顺序）


两种Print均可
fn main() {
    let start: u8 = b'A';
    let total_rows = 15;

    for row in 0..total_rows {
        for col in 0..4 {
            let index = (row + col * total_rows) as u8;
            let ascii_value = start + index;
            print!("{:<4}{:<5}｜", ascii_value, char::from_u32(ascii_value as u32).unwrap());
            // print!("{:<4}{:<5}｜", ascii_value, ascii_value as char);
        }
        println!();
    }
}
