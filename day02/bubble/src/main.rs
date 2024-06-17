use rand::Rng;
fn main() {
    println!("生成10个随机数");
    let mut numbers = [0u32; 10];
    for index in 0..10 {
        numbers[index] = rand::thread_rng().gen_range(1..=100);
    }
    println!("生成的随机数为：{:?}", numbers);
    for i in 0..numbers.len() {
        for j in 0..numbers.len() - i -1{
            let v = numbers[j];
            let v2 = numbers[j+1];
            if v > v2 {
                numbers.swap(j, j+1);
            }
        }
    }
    println!("排序后随机数为：{:?}", numbers);
}
