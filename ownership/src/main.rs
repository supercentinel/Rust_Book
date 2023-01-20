
fn main() {

    let str = "ASFASF   ";

    let mut str_m = String::from("ASDLFFAS sdg asdff");

    str_m.push_str(str);
    
    

    println!("{str}");
    println!("{}", str_m.trim());

    let s1 = String::from("hello");
    let _s2 = s1.clone();

    println!("{}, world!", s1);

    let s3 = primera_palabra(&str_m);
    let s4 = segunda_palabra(&str_m);

    println!("primera palabra: {}", s3);
    println!("segunda palabra: {}", s4);
}


fn primera_palabra(s: &String) -> &str 
{
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate()
    {
        if item == b' '
        {
            return &s[0..i];
        }
    }

    &s[..]
}

fn segunda_palabra(s: &str) -> &str
{
    let bytes = s.as_bytes();
    let mut space1: bool = false;
    let mut space_p: usize = 0;

    for (i, &item) in bytes.iter().enumerate()
    {
        if item == b' '
        {
            if space1 == true
            {
                return &s[space_p..i];
            }else
            {
                space_p = i+1;
            }

            space1 = true;
        }

    }
    
    &s[..]
}