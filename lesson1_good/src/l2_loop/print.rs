pub fn print() {
    let from: u32 = 'A' as u32;
    for ch in (char::from_u32(from + 1).unwrap())..'z' {
        println!("{ch}");
    }
}