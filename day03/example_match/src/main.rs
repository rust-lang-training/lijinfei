use std::io;
fn fibo(n: u32) -> u32{
    // match n {
    //     0 => 1,
    //     1 => 1,
    //     _ => fibo(n-1) + fibo(n-2)
    // }
    match Option::Some(n) {
        Some(_x @ 0..=1) => 1,
        _ => fibo(n-1) + fibo(n-2)
    }
}
fn main() {
    println!("Hello, world!");
    println!("输入第几项");
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("请输入正确的数字");
    let n: u32 = n.trim().parse().expect("请输入正确的数字");
    println!("第n项的输出为：{}", fibo(n));
}
