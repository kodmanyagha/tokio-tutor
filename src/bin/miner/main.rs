use std::{env, fmt::Write, time::Instant};

use byteorder::{BigEndian, ByteOrder};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use ring::digest::{Context, SHA256};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} <prefix> <difficulty>", args[0]);
        return;
    }

    let difficulty = match args[2].parse::<usize>() {
        Ok(val) => val,
        Err(_) => {
            println!("Difficulty is not a valid integer: {}", args[2]);
            return;
        }
    };
    println!("Difficulty: {}", difficulty);

    let start = Instant::now();

    let i = solve_multi(&args[1], difficulty);
    println!("solve_multi answer is {}", i);

    let i = solve_classical(&args[1], difficulty);
    println!("solve_multi answer is {}", i);

    let duration = start.elapsed();
    println!("Time elapsed in solve() is: {:?}", duration);
}

fn solve_multi(prefix: &str, difficulty: usize) -> u64 {
    let mut i: u64 = 0;
    let mut found = false;
    let chunk_size: u64 = 1_000_000;

    while !found {
        let result = (i..i + chunk_size).into_par_iter().find_any(|&i| {
            let mut context = Context::new(&SHA256);
            let data = format!("{}{}", prefix, i);
            context.update(data.as_bytes());
            let digest = context.finish();
            let mut hex = String::new();
            let mut counter = 0;

            for &byte in digest.as_ref() {
                counter += 1;
                write!(&mut hex, "{:02x}", byte).expect("Failed to write hex.");
            }

            let x = &digest.as_ref()[..8];
            // let y = BigEndian::read_u64(x);
            let leading_zeros = BigEndian::read_u64(x).leading_zeros();

            let result = leading_zeros >= difficulty as u32;

            if result {
                println!("Total hex char len: {}", counter);
                println!(
                    "x: {:?} , i: {} , hash: {} , zeros: {}",
                    x, i, hex, leading_zeros
                );
            }

            result
        });

        match result {
            Some(val) => {
                i = val;
                found = true;
            }
            None => {
                i += chunk_size;
            }
        }
    }

    i
}

fn meets_difficulty(hash_str: &str, difficulty: usize) -> bool {
    hash_str.starts_with(&"0".repeat(difficulty))
}

fn solve_classical(data: &str, difficulty: usize) -> u64 {
    let mut nonce = 0u64;
    loop {
        let mut context = Context::new(&SHA256);
        context.update(data.as_bytes());
        context.update(&nonce.to_be_bytes());

        let digest = context.finish();
        let mut actual_hash = String::new();

        for &byte in digest.as_ref() {
            write!(&mut actual_hash, "{:02x}", byte).expect("Failed to write hex.");
        }

        // println!("Nonce: {} , Hash: {}", nonce, actual_hash);

        if meets_difficulty(&actual_hash, difficulty) {
            println!(
                "Hash meets the difficulty {difficulty} leading zeros: {}",
                &actual_hash
            );
            break;
        }

        nonce += 1;
    }

    nonce
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let n = u64::MAX >> 2;
        assert_eq!(n.leading_zeros(), 2);

        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
