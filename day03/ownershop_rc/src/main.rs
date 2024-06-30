fn main() {
    let r1 = Rect::new(32.0, 22.0);
    println!("{:?}", r1);


    let q = Message::Quit;
    let m = Message::Move {x: 100, y: 100};
    let w = Message::Write(String::from("hello"));
    let c = Message::ChangeColor(12,23,12);
    println!("{:?}", q);
    println!("{:?}", m);
    println!("{:?}", w);
    println!("{:?}", c);
}
#[derive(Debug)]
struct Rect {
    width: f32,
    height: f32,
}

impl Rect {
    fn new(width: f32, height: f32) -> Self {
        Self {
            width,
            height
        }
    }
    fn square(size: f32) -> Self {
        Self {
            width: size,
            height: size
        }
    }
}
#[derive(Debug)]
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32)
}