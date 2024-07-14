use std::{sync::{Arc, Mutex}, thread};

fn main() {
    

    let count = Arc::new(Mutex::new(0u8));


    let mut threads = vec![];


    for _i in 0..10 {
        let count =  count.clone();
        let handler = thread::spawn(move || {
            let mut num = count.lock().unwrap();
            *num +=  2;
        });

        threads.push(handler);
    }

    for handler in threads {
        handler.join().unwrap();
    }

    println!(" count is {}", count.lock().unwrap());
}
