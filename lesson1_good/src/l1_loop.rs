//2018之后的Mod写法
pub fn print() {
    let from: u32 = 'Z' as u32;
    for ch in (char::from_u32(from + 1).unwrap())..'a' {
        println!("{}", ch);
    }
    //下面也行
    // for ch in '['..='a' {
    //     println!("{},{}", ch, from);
    // }
}
