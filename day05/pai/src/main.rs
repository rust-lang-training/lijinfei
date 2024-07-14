use std::{rc::Rc, sync::{Mutex, Arc}, thread};
use rand::Rng;
const NUM_POINTS: usize = 100000;
const NUM_THREADS: usize = 12;
fn main() {
    let result = Arc::new(Mutex::new(0u64));

    let mut threads = vec![];

    for _i in 0..NUM_THREADS {
        let result_clone = result.clone();
        let handler = thread::spawn(move || {
            let mut local_count = 0u64;
            let mut rng = rand::thread_rng();
            
            for _i in 0..NUM_POINTS {
                let x = rng.gen::<f64>();
                let y = rng.gen::<f64>();
                if x * x + y * y <= 1.0 {
                    local_count += 1;
                }
            }

            let mut result = result_clone.lock().unwrap();
            *result += local_count;
        });
        threads.push(handler);
    }

    for  thread in threads {
        thread.join().unwrap();
    }

    let result_clone = result.lock().unwrap();
    let pi = 4.0 * *result_clone as f64 / (NUM_POINTS * NUM_THREADS)  as f64;

    println!("pi is {}", pi);
}
