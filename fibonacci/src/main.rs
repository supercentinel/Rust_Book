use std::io;

fn main() {
    let mut counter = 1;
    let mut f_imo: f64 = 1.0;
    let mut f_i: f64 = 1.0;
    let mut f_ipo: f64;

    let mut n: String = String::new();

    println!("Introduce el enésimo término de la secuencia de Fibonacci");

    io::stdin().
    read_line(&mut n).
    expect("no se pudo leer la línea");

    let n: u32 = n.trim().parse().expect("La línea no es un número");

    println!("f_{counter}={f_imo}");
    counter += 1;
    println!("f_{counter}={f_i}");

    while counter < n+1
    {
        f_ipo = f_i + f_imo;

        f_imo = f_i;
        f_i = f_ipo;

        println!("f_{counter}={f_i}");

        counter = counter + 1;
    }
}
