use std::time::{Instant};
use std::thread;
use std::time::Duration;

// As funções podem ser definidas antes ou depois da função principal

fn pulalinha(){
	println!("");
	
}

fn dormir() {
	println!("Início do programa.");

    // Pausa a execução por 2 segundos
    let duracao = Duration::from_secs(2);
    thread::sleep(duracao);

    println!("Depois de dormir por 2 segundos.");

    // Você também pode usar fracionários, por exemplo, 1.5 segundos
    let duracao_fracionada = Duration::from_secs_f64(1.5);
    thread::sleep(duracao_fracionada);

    println!("Depois de dormir por 1.5 segundos.");	
}

fn condicionais(){
	let idade:u8 = 41;
	let responsavel_autorizou:bool = true;
	let eh_maior = idade >= 18;
	
	if eh_maior || idade > 16 && responsavel_autorizou == true {
		println!("Pode entrar na balada.");
	} else {
		println!("Não pode entrar na balada.");
	}
	
	let condicao = if eh_maior {"maior"} else  {"menor"};
	
	println!("É {} de idade.", condicao);
	
	println!("Match Statement");
	
	let linguagem = "Kotlin";
	let proposito = match linguagem {
		"PHP" => "Web",
		"Kotlin"  => "Android",
		"Python" => "Data Science",
		_ => "Desconhecido"
	};
	
	println!("O propósito de {} é {}.", linguagem, proposito);
}

fn repeticoes() {
	let mut multiplicador:u8 = 3;
	let mut contador = 0;
	
	while contador < 10 {
		contador += 1;
		let resultado = multiplicador *  contador;
		
		if contador == 3 {
			continue;			
		}
		
		println!("{} x {} = {}", multiplicador, contador, resultado);		
	}
	
	pulalinha();
	
	contador = 0;
	multiplicador = 4;
	loop {
		contador += 1;
		println!("{} x {} = {}", multiplicador, contador, multiplicador *  contador);
		
		if contador == 10 {
			break;
		}
	}
	
	pulalinha();
	
	multiplicador = 5;
	for i in 1..11 {
		println!("{} x {} = {}", multiplicador, i, multiplicador * i);
	}
}

fn main(){
	
	// Registre o tempo de início
	let start_time = Instant::now();
	
	// Funções
	dormir();
	condicionais();
	pulalinha();
	repeticoes();
	pulalinha();
	
	
	// Registre o tempo de término
    let end_time = Instant::now();

    // Calcule a diferença de tempo
    let elapsed_time = end_time - start_time;

    // Converta o tempo para uma representação de ponto flutuante em segundos
    let elapsed_secs = elapsed_time.as_secs_f64();
	
	// Imprima o tempo decorrido
    println!("Tempo decorrido: {} segundos", elapsed_secs);
}