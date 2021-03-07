use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::io::{Error, ErrorKind, Result};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
pub struct Backup {
    input: String,
    output: String,
}
impl Backup {
    fn create_archive(&self) -> std::io::Result<File> {        
        return File::create(format!("{}/backup-{}.tar", &self.output, get_epoch_ms().to_string()));
    }
    pub fn new(input: &Path, destination: &Path) -> Option<Backup> {
        if input.exists() && destination.exists() {
            let input_str = input.to_str()?.to_string();
            let output_str = destination.to_str()?.to_string();
            return Some(Backup {
                input: input_str,
                output: output_str,
            });

        } else {
            return None;
        }
    }
    pub fn start(&self) -> Result<BackupResult> {
        let start_time_ms = get_epoch_ms();
        let tar_gz = &self.create_archive()?;
        let enc = GzEncoder::new(tar_gz, Compression::default());
        let mut tar = tar::Builder::new(enc);
        match tar.append_dir_all("backup", &self.input) {
            Ok(_) => {
                tar.finish()?;
                let bcp_stats = BackupResult {
                    duration_ms: get_epoch_ms() - start_time_ms,
                    start_time: start_time_ms
                };
                return Ok(bcp_stats);
            }
            Err(_e) => {
                return Err(Error::new(
                    ErrorKind::Other,
                    "Backup failed. Failed to copy file",
                ));
            }
        };
    }
}

fn get_epoch_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}
#[derive(Debug, PartialEq)]
pub struct BackupResult {
    duration_ms: u128,
    start_time: u128,

}
impl std::fmt::Display for BackupResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(start time : {}, duration : {}ms)", self.start_time , self.duration_ms)
    }
}
