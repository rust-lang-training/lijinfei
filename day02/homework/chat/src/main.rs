use std::io;

fn main() {
    println!("请输入任意字符!");
    let mut str = String::new();
    io::stdin().read_line(&mut str).expect("请输入正确的输入");
    let str = str.trim().split("");
}
