use std::{borrow::Borrow, path::PathBuf, slice};

use chrono::Timelike;
use clap::{arg, command, value_parser, ArgAction, Command};

struct Fibonacci {
    counter: u64,
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.counter == 0 {
            return None;
        }

        let mut total = 0_u64;

        for i in 1..=self.counter {
            total += i;
        }
        self.counter -= 1;

        Some(total)
    }
}

trait AnimalMovement {
    fn run(&self);
    fn walk(&self);
    fn talk(&self);
    fn look(&self, x: f32, y: f32, z: f32);
}

type AnimalBox = Box<dyn AnimalMovement>;

struct Dog {
    name: String,
}

impl AnimalMovement for Dog {
    fn run(&self) {
        println!("{} dog running", self.name);
    }

    fn walk(&self) {
        println!("{} dog walking", self.name);
    }

    fn talk(&self) {
        println!("{} dog talking", self.name);
    }

    fn look(&self, x: f32, y: f32, z: f32) {
        println!("{} dog looking to {},{},{}", self.name, x, y, z);
    }
}

fn box_example() {
    let dog1 = Dog {
        name: "charlie".to_string(),
    };
    let animal1: AnimalBox = Box::new(dog1);

    get_box_animal(&animal1);
    get_box_animal(&animal1);
}

fn get_box_animal(animal: &AnimalBox) {
    animal.run();
    animal.talk();
}

fn unsafe_example() {
    let story = "Once upon a time...";
    let ptr = story.as_ptr();
    let len = story.len();

    assert_eq!(19, len);

    let s = unsafe {
        let slice = slice::from_raw_parts(ptr, len);
        std::str::from_utf8(slice)
    };

    let x = {
        let time = chrono::Utc::now().timestamp_millis();
        time % 10
    };

    assert_eq!(s, Ok(story));
}

fn main() {
    let _a = Command::new("cmd").arg(arg!([name] "Optional name to operate on")
        .required(false));

    let _x = command!("");

    let _matches = command!()
        .arg(arg!([name] "Optional name to operate on"))
        .arg(
            arg!(
                -c --config <FILE> "Sets a custom config file"
            )
                .required(false)
                .value_parser(value_parser!(PathBuf)),
        )
        .subcommand(
            Command::new("test")
                .about("does testing things")
                .arg(arg!(-l --list "lists test values").action(ArgAction::SetTrue)),
        )
        .get_matches();

    let fib1 = Fibonacci { counter: 10 };
    for f in fib1 {
        println!("f: {}", f);
    }

    box_example();
    unsafe_example();
}
