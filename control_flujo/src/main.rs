fn main() {
    let cosa = 43;
    let mut i = 0;
    let es = [20, 12, 23, 434, 43];

    if cosa <= 20
    {
        println!("El valor de la cosa es de {cosa}");

    }
    else
    {
        println!("El valor de la cosa es mayor a 20");
    }

    let danas = if cosa > 6 {18.3} else {123.3};

    println!("El valor de danas es {danas}");

    loop {
        i = i+1;

        println!("i = {i}");

        if i == 15 {
            break;
        }

    }

    for ess in es.iter().rev() {
        println!("El valor es es {ess}");
    }

    'clna: loop {
        i = i + 2;
        loop {
            i = i + 1;
            if i > 50
            {
                break 'clna println!("i = {i}");
            }
        }
    }

    while i != 100 {
        i = i + 1;

    }
    println!("i = {i}")

}
