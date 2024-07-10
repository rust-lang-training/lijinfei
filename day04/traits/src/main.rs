use std::{fmt::Display, path::Path, io, ops::{Mul, Add}};

fn main() {
    println!("Hello, world!");
    let headline = String::from("headline");
    let location = String::from("location");
    let author = String::from("author");
    let content = String::from("content");
    let artical1 = NewsArtical::new(headline, location, author, content);
    print!("{}", artical1);
    notify(&artical1);


    let x = 42;
    let c1 = || println!("Hello World");
    let c2 =  || println!("x = {}", x);
    let c3 = || {
        let s = String::from("hello local variable");
        println!("local variable = {}", s);
    };

    let mut s = String::from("Hello");
    let mut s1 = || s.push_str(" Rust closure");
    // fn s2() {
    //     println!("s = {}", s);
    // }
    s1();
    println!("s = {}", s);


    let s2 = String::from("Hello World!");
    let closure  = move || {
        let v = (s2, 1);
        println!("{:?}", v);
    };
    closure();

    let add_one = |i: i32| i + 1;
    let y = add_one(10);
    println!("{}", y);
    // println!("{}", s2);
    let s = String::from("");

}

struct Rectangle<T> {
    width: T,
    height: T
}

impl<T: Mul<Output = T> + Copy + Add<Output = T> + From<i8>> Rectangle<T> {
    fn area(&self) -> T {
        self.width * self.height
    }
    fn perimeter(&self) -> T {
        // self.width + self.height + self.width + self.height 
        (self.width + self.height) * T::from(2)
    }
}


fn check_file<P: AsRef<Path>>(item: P) -> Result<(), io::Error>{
    Ok(())
}
// fn notify(item: &impl Summary) {
//     println!("News: {}", item.summarize());
// }

// fn notify<T: Summary> (item: &T) {
//     println!("News: {}", item.summarize())
// }

fn notify<T> (item: &T) where T: Summary {
    println!("News: {}", item.summarize())
}
trait Summary {
    fn summarize(&self) -> String;
}
trait Like {
    fn like(&self) -> String;
}


struct NewsArtical {
    headline: String,
    location: String,
    author: String,
    content: String,
}
impl NewsArtical {
    fn new(headline: String, location: String, author: String, content: String) -> Self {
        Self {
            headline,
            location,
            author,
            content
        }
    }
}
impl Display for NewsArtical {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.headline)
    }
}
impl Summary for NewsArtical {
    fn summarize(&self) -> String {
        format!("{} , by {} {}", &self.headline, &self.author, &self.location)
    }
}


struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Tweet {
    fn new(username: String, content: String, reply: bool, retweet: bool) -> Self {
        Self {
            username,
            content,
            reply,
            retweet
        }
    }
}
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{} , {}", &self.username, &self.content)
    }
}