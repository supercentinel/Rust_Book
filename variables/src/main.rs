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

    let _guess: u32 = "42".parse().expect("no es un numero");

    let _int_tuple: (i8, u8,
                    i16, u16,
                    i32, u32,
                    i64, u64,
                    i128, u128,
                    isize, usize) = (-12, b'F', -0x13, 0x1c, -0o12, 0o34, -0b1101, 0b1101110, -1234124, 123124, 345609346, 346373890);

    let _f32_array: [f32; 7];
    let _f64_array: [f64; 8];


}
