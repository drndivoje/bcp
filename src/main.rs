use clap::{App, Arg, SubCommand, AppSettings, crate_version};
use human_panic::setup_panic;
use std::env;
use std::path::PathBuf;
use bcp::*;


fn main() {
    setup_panic!();
    let matches = App::new("BCP is short for BaCkuP")
        .setting(AppSettings::ArgRequiredElseHelp)
        .version(crate_version!())
        .author("Ivan Doskovic")
        .about("Simple backup tool")
        .arg(Arg::with_name("v")
            .short("v")
            .multiple(true)
            .help("Sets the level of verbosity"))
        .subcommand(SubCommand::with_name("exec")
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
    if let Some(matches) = matches.subcommand_matches("exec") {
        let input_folder = matches.value_of("input").unwrap();
        let output_folder = matches.value_of("output").unwrap();
        let input = PathBuf::from(input_folder);
        let output = PathBuf::from(output_folder);
        let backup = Backup::new(input.as_path(), output.as_path()).unwrap();
        match backup.start() {
            Ok(_res) => {
                println!("Backup finished: {}", _res);
            },
            Err(_err) => {
                panic!("Backup failed.");
            }
        }
    }
}