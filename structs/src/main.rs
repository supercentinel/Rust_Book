struct Usuario 
{
    nombre: String,
    numero_de_cuenta: u32,
    vivo: bool,

}

fn crear_usuario(nombre: String, numero_de_cuenta: u32) -> Usuario
{
    Usuario { nombre,
    numero_de_cuenta,
    vivo: true,
    }

}

fn print_usuario(usuario: &Usuario)
{
    println!("Nombre: {}", usuario.nombre);
    println!("Número de cuenta: {}", usuario.numero_de_cuenta);
    println!("Vivo: {}", usuario.vivo);
}

fn main() {
    let usr_1 = crear_usuario(String::from("Pedro"), 1241241);
    let usr_2 = Usuario {
        numero_de_cuenta: 12341,
        nombre: String::from("asfaSF"),
        ..usr_1
    };


    print_usuario(&usr_1);
    print_usuario(&usr_2);


    println!("Hello, world!");
}
