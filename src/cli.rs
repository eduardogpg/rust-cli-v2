
fn demo() {
    println!("{:?}", std::env::args());

    let arguments: Vec<String> = std::env::args().collect();

    assert!(arguments.len, 3);

    for argument in arguments.iter() {
        println!("!--> {}", argument)
    }

    if arguments.len() >= 1 {
        println!("First element: {}", arguments[0]);
        println!("Second element: {}", arguments[1]);
        println!("Third element: {}", arguments[2]);
    }

    let first_element: &String = &arguments[0];
    println!(">>> {}", first_element);

    let second_argument: &String = &arguments[1];

    if second_argument == "Eduardo" {
        println!("El Segundo argumento es Eduardo!!!");
    }

    println!("Nombre completo: {} - {}", first_element, second_argument);

}
