use std::io;

fn main() {
  
   let mut input = String::new();
   println!("Digite um número");
   io::stdin().read_line(&mut input).expect("viixi, alguem vacilou; erro ao ler entrada");
   
  let number: i64 = input.trim().parse().expect("ai, namoral... insere um número válido");
  
  if number % 2 == 0 {
   println!("O número {} é par", number);
  } else {
   println!("O numero {} é impar", number);
  }

  println!("CONTANDO de 1 até {}", number);
  for i in 1..=number {
   println!("{}", i);
  } 
  println!();

  match number {
   1 => println!("vc escolheu o número 1, A mudança abrubta."),
   2..=10 => println!("VC escolheu um numéro ordináaaario"),
   11..=100 => println!("Esse é um numero considerávelmente grande num pique esconde"),
   101..=300 => println!("vc escolheu a quantidade de bolas de sorvete que vou querer"),
   _ => println!("nossa, vc escolheu um numero bigmonster ultra pro de grande."),

  }

}
