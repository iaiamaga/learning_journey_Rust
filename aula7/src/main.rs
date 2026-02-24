fn main() {

  let frase = String::from("Rust é Seguro e rápidamente rápido!");

// ownership: transfira a propriedade da frase para uma função e retorne o valor modificado 
  
  let frase_modificada = transfere_e_modifica(frase);
  
  println!("Frase modificada: {}", frase_modificada);




// Borrowing: crie uma nova string e use borrowing para exibi-la sem mover a propriedade. 

  let frase_borrow = String::from("Aprender e continuar são coisas incriveis!");
  
  exibir_frase(&frase_borrow);
  
  println!("Ainda posso usar frase_borrow: {}", frase_borrow);


  
// Slice: pegue a primeira palavra da string e exiba

  let primeira_palavra = obter_primeira_palavra(&frase_modificada);
  
  println!("Primeira palavra da frase modificada: {}", primeira_palavra);



// Função que recebe uma String e transfere a propriedade, modificando o valor

  fn transfere_e_modifica(mut texto: String) -> String {
  
  texto.push_str("Nunca pare de aprender!"); // add texto à string

  texto // retorna a string modificada, transferindo de volda a propriedade
  
  }

  

  fn exibir_frase(texto: &String) {
  
  println!("Frase recebida: {}", texto);
  
  }

  

  fn obter_primeira_palavra(s: &String) -> &str {
  
    let bytes = s.as_bytes();
   
     for (i, &item) in bytes.iter().enumerate() {
       if item == b' '{
         return &s[..i];
        }

   }
   
   &s[..] //retorna a string inteira se não houver espaço


 }

}
