extern crate structopt;

#[macro_use]
mod macros;
mod counter;
mod options;

use std::io;
use std::fs;

use structopt::StructOpt;

use crate::counter::FileCounter;
use crate::options::Options;

fn main() -> io::Result<()> {
    let ops = Options::from_args();

    if ops.no_count_files && ops.no_count_folders && ops.no_count_sym_links {
        error_message!(0);
    }
    let meta = fs::metadata(&ops.dir).unwrap_or_else(|err| {
        error_message!(
            err.raw_os_error().unwrap(),
            "{}: Could not open folder, {:?}",
            ops.dir.display(),
            err
        );
    });
    if meta.is_file() {
        error_message!(1, "{}: File given, expected directory.", ops.dir.display());
    }

    let mut file_counter = FileCounter::new(ops);
    file_counter.get_file_and_folder_count();

    println!("{}", file_counter);
    Ok(())
}