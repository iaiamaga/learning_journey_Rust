use std::io;

fn calcular_media(notas: &[f64]) -> f64 {
  let soma: f64 = notas.iter().sum();
  soma / notas.len() as f64
}

fn encontrar_maior_menor(notas: &[f64]) -> (f64, f64) {
   let maior = notas.iter().cloned().fold(f64::MIN, f64::max);
   let menor = notas.iter().cloned().fold(f64::MAX, f64::min);
   (maior, menor)
}

fn classificar_notas(notas: &[f64]) -> (Vec<f64>, Vec<f64>) {
   let aprovadas: Vec<f64> = notas.iter().cloned().filter(|&n| n >= 5.0).collect();
   let reprovadas: Vec<f64> = notas.iter().cloned().filter(|&n| n < 5.0).collect();
   (aprovadas, reprovadas)  
}

fn main() {
   let mut notas: Vec<f64> = Vec::new();

   println!("Digite 10 notas:");

   for i in 1..=10 { 
      let mut entrada = String::new();
      println!("Nota {}:", i);
      io::stdin().read_line(&mut entrada).expect("Erro (vc) ao ler entrada");
      let nota: f64 = entrada.trim().parse().expect("Digite um numero válido aiiii po");
      notas.push(nota);
   }

   let media = calcular_media(&notas);
   let (maior, menor) = encontrar_maior_menor(&notas);
   let (aprovadas, reprovadas) = classificar_notas(&notas);

   println!("\n### RESULTADOS ###");
   println!("Média das notas: {:.2}", media);
   println!("Maior nota: {:.2}", maior);
   println!("Menor nota: {:.2}", menor);
   println!("Notas aprovadas: ({}) : {:?}", aprovadas.len(), aprovadas);
   println!("Notas reprovadas: ({}) : {:?}", reprovadas.len(), reprovadas);

}
