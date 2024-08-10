#![allow(unused)]

use std::cell::RefCell;
use std::rc::Rc;

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
