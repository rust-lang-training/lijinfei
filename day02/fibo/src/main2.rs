use std::io;
fn fibo(n: u32) -> u32{
    if n==0 || n == 1 || n == 2 {
        return 1;
    }
    fibo(n - 1) + fibo(n - 2)
}
fn main() {
    println!("Hello, world!");
    println!("输入第几项");
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("请输入正确的数字");
    let n: u32 = n.trim().parse().expect("请输入正确的数字");
    println!("第n项的输出为：{}", fibo(n));
}
