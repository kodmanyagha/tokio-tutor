use std_ops::std_ops1;

pub mod std_ops;

#[derive(Default, Debug, Clone, Copy)]
struct Arithmetic {
    pub no1: i32,
}

fn main() {
    copy1();
    trait1();

    std_ops1();
}

fn copy1() {
    let mut a1 = Arithmetic::default();
    let mut a2 = a1;
    a1.no1 = 10;
    a2.no1 = 20;

    println!("a1: {:?}", a1);
    println!("a2: {:?}", a2);
}

trait Animal {
    fn make_sound(&self);
}

#[derive(Default)]
struct Dog;

#[derive(Default)]
struct Cat;

#[derive(Default)]
struct Lion;

impl Animal for Dog {
    fn make_sound(&self) {
        println!("Dog barking");
    }
}

impl Animal for Cat {
    fn make_sound(&self) {
        println!("Cat miyav");
    }
}

impl Animal for Lion {
    fn make_sound(&self) {
        println!("Lion roar");
    }
}

fn trait1() {
    let mut animals: Vec<Box<dyn Animal>> = vec![];
    animals.push(Box::new(Dog));
    animals.push(Box::new(Dog));
    animals.push(Box::new(Cat));

    for animal in animals {
        animal.make_sound();
    }
}
