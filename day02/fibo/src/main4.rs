use std::io;
fn fibo(n: u32) -> u32{
    if n == 0 {
        return 0;
    }
    if n == 1 || n == 2 {
        return 1;
    }
    let mut num1 = 1u32;
    let mut num2 = 1u32;
    for _v in 3..n {
        let m = num1 + num2;
        num1 = num2;
        num2 = m;
    }
    num1+num2
}
fn main() {
    println!("Hello, world!");
    println!("输入第几项");
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("请输入正确的数字");
    let n: u32 = n.trim().parse().expect("请输入正确的数字");
    println!("第n项的输出为：{}", fibo(n));
}
