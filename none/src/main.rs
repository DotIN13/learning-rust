fn main() {
    let ale = Some(3);
    match ale {
        Some(num) => println!("{num}"),
        _ => ()
    }
    println!("{}", ale.unwrap_or(4));
}
