use std::{
    sync::{mpsc, Arc},
    thread,
    time::Duration,
};

pub fn thread_example() {
    //let (tx, rx): (Sender<i32>, Receiver<bool>) = mpsc::channel();
    start_thread(3, Arc::new(|i| println!("fn invoked, i: {}", i)));
}

pub fn start_thread(until: i32, f: Arc<dyn Fn(i32) -> () + Send + Sync>) {
    let x = String::from("test");

    let child = thread::spawn(move || {
        for i in 1..until {
            thread::sleep(Duration::from_millis(555));
            f(i);
        }
    });

    let _ = child.join();

    println!("İşlem tamam");
}
