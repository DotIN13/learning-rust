use std::io;

fn main() {
    println!("Please type the index of the desired fibonacci number (index starting from 0).");

    let mut fib_index = String::new();
    io::stdin()
        .read_line(&mut fib_index)
        .expect("Please input a number greater than or equal to 0.");

    let fib_index: u32 = fib_index.trim().parse().expect("Please input a number greater than or equal to 0.");

    if fib_index == 0 || fib_index == 1 {
        println!("{}", fib_index);
        return;
    }

    let mut counter = 0;
    let mut first: u64 = 0;
    let mut second: u64 = 1;
    let mut sum: u64;
    while counter < fib_index - 1 {
         sum = first + second;
         first = second;
         second = sum;
         counter += 1;
    }
    println!("{}", second);
}
