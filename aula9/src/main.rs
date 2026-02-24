use std::collections::HashMap;

fn criar_usuarios() -> HashMap<&'static str, &'static str> {
   let mut usuarios = HashMap::new();
   usuarios.insert("coelho cinza", "senhachurras");
   usuarios.insert("rudolfo","336699");
   usuarios.insert("luli", "euamochocolate");
   usuarios

}

fn autenticar(
    usuarios: &HashMap<&str, &str>,
    usuario: &str,
    senha: &str, 
) -> Result<String, String> {
    match usuarios.get(usuario) {
        Some(senha_correta) => {
            if senha == *senha_correta {
               Ok(format!("Login bem sucedido!Seja muito Bem-vind@, {}!", usuario))
            } else {
                 Err("Senha incorreta, tente loucamente-opa, não-, digo: novamente.".to_string()) 
              }
        }

        None => Err("FUDEU 404Usuario não encontrado. se alguém achá-lo diga para voltar pra casa".to_string()),   
    }

}


fn main() {
  let usuarios = criar_usuarios();
 
  println!("Digite seu nome de usuário:");
  let mut nome_usuario = String::new();
  std::io::stdin().read_line(&mut nome_usuario).unwrap();
  let nome_usuario = nome_usuario.trim().to_string();

  println!("Digite a senha: ");
  let mut senha = String::new();
  std::io::stdin().read_line(&mut senha).unwrap();
  let senha = senha.trim().to_string();

  match autenticar(&usuarios, &nome_usuario, &senha) {
      Ok(mensagem) => println!("{}", mensagem),
      Err(erro) => println!("Erro: {}", erro),
  }
}
