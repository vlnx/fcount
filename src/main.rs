#[macro_use]
mod macros;
mod counter;
mod options;

use std::env;
use std::fs;
use std::io;
use std::process;

use crate::counter::FileCounter;
use crate::options::Options;

fn main() -> io::Result<()> {
    let ops = Options::parse_arguments(
        // Skip the "fcount" at the start.
        env::args().skip(1).collect::<Vec<_>>(),
    )?;

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

pub fn print_usage_and_exit() {
    println!("Usage: fcount [options]... [directory]");
    println!("Count files, directories and sys");

    println!(
"
  -r\tenter folders (traverse directory recursively)
  -s\tdo not count symbolic links
  -d\tdo not count folders
  -f\tdo not count files

Examples:
  fcount -rs /my/directory\tTraverse '/my/directory' recursively and do not count symbolic links.
  fcount /my/directory\tCount all files, folders and symbolic links in this folder, without traversing sub folders.
"   );

    process::exit(0);
}
