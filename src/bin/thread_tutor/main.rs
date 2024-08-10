use std::{
    sync::{Arc, Mutex},
    thread::{self, sleep},
    time::Duration,
};

use anyhow::anyhow;

#[derive(Default, Debug)]
struct User<'a> {
    pub id: u64,
    pub username: &'a str,
}

fn main() -> anyhow::Result<()> {
    let user = User::default();
    let arc_user = Arc::new(Mutex::new(user));

    for _ in 0..22 {
        let arc_user_copy = Arc::clone(&arc_user);
        thread::spawn(move || change_user_id(arc_user_copy));
    }

    let arc_user_copy = arc_user.lock().unwrap();
    println!("1- user id: {}", arc_user_copy.id);
    drop(arc_user_copy);

    sleep(Duration::from_millis(1500));

    let arc_user_copy = arc_user.lock().unwrap();
    println!("2- user id: {}", arc_user_copy.id);

    let th = thread::spawn(run_me);

    println!("This line is staying after the thread.");

    th.join()
        .map_err(|_| anyhow!("An error occured when executing thread."))?;

    try_race_condition_1();
    try_race_condition_2();

    Ok(())
    // thread::sleep(Duration::from_secs(1));
}

fn change_user_id(arc_user: Arc<Mutex<User<'_>>>) {
    let mut user = arc_user.lock().unwrap();
    println!("Increasing user id: {:?}", user);

    user.id += 1;
}

fn run_me() {
    println!("Running this in different thread.");
    thread::sleep(Duration::from_secs(1));
    println!("And now quitting.");
}

fn try_race_condition_1() {
    let counter = Arc::new(0);
    let counter_clone1 = counter.clone();
    let counter_clone2 = counter.clone();

    let thread1 = thread::spawn(move || {
        let mut value = *counter_clone1;
        value += 1;
        println!("thread1 value: {value}");
    });

    let thread2 = thread::spawn(move || {
        let mut value = *counter_clone2;
        value += 1;
        println!("thread2 value: {value}");
    });

    thread1.join().unwrap();
    thread2.join().unwrap();

    println!("Final counter value: {}", *counter);
}

fn try_race_condition_2() {
    let counter = Arc::new(Mutex::new(0));
    let counter_clone1 = counter.clone();
    let counter_clone2 = counter.clone();

    let thread1 = thread::spawn(move || {
        let mut value = counter_clone1.lock().unwrap();
        *value += 1;
    });

    let thread2 = thread::spawn(move || {
        let mut value = counter_clone2.lock().unwrap();
        *value += 1;
    });

    thread1.join().unwrap();
    thread2.join().unwrap();

    println!("Final counter value: {}", *counter.lock().unwrap());
}
