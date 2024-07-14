use std::{sync::{Arc, Mutex, mpsc}, thread, io::stdin};


const NUM_POINTS: u64 = 100000;
const NUM_THREADS: u64 = 10;

fn main() {
    // println!("Hello, world!");
    // 多线程+线程间共享可变数据的模式
    // 多线程 +mpsc 模式
    // 以上两种模式任选其一或两者都做

    // 多线程
    let mut choice = String::new();
    println!("输入1 根据多线程计算");
    println!("输入2 根据管道通信计算");
    stdin().read_line(&mut choice).unwrap();

    match choice.trim() {
        "1" => thread_fn(),
        "2" => channel_fn(),
        _ => println!("Unknown")
    }
    
}

fn thread_fn() {
    let mut threads = vec![];
    let result: Arc<Mutex<Vec<u64>>> = Arc::new(Mutex::new(vec![]));

    for i in 0..NUM_THREADS {
        let result_clone = result.clone();
        let handler = thread::spawn(move || {
            let mut local_result = vec![];
            let start = i * NUM_POINTS;
            let end = i * NUM_POINTS + NUM_POINTS;
            for j in start ..end {
                if is_prima(j) {
                    local_result.push(j);
                }
            }

            let mut result = result_clone.lock().unwrap();
            result.extend(local_result);
        });
        threads.push(handler);
    }
    for tr in threads {
        tr.join().unwrap();
    }
    let result = result.lock().unwrap();

    println!("100w 以内的质数: {:?}", result);
}

fn channel_fn() {
    let mut threads = vec![];

    let (sender, receiver) = mpsc::channel::<Vec<u64>>();
    let _logger_thread = thread::spawn(move || {
        while let Ok(s) = receiver.recv() {
            println!("100w 以内的质数: {:?}", s);
        }
    });

    for i in 0..NUM_THREADS {
        let tx = sender.clone();
        let handler = thread::spawn(move || {
            let mut local_result = vec![];
            let start = i * NUM_POINTS;
            let end = i * NUM_POINTS + NUM_POINTS;
            for j in start ..end {
                if is_prima(j) {
                    local_result.push(j);
                }
            }
            let _ =tx.send(local_result);
            
        });
        threads.push(handler);
    }
    for tr in threads {
        tr.join().unwrap();
    }
}


fn is_prima(num: u64) -> bool {
    if num >= 2 {
        let mut index = 2;
        let result = loop {
            if index * index <= num {
                if num % index == 0 {
                    break false;
                }
                index += 1;
            } else {
                break true;
            }
        };
        result
    } else {
        false
    }
}
