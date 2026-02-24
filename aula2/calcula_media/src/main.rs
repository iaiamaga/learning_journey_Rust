fn main() {
    let numbers = [7.5, 8.0, 9.0];
    let sum: f64 = numbers.iter().sum();
    let average = sum / numbers.len() as f64;

    println!("A média dos números é: {:.2}", average);
}
