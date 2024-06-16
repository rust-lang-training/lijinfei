use std::io;
fn get_result(mode: u32, temp: f32) -> (u32, f32) {
    if mode == 1 {
        // 输入1为摄氏度转华氏度
        (mode, temp * 1.8 + 32.0)
    } else if mode == 2 {
        // 输入2为华氏度转摄氏度
        (mode, (temp - 32.0) / 1.8)
    } else {
        (mode, 0.0)
    }
}
fn main() {
    let mut temp = String::new();
    let mut mode = String::new();
    println!("输入1为摄氏度转华氏度");
    println!("输入2为华氏度转摄氏度");
    // F = C * 1.8 +32
    // C = F -32 / 1.8
    io::stdin().read_line(&mut mode).expect("输入正确数值");
    let mode: u32 = mode.trim().parse().expect("输入值有误");
    println!("继续输入温度");
    io::stdin().read_line(&mut temp).expect("输入正确温度");
    let temp: f32 = temp.trim().parse().expect("输入值有误");
    let result = get_result(mode, temp);
    println!("转换后温度为：{:?}", result.1)
    // io::stdin().read_line(&mut temp).expect("输入正确温度");
}
