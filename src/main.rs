mod day_three;
mod day_two;

fn main() {
    gm_to_rust();
    // day_two::main();
    // day_three::main();
}

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
