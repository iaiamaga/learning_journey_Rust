///
/// * Exemplo
///
/// resultado calculadora::soma(2, 3);
/// essert_eq!(resultado, 5);
///

fn soma(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let resultado = soma(10, 5);
    println!("Resultado da soma: {}", resultado);
}
