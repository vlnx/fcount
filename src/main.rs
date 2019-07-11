mod options;
mod counter;

use std::fs;
use std::io;
use std::process;

use crate::options::Options;
use crate::counter::FileCounter;

fn main() -> io::Result<()> {
    let ops = Options::parse_arguments()?;

    let meta = fs::metadata(&ops.dir).unwrap_or_else(|err| {
        eprintln!("filec: {}: Could not open folder, {}", ops.dir.display(), err);
        process::exit(err.raw_os_error().unwrap());
    });

    if meta.is_file() {
        eprintln!("filec: {}: File given, expected directory.", ops.dir.display());
        process::exit(1);
    }

    let mut file_counter = FileCounter::new(ops);

    file_counter.get_file_and_dir_count();
    println!("{}", file_counter);

    Ok(())
}
