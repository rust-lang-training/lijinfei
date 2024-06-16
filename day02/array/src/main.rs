use std::io;
fn arr_add(numbers: &[u32]) -> u32 {
    let mut result: u32 = 0;
    for number in numbers {
        result += number
    }
    result
}
fn main() {
    println!("Hello, world!");
    println!("请输入以空格分割的整数");
    let mut str_arr = String::new();
    io::stdin().read_line(&mut str_arr).expect("输入错误");
    let str_arr = str_arr.trim();
    let str_arr = str_arr.split(" ");
    let mut index = 0;
    let mut arr = [0u32; 5];
    for v in str_arr {
        let v: u32 = v.trim().parse().expect("请输入正确的整数");
        arr[index] = v;
        index+=1;
        if index == 5 {
            break;
        }
    }
    println!("输入的数组为：{:?}", arr);
    let total = arr_add(&arr[..]);
    println!("数组之和为：{}", total);
}
