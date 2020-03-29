extern crate chrono;

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::Instant;

pub fn backup(path: &str, destination: &str) {
    if Path::new(path).exists() {
        let backup_folder = format!("{}/backup-{}", destination, chrono::offset::Local::now().to_string());
        let _bcp_folder = fs::create_dir(backup_folder.as_str());
        match _bcp_folder {
            Ok(_bcp_folder) => {
                println!("Starting backup");
                match copy(Path::new(path), Path::new(backup_folder.as_str())) {
                    Ok(_result) => {
                        _result.save(backup_folder.as_str()).expect("failed to save bcp.info file");
                    },
                    Err(_e) => {
                        println!("Backup failed. Failed to create backup folder.")
                    }
                }

            }
            Err(_e) => {
                println!("Backup failed. Failed to create backup folder.")
            }
        }
    } else {
        println!("Backup failed. Folder {} does not exist.", path)
    }
}

#[derive(Debug)]
struct BackupStatistic {
    duration_ms: u128,
    files_count: i32,
    start_time: Instant,
}

impl BackupStatistic {
    fn save(&self, path:&str) -> std::io::Result<()> {
        let mut file = fs::File::create(format!("{}/bcp.info", path))?;
        file.write_all( format!("{:?}", self).as_bytes())?;
        Ok(())
    }
    fn file_count(&mut self) {
        self.files_count += 1;
    }

    fn new() -> BackupStatistic {
        BackupStatistic{duration_ms: 0, files_count:0, start_time: Instant::now()}
    }

    fn stop(&mut self) {
        self.duration_ms = self.start_time.elapsed().as_millis();
    }

}


fn copy<U: AsRef<Path>, V: AsRef<Path>>(from: U, to: V) -> Result<BackupStatistic, std::io::Error> {
    let mut bcp_stats = BackupStatistic::new();
    let mut stack = Vec::new();
    stack.push(PathBuf::from(from.as_ref()));

    let output_root = PathBuf::from(to.as_ref());
    let input_root = PathBuf::from(from.as_ref()).components().count();

    while let Some(working_path) = stack.pop() {
        println!("process: {:?}", &working_path);

        // Generate a relative path
        let src: PathBuf = working_path.components().skip(input_root).collect();

        // Create a destination if missing
        let dest = if src.components().count() == 0 {
            output_root.clone()
        } else {
            output_root.join(&src)
        };
        if fs::metadata(&dest).is_err() {
            println!(" mkdir: {:?}", dest);
            fs::create_dir_all(&dest)?;
        }

        for entry in fs::read_dir(working_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else {
                match path.file_name() {
                    Some(filename) => {
                        let dest_path = dest.join(filename);
                        println!("  copy: {:?} -> {:?}", &path, &dest_path);
                        fs::copy(&path, &dest_path)?;
                        bcp_stats.file_count();
                    }
                    None => {
                        println!("failed: {:?}", path);
                    }
                }
            }
        }
    }

    bcp_stats.stop();
    Ok(bcp_stats)
}
