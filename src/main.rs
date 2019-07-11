#[macro_use] mod macros;
mod options;
mod counter;

use std::fs;
use std::io;
use std::process;

use crate::options::Options;
use crate::counter::FileCounter;

fn main() -> io::Result<()> {
    let ops = Options::parse_arguments()?;

    if !ops.count_files && !ops.count_folders && !ops.count_sym_links {
        error_message!(0);
    }

    let meta = fs::metadata(&ops.dir).unwrap_or_else(|err| {
        error_message!(err.raw_os_error().unwrap(), "{}: Could not open folder, {:?}", ops.dir.display(), err);
    });

    if meta.is_file() {
        error_message!(1, "{}: File given, expected directory.", ops.dir.display());
    }

    let mut file_counter = FileCounter::new(ops);

    file_counter.get_file_and_folder_count();
    println!("{}", file_counter);

    Ok(())
}


pub fn print_usage_and_exit() {
    println!("Usage: filec [options]... [directory]");
    println!("Count files, directories and sys");

    process::exit(0);
}