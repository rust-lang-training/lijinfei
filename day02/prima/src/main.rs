use std::io;
fn is_prima(num: u32) -> bool {
    if num >= 2 {
        let mut index = 2;
        let result = loop {
            if index * index <= num {
                if num % index == 0 {
                    break false;
                }
                index += 1;
            } else {
                break true;
            }
        };
        result
    } else {
        false
    }
}
fn main() {
    // 是否质数
    println!("输入一个整数，我将检测他是否为质数");
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("请输入正确的数值");
    let num: u32 = num.trim().parse().expect("请输入正确的数值");

    let result = is_prima(num); 
    println!("{}是否为质数{}", num, result);
}
