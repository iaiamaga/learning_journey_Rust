fn main() {
  let a: i32 = 8;
  let b: i32 = 3;

  let soma = a + b;
  let subtracao = a - b;
  let produto = a * b;
  let divisao = if b != a { Some(a / b) } else { None };
  let maior = a > b;
  let soma_par = soma % 2 == 0;

println!("Resultados:");
println!("Soma: {}", soma);
println!("Subtração: {}", subtracao);
println!("Produto: {}", produto);

match divisao {
Some(valor) => println!("Divisão: {}", valor),
None => println!("Divisão:não é possível dividir por zero"),
}

println!("a é maior que b? {}", maior);
println!("A soma de a e b é par? {}", soma_par);

}
