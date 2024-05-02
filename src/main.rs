use clap::{App, Arg};

fn main() {
    let matches = App::new("MyApp")
        .version("1.0")
        .author("Your Name")
        .about("Description of your program")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .value_name("FILE")
                .help("Sets the input file to use")
                .takes_value(true),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("FILE")
                .help("Sets the output file to use")
                .takes_value(true),
        )
        .get_matches();

    if let Some(input_file) = matches.value_of("input") {
        println!("Input file: {}", input_file);
    }

    if let Some(output_file) = matches.value_of("output") {
        println!("Output file: {}", output_file);
    }
}
