use clap::{Arg, App};
mod lib;

fn main() {

    let matches = App::new("Soustitle-rs")
        .version("0.1.0")
        .author("Muideen Lawal <muideen.lawal320@gmail.com>")
        .about("A simple (subtitle) .srt parser written in rust.")
        .arg(Arg::with_name("input")
                 .short('i')
                 .long("input")
                 .takes_value(true)
                 .help("Path to an .srt file"))
        .arg(Arg::with_name("output")
                 .short('o')
                 .long("output")
                 .takes_value(true)
                 .help("Path to save the output file"))
        .get_matches();

        let myfile = matches.value_of("input");
        match myfile {
            None => println!("You have to pass a valid subtitle file path."),
            Some(s) => {
                let result = lib::parse(s);
                println!("Parsed SRT: {:?}", result);
            }
        }

    //lib::write_data(result, "/data/output.csv", true).unwrap();

}