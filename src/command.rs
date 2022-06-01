use clap::App;
use clap::Arg;

fn command() { 
    let arg1 = Arg::with_name("course")
                    .short("c")
                    .value_name("COURSE")
                    .help("Nombre del curso")
                    .required(true);

    let arg2 = Arg::with_name("video")
                    .short("v")
                    .value_name("VIDEO")
                    .help("Nombre del video")
                    .required(false);

    let matches = App::new("CódigoFacilito")
        .version("0.0.1")
        .author("Eduardo Ismael <eduardo@codigofacilito.com>")
        .about("Simple CLI")
        .arg(arg1)
        .arg(arg2)
        .get_matches();
    
    if matches.is_present("course") {
        
        let course_name = matches.value_of("course").unwrap();
        let video_name = matches.value_of("video").expect("Se espera el nombre del vídeo.");

        println!("Curso: {} - Video: {}",  course_name, video_name);
    }
}