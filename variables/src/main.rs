fn main() {
    let mut x = 5;
    println!("El valor de x es {x}");
    x = 6;
    println!("El valor de x es {x}");

    let y = 23;
    let y = y + 3;
    {
        let y = y + 4;
        println!("El valor de y el el primer alcance(?) es {y}");
    }
    println!("El valor de y es {y}");
    

    let spaces = "   ";
    let spaces = spaces.len();
    print!("El numero de espacios es {spaces}");

}
