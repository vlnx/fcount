mod counter;
mod options;

use std::io::{self, ErrorKind};
use std::fs;
use std::process;

use structopt::StructOpt;

use crate::counter::FileCounter;
use crate::options::Options;

fn run() -> io::Result<()> {
    let ops = Options::from_args();

    if ops.no_count_files && ops.no_count_folders && ops.no_count_sym_links {
        process::exit(0);
    }
    let meta = fs::metadata(&ops.dir)?;

    if meta.is_file() {
        return Err(io::Error::new(ErrorKind::Other, "Cheeseburger"))
    }

    let mut file_counter = FileCounter::new(ops);
    
    file_counter.get_file_and_folder_count()?;
    println!("{}", file_counter);

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