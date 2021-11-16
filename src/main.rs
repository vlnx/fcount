use std::env;
use std::fs;
use std::io::{self, ErrorKind};
use std::path::Path;
use std::process;

fn run() -> io::Result<()> {
    let current_path = Path::new("./").to_path_buf();
    if env::args().count() == 1 {
        let mut directory_count = 0usize;
        let mut file_count = 0usize;
        match current_path.read_dir() {
            Ok(read_dir) => {
                for sub in read_dir {
                    if fs::symlink_metadata(sub?.path().as_path())?
                        .file_type()
                        .is_dir()
                    {
                        directory_count += 1;
                    } else {
                        file_count += 1;
    }
    }
            }
            Err(e) => {
                if e.kind() == ErrorKind::PermissionDenied {
                    // Do not want to return from function if it is just a permission error.
                    eprintln!("fcount: Permission denied.");
                } else {
                    return Err(e);
                }
            }
        }
        println!("{} directories, {} files", directory_count, file_count);
    } else {
        // if called with any arguments at all, only count
        // everything, don't try to tell the type
        println!("{}", current_path.read_dir()?.count());
    }
    Ok(())
}

fn main() {
    match run() {
        Ok(_) => (),
        Err(e) => {
            eprintln!("fcount: error: {}", e);
            if let Some(os_error) = e.raw_os_error() {
                process::exit(os_error)
            } else {
                process::exit(1)
            }
        }
    }
}