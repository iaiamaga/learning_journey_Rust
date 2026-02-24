fn main() {
	const PI: f64 = 3.14159;
	let mut raio = 2.5; //mutávellll
	let area = PI * raio * raio;

	println!("A área do círculo com o raio {} é: {}",raio, area);
	
	raio = 3.0;
	let nova_area = PI * raio * raio; 
	println!("A nova área do circulo com o raio {} é: {}", raio, nova_area);

}
