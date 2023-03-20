use std::io;

fn main() {
    println!("Please input a word to be converted into PIG LATIN!");
    let mut word = String::new();
    io::stdin().read_line(&mut word).expect("A word is expected.");
    
    let converted = word_to_pig_latin(word.trim().to_string());
    println!("{}", converted);
}

fn word_to_pig_latin(word: String) -> String {
    let (first_ch, rest) = match word.chars().next() {
        Some(ch) => word.split_at(ch.len_utf8()),
        None => word.split_at(0),
    };
    if ["a", "e", "i", "o", "u"].contains(&first_ch) {
        return word + "-hay";
    } else {
        return String::from(rest) + "-" + first_ch + "ay";
    };
}
