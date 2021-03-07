use bcp::*;
use fs::File;
use std::fs;
use core::panic;
use std::path::Path;

#[cfg(test)]
#[test]
fn backup_test() {
    let input = Path::new("./testFolder");
    create_test_input(input);
    let output = Path::new("./testOutput");
    create_test_dirs( output);

    let backup = Backup::new(input, output).unwrap();
    let _result = match backup.start() {
        Ok(r) => r,
        Err(e) => {
            panic!("Failed test {}", e)
        }
    };
    let mut archive_files = fs::read_dir(output).unwrap().map(|x| x.unwrap().path().display().to_string()).collect::<Vec<String>>();
    //the output folder should contain only the backup archive
    assert_eq!(archive_files.len(), 1);

    let archive_path = archive_files.pop().unwrap();
    //the backup archive file name start with backup
    assert_eq!(true, archive_path.starts_with("./testOutput/backup-"));
    
    clean_test_dirs(input);
    
    clean_test_dirs(output);

}

fn create_test_dirs<P: AsRef<Path>>(input: P){

    fs::create_dir_all(input).unwrap();
}

fn create_test_input(input: &Path) {
    create_test_dirs(input);
    let input_str = input.display().to_string();    
    File::create( format!("{}/test.txt", input_str)).unwrap();
    create_test_dirs(format!("{}/subfolder", input_str))
}

fn clean_test_dirs(input: &Path) {
    fs::remove_dir_all(input).unwrap();
}
