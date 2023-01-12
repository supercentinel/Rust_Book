/*
    idk
*/
fn another_function(x: i32)
{
    println!("El valor de x es {x}");
}

// Returns the square of a number
fn square(x: i32) -> i32 {
    x * x 
}

fn main() {
    println!("Hello, world!");

    let y = 
    {
        let x = 34;
        x + 2
    };

    let mut x = 7;
    x = square(x);

    another_function(-0x1C);
    println!("El valor de y es de {y}");
    println!("El valor cuadrado de x es {x}")
}
