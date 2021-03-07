# bcp - Simple CLI backup tool made in Rust

The main goal of this project is to learn Rust programming language. The project is simple CLI tool to backup folders into tar archive. 

To backup folder run

```bash
bcp run -i <folder_to_backup> -o <destination_folder>
```
The output of this commnad is the tar archive named like backuop_<start_time_ms>.tar where start_time_ms represents start time of the backup as epoch time.
