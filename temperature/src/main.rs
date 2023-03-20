use std::io;

fn main() {
    println!("The temperature you want to convert is in [1] Celsius; [2] Fahrenheit:");
    let mut temp_type = String::new();
    io::stdin()
        .read_line(&mut temp_type)
        .expect("Please input a single-digit choice.");

    println!("Please type the temperature:");
    let mut temp = String::new();
    io::stdin()
        .read_line(&mut temp)
        .expect("Please input a number");

    let temp: f32 = temp.trim().parse().expect("Please input a number for the temperature");

    if temp_type.trim() == "1" {
        let temp: f32 = temp * 1.8 + 32.0;
        println!("The temperature is {temp} Fahrenheit.");
    } else {
        let temp: f32 = (temp - 32.0) / 1.8;
        println!("The temperature is {temp} Celsius.");
    }
}
