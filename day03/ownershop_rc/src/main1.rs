use std::{
    // rc::Rc, 
    cell::RefCell};
fn main() {
    // let s1 = Rc::new(String::from("test"));
    // let s2 = s1.clone();
    // let s3 = s1.clone();


    // println!("ref count {}", Rc::strong_count(&s1));
    // println!("ref count {}", Rc::strong_count(&s2));
    // println!("ref count {}", Rc::strong_count(&s3));

    // // 加上作用域范围
    // let s1 = Rc::new(String::from("Hello"));
    // println!("ref count {}", Rc::strong_count(&s1));
    // {
    //     let s2 = s1.clone();
    //     println!("ref count {}", Rc::strong_count(&s2));
    //     {
    //         let s3 = s1.clone();
    //         println!("ref count {}", Rc::strong_count(&s3));
    //     }
    //     println!("ref count {}", Rc::strong_count(&s2));
    // }
    // println!("ref count {}", Rc::strong_count(&s1));


    // let s = RefCell::new(String::from("Hello World!"));
    // // append_string(&s);
    // // println!("s is {}", s.borrow());


    // let mut user1 = User {
    //     username: String::from("user1"),
    //     age: 18u32,
    //     email: String::from("1231231@q..comdw"),
    //     active: true
    // };
    // user1.active = false;
    // let User {
    //     username: username2,
    //     age,
    //     email,
    //     active
    // } = user1;
    // // println!("{}", user1.username);
    // println!("{}", username2);
    // println!("{}", age);
    // println!("{}", email);
    // println!("{}", active);

    let p = Point(1.0, 2.0);
    println!("point: ({}, {})", p.0, p.1);

    let Point(x, y) = p;
    println!("point: ({}, {})", x, y);
    
}
struct User {
    username: String,
    age: u32,
    email: String,
    active: bool
}
struct Point(f32, f32);



fn append_string(s: &RefCell<String>) {
    let rs = s.borrow();
    let mut ms = s.borrow_mut();
    (*ms).push_str(" I'm Rust");
    println!("s is {}", rs);
}