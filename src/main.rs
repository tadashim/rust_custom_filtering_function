fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let sum: i32 = numbers.iter().fold(0, |acc, x| acc + x);

    println!("The sum of the numbers is: {}", sum);
}
