use anyhow::{anyhow, Error};

fn main() {
    start().unwrap();
}

fn start() -> anyhow::Result<()> {
    let ret_val = func1(1, 2)
        .and_then(move |val| {
            println!("val 1: {:?}", val);
            let val = val + 1;
            Ok(val)
        })
        .and_then(move |val| {
            println!("val 2: {:?}", val);
            Ok(val)
        })
        .map_err(|_| anyhow!("Unknown error occured"))?;
    println!("ret_val: {:?}", ret_val);

    Ok(())
}

fn func1(no1: i32, no2: i32) -> anyhow::Result<i32> {
    Ok(no1 + no2)
}

fn calculate_two_numbers(
    no1: i32,
    no2: i32,
    cb: fn(i32, i32) -> anyhow::Result<i32>,
) -> anyhow::Result<i32> {
    Ok(cb(no1, no2).map_err(|_| anyhow!("An error occured"))?)
}
