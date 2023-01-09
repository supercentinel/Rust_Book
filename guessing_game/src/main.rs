use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Adivina el número!");

    
    let rando = rand::thread_rng().gen_range(1..=100);
    
    println!("El numero aleatorio es: {rando}");
    
    println!("Adivina");
    
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Falló leer la linea");

    let guess: u32 = guess.trim().parse().expect("Introduce un número");

    println!("Tu respuesta: {guess} ");

    match guess.cmp(&rando) {
        Ordering::Less => println!("Muy pequeño!"),
        Ordering::Greater => println!("Muy grande!"),
        Ordering::Equal => println!("Adivinaste!"),
    }
}
