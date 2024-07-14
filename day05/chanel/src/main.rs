use std::{sync::mpsc, thread, time::Duration};

fn main() {
    let (sender, receiver) = mpsc::channel::<String>();
    let _logger_thread = thread::spawn(move || {
        while let Ok(s) = receiver.recv() {
            println!("{:?} {}", thread::current().id(), s);
        }
    });

    let mut sender_threads = vec![];


    for _i in 0..5 {
        let tx = sender.clone();
        sender_threads.push(thread::spawn(move || {
            let msg = format!("this is a messsage from {:?}", thread::current().id());
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_secs(1));
        }))
    }

    for st in sender_threads {
        st.join().unwrap();
    }
}
