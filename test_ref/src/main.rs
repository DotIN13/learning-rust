fn main() {
    let mut ale = 1;
    ale = 2;
    let yake;
    yake = &ale;
    println!("{}", yake);
    
    let mut chick = vec![1, 2];
    let muta = &mut chick;
    muta.push(ale);
    for (i, item) in muta.iter().enumerate() {
        println!("{}", *item);
    }
}
