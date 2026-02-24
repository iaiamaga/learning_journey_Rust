use regex::Regex;

fn main() {
    // Definindo um padrão de e-mail válido
    let email_pattern = r"^[\w\.-]+@[\w\.-]+\.[\w]{2,}$";
    let email_regex = Regex::new(email_pattern).unwrap();

    // Define os e-mails para validar
    let emails = ["test@example.com", "invalid-email", "user@domain.co"];

    // Validando e exibindo os resultados
    for email in emails.iter() {
        if email_regex.is_match(email) {
            println!("{} é um e-mail válido.", email);
        } else {
            println!("{} é um e-mail inválido.", email);
        }
    }
}
