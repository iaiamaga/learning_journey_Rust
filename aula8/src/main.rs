struct Livro {
    titulo: String,
    autor: String,
    ano: u16,
}

impl Livro {
    // método para exibir informações do livro
    fn detalhes(&self) {
        println!("Título: {}, Autor: {}, Ano: {}", self.titulo, self.autor, self.ano);
    }
}


// função para imprimir todos os livros em um vetor 
fn listar_livros(livros: &[&Livro]) {
   for livro in livros{
       livro.detalhes();
   }
}


// função para filtrar livros dentro de um periodo usando slices
fn livros_por_periodos(livros: &[Livro], ano_inicial: u16, ano_final: u16) {
   println!("/nLivros publicados entre {} e {}:", ano_inicial, ano_final);
   let filtro: Vec<&Livro> = livros.iter()
       .filter(|&livro| livro.ano >= ano_inicial && livro.ano <= ano_final)
       .collect();

   listar_livros(&filtro);
}

fn main() {
   //criando um vetor de livros
   let biblioteca = vec![
        Livro { titulo: String::from("O Senhor dos Anéis"), autor: String::from("J.R.R. Tolkien"), ano: 1945 },
        Livro { titulo: String::from("1984"), autor: String::from("Geoge Orwell"), ano: 1949 },
        Livro { titulo: String::from("Duna"), autor: String::from("Frank Herbert"), ano: 1965 },
        Livro { titulo: String::from("Neuromancer"), autor: String::from("William Gibson"), ano: 1984 },
        Livro { titulo: String::from("Fundação"), autor: String::from("Isaac Asimov"), ano: 1951 }, 
   
    ];
   
    println!("Lista completa de livros:");
    // Convertendo todos os livros em referências para listar
    let referencias: Vec<&Livro> = biblioteca.iter().collect();
    listar_livros(&referencias);

    // filtrando livros publicados entre 1950 e 1970
    livros_por_periodos(&biblioteca, 1950, 1970);

}
