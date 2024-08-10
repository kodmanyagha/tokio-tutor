mod create_user;
mod thread_example;
mod trait_example;

use std::ops::Deref;

use anyhow::Error;
use create_user::{EmailAddress, Password, Username};
use thread_example::thread_example;
use trait_example::run_trait_example;

/// A transparent wrapper around any type `T`.
struct SmartBox<T>(T);

impl<T> Deref for SmartBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> SmartBox<T> {
    fn print_best_rust_resource() {
        println!("howtocodeit.com");
    }
}

struct ConfusedUnitStruct;

impl ConfusedUnitStruct {
    fn print_best_rust_resource(&self) {
        println!("Scrabbling around on YouTube or something?");
    }
}

fn inherent_demonstrate() {
    let smart_box = SmartBox(ConfusedUnitStruct);
    smart_box.print_best_rust_resource();
    SmartBox::<ConfusedUnitStruct>::print_best_rust_resource();
}

fn execute_create_user() -> Result<(), Error> {
    let crate_user_result = create_user::create_user(
        Username::new("foo")?,
        EmailAddress::new("bar@test.com")?,
        Password::new("TestPass!")?,
    )?;

    println!("Create user result: {0}", crate_user_result.get_username());

    Ok(())
}

#[derive(Debug)]
struct Name<'a> {
    name: &'a str,
}

fn main() -> Result<(), Error> {
    thread_example();
    run_trait_example();

    execute_create_user()?;
    inherent_demonstrate();

    let name_str = String::from("bob");
    let n = Name { name: &name_str };
    println!("Name struct: {:?}", n);

    let n = Name { name: "foo" };
    println!("Name struct: {}", n.name);

    Ok(())
}
