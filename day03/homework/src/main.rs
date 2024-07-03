use std::{vec, io::stdin};

fn main() {
    // 编写一个数组工具，实现以下函数(具体是传入引用还是值，传入哪种引用，自行决定)
    // ·传入一个 i32 切片，返回其中最大的值
    // ·传入一个 i32 切片，返回其中最小的值
    // 传入一个 i32 切片，返回其中最大的值的引用
    // 传入一个 i32 切片，返回其中最小的值的引用。
    // 传入一个 i32 切片，返回元素的平均值。
    // ·传入一个 i32 切片，将其中的每个元素都放大到原来的2倍
    // let vec1: Vec<i32> = vec![23, 2, 32];
    let mut arr1 = ArrayUtils::new();
    let mut str = String::new();
    println!("请输入空格分开的值");
    stdin().read_line(&mut str).expect("请输入正确的值");
    for num in str.trim().split(" ") {
        let num:i32 = num.parse().expect("请输入正确的值");
        arr1.push(num);
    }
    println!("您输入的值为：{:?}", arr1);
    let descrip = [
        "返回其中最大的值",
        "返回其中最小的值",
        "返回其中最大的值的引用",
        "返回其中最小的值的引用",
        "返回元素的平均值",
        "将其中的每个元素都放大到原来的2倍"        
    ];

    for i in 0..descrip.len() {
        println!("输入{}，{}", i+1, descrip[i]);
    }
    let input_mode = [
        ArrayMode::Max,
        ArrayMode::Min,
        ArrayMode::RefMax,
        ArrayMode::RefMin,
        ArrayMode::Average,
        ArrayMode::Scale,
    ];
    
    let mut input_str = String::new();
    stdin().read_line(&mut input_str).expect("请输入正确的值");
    let mode: u8 = input_str.trim().parse().expect("请输入正确的值");
    match Option::Some(mode) {
        Some(x @ 1..=7) => {
            let mode = &input_mode[x as usize - 1];
            get_array(arr1, mode);
        },
        Some(_x ) => {
            println!("请输入正确的值");
        },
        None => {
            println!("请输入正确的值");
        }
    }
    // arr1.push(23);
    // arr1.push(22);
    // arr1.push(21);
    // println!("{:?}", arr1.max());
    // println!("{:?}", arr1.min());
    
}
fn get_array(arr: ArrayUtils, mode: &ArrayMode) {
    match mode {
        &ArrayMode::Max => {
            println!("最大的值为: {}", arr.max());
        },
        &ArrayMode::Min => {
            println!("最小的值为: {}", arr.min());
        },
        &ArrayMode::RefMax => {
            println!("最大的值的引用为: {}", arr.ref_max());
        },
        &ArrayMode::RefMin => {
            println!("最大的值的引用为: {}", arr.ref_min());
        },
        &ArrayMode::Average => {
            println!("元素的平均值为: {}", arr.average());
        },
        &ArrayMode::Scale => {
            println!("将其中的每个元素都放大到原来的2倍: {:?}", arr.scale());
        }
    }
}


#[derive(Debug)]

enum ArrayMode {
    Max,
    Min,
    RefMax,
    RefMin,
    Average,
    Scale
}
#[derive(Debug)]
struct ArrayUtils {
    value: Vec<i32>
}

impl ArrayUtils {
    fn new() -> Self{
        Self {
            value: vec![]
        }
    }
    fn push(&mut self, v: i32){
        self.value.push(v);
    }
    fn max(&self) -> i32 {
        let value = &self.value;
        let len = value.len();
        let mut max_value = value[0];
        for i in 1..=len-1 {
            if max_value < value[i] {
                max_value = value[i];
            }
        }
        max_value
    }
    fn min(&self) -> i32 {
        let value = &self.value;
        let len = value.len();
        let mut min_value: i32 = value[0];
        for i in 1..=len-1 {
            if min_value > value[i] {
                min_value = value[i];
            }
        }
        min_value
    }
    fn ref_max(&self) -> &i32 {
        let value = &self.value;
        let len = value.len();
        let mut max_index = 0;
        for i in 1..=len-1 {
            if value[max_index] < value[i] {
                max_index = i;
            }
        }
        &value[max_index]
    }
    fn ref_min(&self) -> &i32 {
        let value = &self.value;
        let len = value.len();
        let mut min_index = 0;
        for i in 1..=len-1 {
            if value[min_index] > value[i] {
                min_index = i;
            }
        }
        &value[min_index]
    }
    fn average(&self) -> i32 {
        let mut count = 0i32;
        let value = &self.value;
        for i in 0..=value.len()-1 {
            count = count + value.clone()[i]
        }
        count / (value.len() as i32)
    }
    fn scale(&self) -> ArrayUtils{ 
        let mut arr = ArrayUtils::new();
        let value = &self.value;
        for i in 0..value.len() {
            arr.push(value.clone()[i] * 2);
        }
        arr
    }
}