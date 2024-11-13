mod week_one;
mod week_two;

use crate::week_one::{
    day_one,
    day_three::{factorial, guessing_game, is_prime},
    day_two,
};

use crate::week_two::day_one::{convert_f_to_c, generate_fibonacci_num};

fn main() {
    day_one::main();
    day_two::main();
    println!("{}", factorial(12));
    println!("{}", is_prime(17));
    println!("{}", guessing_game());

    println!("\nWeek 2 \n ============================================================ \n");
    println!("Convert_F_to_C: {}", convert_f_to_c(100.0));
    println!("Nth fibonacci digit: {}", generate_fibonacci_num(10));
}
