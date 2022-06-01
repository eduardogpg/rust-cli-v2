struct Config {
    program: String,
    path: String,
    file: String
}

fn parse_args(args: Vec<String>) -> Config {
    if args.len() < 3 {
        panic!("Se esperaban 3 parámetros.")
    }

    let program: String = args[0].clone();
    let path: String = args[1].clone();
    let file: String = args[2].clone();

    Config { program, path, file }
}

fn main() { 
    let args: Vec<String> = std::env::args().collect();
    let custome_config = parse_args(args);

    println!("{}", custome_config.program);
    println!("{}", custome_config.path);
    println!("{}", custome_config.file);

}