use std::io;

fn main() {
    println!("请输入任意字符!");
    let mut str = String::new();
    io::stdin().read_line(&mut str).expect("请输入正确的输入");
    let str = str.trim().to_lowercase();
    let mut result = [0u32; 26];
    for s in str.chars() {
        let char_code = s as u8;
        // println!("{s} => {char_code}");
        if char_code >= 97 && char_code <= 122 {
            result[(char_code - 97) as usize] = result[(char_code - 97) as usize] + 1;
        }
    }
    // println!("{:?}", result);
    for code in 0..26 {
        let char_code = code + 97 as u8;
        println!("{} => 出现了 {} 次", char_code as char, result[code as usize]);
    }
}
