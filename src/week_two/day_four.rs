pub fn main() {
    println!("{}", convert_f_to_c(100.0));

    generate_fibonacci_num(10);
}

pub fn convert_f_to_c(f: f32) -> f32 {
    (f - 32.0) * 5.0 / 9.0
}

pub fn generate_fibonacci_num(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    let mut a = 0;
    let mut b = 1;
    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
        // println!("{n}: {b}");
    }

    b
}
