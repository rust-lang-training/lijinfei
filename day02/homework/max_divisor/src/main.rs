use std::io;

fn main() {
    println!("请输入空格分割的两个数字，我将计算这两个数的最大公约数");
    let mut input_str = String::new();
    io::stdin().read_line(&mut input_str).expect("请输入正确的内容");
    let mut input_arr = [0u32; 2];
    let input_str = input_str.trim().split(" ");
    let mut index = 0u8;
    for str in input_str {
        if index < 2 {
            let str:u32 = str.parse().expect("请输入正确的值");
            input_arr[index as usize] = str;
        }
        index+=1;
        
    }
    println!("将计算 {} 和 {} 的最大公约数，请耐心等待", input_arr[0], input_arr[1]);

    let result = get_common_divisor(input_arr);
    println!("{} 和 {} 的最大公约数为 {}", input_arr[0], input_arr[1], result);
}
fn  get_common_divisor(input_arr: [u32;2]) -> u32 {
    if input_arr[0] == input_arr[1] {
        return input_arr[0]
    } else if input_arr[0] > input_arr[1] {
        return get_divisor(input_arr[0], input_arr[1])
    } else {
        return get_divisor(input_arr[1], input_arr[0])
    }
}
fn get_divisor(max: u32, min: u32) -> u32{
    let mut new_max = max;
    let mut new_min = min;
    loop {
        let remainder =  new_max % new_min;
        if remainder == 0 {
         break new_min
        } else {
            new_max = new_min;
            new_min = remainder
        }
    }
}