fn main() {
    let words = vec!["apple", "banana", "cherry", "date", "fig"];
    let result: Vec<_> = words
        .iter()
        .enumerate()
        .filter(|(i, _)| i % 2 == 0)
        .map(|(i, w)| format!("{}: {}", i + 1, w.to_uppercase()))
        .collect();
    
    println!("Result: {:?}", result);
}
