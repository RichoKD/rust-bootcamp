use std::io;
use std::io::Write; // for flush

fn calc_factorial(mut n: u32) -> u32 {
    let mut result: u32 = n.clone();

    loop {
        if n == 1 {
            return result;
        }

        n -= 1;

        result = result * n;
    }
}

fn is_prime(n: u32) -> bool {
    if n == 2 {
        return true;
    }

    if n % 2 == 1 {
        return true;
    } else {
        return false;
    }
}

fn match_prime(n: u32) -> bool {
    match n {
        2 => true,

        n if (n % 2 == 1) => true,

        _ => false,
    }
}

// fn prnt_out(prompt: &str){

// }

fn input_any(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn input_u32(prompt: &str) -> u32 {
    input_any(prompt)
        .trim()
        .parse::<u32>()
        .expect("Invalid input. Please enter a number.")
}

// fn proximity(old:u32, new:u32) -> bool{
//     if
// }

fn guessing_game(num: u32) {
    // let mut
    loop {
        let guess: u32 = input_u32("Guest a number from 0 to 99: ");
        if num == guess {
            println!("Bingo");
            break;
        } else if num < guess {
            println!("Warm");
        } else {
            println!("Cold");
        }

        // if num < guess
    }
}

// use std::rand;
use rand::prelude::*;

pub fn main() {
    let num: u32 = rand::thread_rng().gen_range(0..100);
    println!("{}", num);

    println!("{}", calc_factorial(5));
    println!("{}", 4 % 2);
    println!("Is Prime: {}", match_prime(5));

    assert!(calc_factorial(5) == 120, "Not Factorial");
    assert!(match_prime(3), "Not Prime");

    guessing_game(num);
}
