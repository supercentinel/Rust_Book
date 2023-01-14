
fn main() {

    let str = "ASFASF   ";

    let mut str_m = String::from("ASDLFFASF ");

    str_m.push_str(str);
    
    

    println!("{str}");
    println!("{}", str_m.trim())

}
