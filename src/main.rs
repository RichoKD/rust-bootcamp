mod week_one;

use crate::week_one::{
    day_one,
    day_three::{factorial, guessing_game, is_prime},
    day_two,
};

fn main() {
    day_one::main();
    day_two::main();
    println!("{}", factorial(12));
    println!("{}", is_prime(17));
    println!("{}", guessing_game());
}
