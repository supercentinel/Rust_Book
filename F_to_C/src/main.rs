use std::io;

fn f_to_c(temp_f: f64) -> f64
{
    return (temp_f - 32.00)/(1.8);
}

fn main() {
    
    let mut tempf: String = String::new();
    
    println!("\t\tFarenheit a Celcius");
    println!("Introduce el valor en Farenheit");
    
    io::stdin().
    read_line(&mut tempf).
    expect("No se pudo leer la linea");
    
    let tempf: f64 = tempf.trim().parse().expect("La línea no es un número");

    let tempc = f_to_c(tempf);

    println!("El valor {tempf}F en Celcius es {tempc}C");

}
