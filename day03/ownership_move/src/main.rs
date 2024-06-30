fn main() {
    println!("Hello, world!");
    // let s = String::from("hello");
    // let s2 = s;
    // println!("{s}")

    let s1 = String::from("hello");
    println!("{s1}");
    take_ownership(&s1);
    println!("{s1}");

    let x = 5;
    mask_copy(x);
    println!("{x}");



    let names = [
        String::from("test1"),
        String::from("test2"),
        String::from("test3"),
        String::from("test4"),
    ];
    // for i in 0..4 {
    //     // let names = names.clone();
    //     // let name = names[i];
    //     // let name = names[i];
    //     // println!("{name}", i)
    // }

    let s = String::from("hello");
    let bytes = s.clone().into_bytes();
    println!("{s}, {:?}", bytes);


}

fn take_ownership(s: &String) {
    println!("inner log{s}");
}

fn mask_copy(x: i32) {
    println!("{x}");
}