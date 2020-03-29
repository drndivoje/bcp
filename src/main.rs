use clap::{App, Arg, SubCommand};
use human_panic::setup_panic;

mod bcp;


fn main() {
    setup_panic!();
    let matches = App::new("BCP is short for BaCkuP")
        .version("0.0.1")
        .author("Ivan Doskovic")
        .about("Simple backup tool")
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("FILE")
            .help("Sets a custom config file")
            .takes_value(true))
        .arg(Arg::with_name("v")
            .short("v")
            .multiple(true)
            .help("Sets the level of verbosity"))
        .subcommand(SubCommand::with_name("run")
            .about("executing backup of given folder")
            .version("1.3")
            .arg(Arg::with_name("input")
                .short("i")
                .help("the path to the folder to be backed up")
                .required(true)
                .takes_value(true))
            .arg(Arg::with_name("output")
                .short("o")
                .help("the path where data will be backed up")
                .required(true)
                .takes_value(true)))
        .get_matches();

    // TODO: setup configuration
    // let config = matches.value_of("config").unwrap_or("default.conf");

    if let Some(matches) = matches.subcommand_matches("run") {
        let input_folder = matches.value_of("input").unwrap();
        let output_folder = matches.value_of("output").unwrap();
        bcp::backup(input_folder, output_folder)
    }
}
