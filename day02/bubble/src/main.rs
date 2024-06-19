use rand::Rng;
fn main() {
    println!("生成10个随机数");
    let mut numbers = [0u32; 10];
    for index in 0..10 {
        numbers[index] = rand::thread_rng().gen_range(1..=100);
    }
    println!("生成的随机数为：{:?}", numbers);
    // 选择排序
    let result = select_sort(numbers);
    println!("排序后随机数为：{:?}", result);
}
fn select_sort(mut numbers: [u32;10]) -> [u32;10]{
    let mut min_index = 0;
    for i in 0..numbers.len() {
        min_index = i;
        for j in i..numbers.len() {
            if numbers[j] < numbers[min_index]  {
                min_index = j;
            }
        }
        if min_index != i {
            numbers.swap(min_index, i)
        }
    }
    return numbers
}
