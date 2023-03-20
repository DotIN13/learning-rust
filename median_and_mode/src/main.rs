use std::collections::HashMap;

fn main() {
    let mut vec = vec![1, 5, 10, 2, 2, 2, 15, 15, 15];
    vec.sort();
    let len: usize = vec.len();
    let median = if len % 2 == 1 {
        vec[len / 2]
    } else {
        (vec[len / 2] + vec[len / 2 - 1]) / 2
    };

    let mut occurrences = HashMap::new();
    let mut max_count = 0;
    let mut number_with_max_count = Vec::new();
    for number in &vec {
        let count = occurrences.entry(number).or_insert(0);
        *count += 1;
        if *count > max_count {
            max_count = *count;
            number_with_max_count.clear();
        }
        if *count >= max_count {
            number_with_max_count.push(number);
        }
    }
    let mode: Vec<String> = number_with_max_count
        .into_iter()
        .map(|el| el.to_string())
        .collect();

    println!("Median is {}, mode is {}", median, mode.join(", "));
}
