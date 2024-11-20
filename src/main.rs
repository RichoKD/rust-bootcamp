mod day_three;
mod day_two;

// fn main() {
//     gm_to_rust();
//     // day_two::main();
//     // day_three::main();
// }

// function to print GM Rust!
fn gm_to_rust() {

    let first: &str = "GMzzzzzz";

    {
        let second: &str = "Rust!";
        println!("GM Rust! {}", longest(first, second));
    }

    // println!("GM Rust! {}", longest(first, second));

    vec_test(0);
    vec_test(5);
    vec_test(2);
    vec_test(9);

}

fn longest<'a>(x:&'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }

}

fn vec_test( indx: usize) {
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    // for i in &v {
    //     println!("{}", i);
    // };

    let mut v2 = vec![1, 2, 3, 4, 5];
    // let third: &i32 = &v2[2];
    // println!("The third element is {}", third);

    v2.push(6);

    match v2.get(indx) {
        Some(res) => println!("Element [{indx}] is {}", res),
        None => println!("There is no element at [{}]", indx),
        _ => println!("Default")
        
    }
}

// To avoid typing long module paths, you can use the use keyword to bring items into scope.
// use crate::week_one::{
//     day_one,
//     day_three::{factorial, guessing_game, is_prime},
//     day_two,
// };

// use crate::week_two::{
//     day_one::{christmas_song, fibonacci, temp_converter},
//     day_two::calculator::{addition, division, multiplication, subtraction},
// };

#[derive(Debug)]
struct Rect {
    widht: u32,
    height: u32
}

// fn area(h:u32, w:u32) -> u32 {
//     h * w
// }

impl Rect {

    fn new() -> Self{
        Self{
            widht: 45,
            height: 43,
        }
    }

    fn area(&self) -> u32 {
        self.height * self.widht
    }


}

// The main function in Rust serves as the entry point for your program's execution.
fn main() {

    let mut rc = Rect{
        widht: 40,
        height: 34
    };

    rc.height = 55;

    dbg!(&rc);

    let mut rc2:Rect = Rect::new();

    rc2.widht = 10;
    println!("Area: {}", rc2.area());

    // println!("{}", area(rc.height, rc.widht));
    // day_one::main();
    // day_two::main();
    // println!("{}", factorial(12));
    // println!("{}", is_prime(17));
    // println!("{}", guessing_game());
    // println!("{}", fibonacci(30));
    // println!("{}", temp_converter());
    // christmas_song();
    // println!("{}", addition(10, 20));
    // println!("{}", subtraction(40, 20));
    // println!("{}", multiplication(10, 20));
    // println!("{}", division(100, 5));
}
