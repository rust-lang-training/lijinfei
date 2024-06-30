use std::collections::HashMap;


// let skill_table = HashMap();
fn main() {
    // // 解引用
    // let x1 = 11;
    // let y1 = &x1;
    // // assert!(*y1 == 10);
    // // assert!(y1 == &10);


    // let mut x2 = 10;
    // let y2 = &mut x2;
    // *y2 = 2;

    // let x3 = String::from("hello");
    // let len = calculate_length(&x3);
    // println!("{} {}", x3, len);

    // let x4 = 5;
    // let y4 = &&&&&x4;
    // // assert!(*****y4 == 10);


    // let mut x5 = String::from("Hello");
    // let x6 = &x5;
    // let x7 = &x5;
    // let x8 = &mut x5;
    // // println!("{}, {}, {}", x6, x7, x8);

    // let x8 = String::from("test1");
    // {

    //     let y8 = String::from("test22");
    //     let x9 = longest(&x8, &y8);
    //     println!("{x9}");
    // }

    // let mut x10 = String::from("Hello");
    // let rx10 = &x10;
    // // push_str 发生了可变引用
    // // x10.push_str(" I'm xx");
    // println!("{} {}", x10, rx10);



    // let x11 = String::from("hello");
    // let x12 = &x11;
    // let x13 = x11;//cannot move out of `x11` because it is borrowed
    // println!("{}", x12);


    // let x14 = gen_string().as_bytes();
    let binding = gen_string();
    let bytes = binding.as_bytes();
    println!("{:?}", bytes);
}
// 生命周期说的是x 和 y 的生命周期得一致
fn longest<'a>(x: &'a String, y: &'a String) -> &'a String {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn calculate_length(v: &String) -> usize {
    let x = &v;
    let y = &x;
    let z = &y;
    println!("{}", ****z);
    return z.len();
}

fn gen_string() -> String {
    String::from("Hello World")
}