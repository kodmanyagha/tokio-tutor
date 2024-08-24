#![allow(unused)]

mod refcell_tutor;
mod weak_tutor;

use std::rc::Rc;
use std::{cell::RefCell, ops::Sub};

use chrono::{DateTime, Utc};
use refcell_tutor::refcell_borrow_mut;
use weak_tutor::weak_example;

#[derive(Debug)]
struct Person {
    pub name: String,
    pub age: u32,
}

fn main() {
    let mut person1 = Rc::new(RefCell::new(Person {
        name: "emir".to_string(),
        age: 35_u32,
    }));

    let mut person2 = Rc::clone(&person1);
    let mut person3 = Rc::clone(&person2);
    let mut person4 = Rc::clone(&person3);
    let mut person5 = person3.clone();

    //*person4.age = 28_u32;
    let mut p1 = person4.borrow_mut();
    p1.age = 33_u32;
    drop(p1);

    println!("Reference counting: {}", Rc::strong_count(&person4));
    println!("person1: {:?}", *person1);
    println!("person2: {:?}", *person2);
    println!("person3: {:?}", *person3);
    println!("person4: {:?}", *person4);

    double_ref();

    filter_example();

    ptr_example_1();
    ptr_example_2();

    rc_example_1();

    refcell_borrow_mut();
    weak_example();
}

struct User {
    pub id: u32,
    pub username: String,
}

fn double_ref() {
    let val1 = User {
        id: 10,
        username: "foo".to_string(),
    };

    let ref1 = &val1;
    let ref2 = &&val1;
    let ref3 = *ref2;

    //let val2 = *ref1;
    //let val3 = **ref2;
}

fn mutable_params(mut u1: User, u2: &mut User) {
    u2.username = u1.username;
}

fn filter_example() {
    // consider we have 10K names in here.
    let names = vec![
        "foo".to_string(),
        "bar".to_string(),
        "baz".to_string(),
        "thousands of values will be here".to_string(),
    ];
    // this value will have almost 90K names.
    let filtered_names_cloned = filter_vec_cloned(&names, "a".to_string());
    // now we have 10K and 90K names storing in memory.

    println!("{:?}", filtered_names_cloned);

    let filtered_names = filter_vec(&names, "o");
    for filtered_name in filtered_names {
        println!("Filtered name: {filtered_name}");
    }
}

fn filter_vec_cloned(v: &Vec<String>, filter_str: String) -> Vec<String> {
    v.iter()
        .filter(|item| item.contains(filter_str.as_str()))
        .cloned()
        .collect()
}

fn filter_vec<'a>(v: &'a [String], filter_str: &'a str) -> impl Iterator<Item = &'a String> + 'a {
    v.iter().filter(move |item| item.contains(filter_str))
}

// impl, Fn, FnOnce, impl trait, etc...

fn fn_once_1(fn1: impl FnOnce() -> String) {
    let str1 = fn1();
    println!("str1: {str1}");
}

#[derive(Default, Debug)]
struct Subscription {
    pub id: u64,
    pub user_id: u64,
    pub date: DateTime<Utc>,
}

fn ptr_example_1() {
    let mut foo = Box::new(Subscription::default());
    println!("foo: {:?}", foo.id);

    *foo.as_mut() = Subscription {
        id: 10,
        user_id: 20,
        date: Utc::now(),
    };
    println!("foo: {:?}", foo);

    *foo = Subscription {
        id: 30,
        user_id: 40,
        date: Utc::now(),
    };
    println!("foo: {:?}", foo);
}

fn ptr_example_2() {
    /*
    If you want to replace the whole thing, you can do my_ref_cell.replace(new_value)
    But if you want to get a mutable reference to the stuff inside, you need to do my_ref_cell.borrow_mut()
    https://doc.rust-lang.org/stable/std/cell/struct.RefCell.html#method.borrow_mut
    https://doc.rust-lang.org/book/ch15-05-interior-mutability.html

    You can't directly modify inside an Rc.
    You can directly modify inside a RefCell, even if you only have a &RefCell<T>.
    Also, there's not really a reason to put a Box inside an Rc.

    can anyone recommend a good benchmark lib?
    The most popular one is criterion. A newer alternative is divan

    idk
    ¯\_(ツ)_/¯
    */
    // there's not really a reason to put a Box inside an Rc
    // let rc1 = Rc::new(Box::new(Subscription::default()));

    let rc2 = Rc::new(RefCell::new(Subscription::default()));
    println!("{:?}", rc2);

    *rc2.borrow_mut() = Subscription {
        id: 50,
        user_id: 60,
        ..Default::default()
    };
    println!("{:?}", rc2);

    rc2.replace(Subscription {
        id: 70,
        user_id: 80,
        ..Default::default()
    });
    println!("{:?}", rc2);

    //Box::into_raw(b)
}

#[derive(Default, Debug)]
struct Payment {
    pub amount: f64,
    pub date: DateTime<Utc>,
}

fn rc_example_1() {
    let rcell_val_1 = RefCell::new(10);
    let mut rcell_mut_val_1 = rcell_val_1.borrow_mut();
    //let mut rcell_mut_val_2 = rcell_val_1.borrow_mut();
    //let mut rcell_mut_val_3 = rcell_val_1.borrow_mut();

    *rcell_mut_val_1 = 20;

    // You must drop the mutable reference becaouse at the next line you want to borrow.
    drop(rcell_mut_val_1);

    println!("Val: {}", rcell_val_1.borrow());

    let rc_payment = Rc::new(RefCell::new(Payment {
        amount: 10_f64,
        date: Utc::now(),
    }));

    // let x = *rc_payment;
    *rc_payment.borrow_mut() = Payment::default();
    println!("{:?}", rc_payment);

    rc_payment.replace(Payment::default());
    println!("{:?}", rc_payment);

    // let rc1 = *rc_payment;
    *rc_payment.borrow_mut() = Payment::default();

    let mut rc_string = Rc::new("test".to_string());
    // let x = *rc_string;
    // *rc_string = "foo".to_string();

    // rc_payment.borrow_mut() = RefCell::new(Payment {
    // amount: 20_f64,
    // date: Utc::now(),
    // });
    // println!("{:?}", rc_payment);
}
