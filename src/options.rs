use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "fcount")]
pub struct Options {
    /// The starting directory
    #[structopt(name = "directory", parse(from_os_str))]
    pub dir: PathBuf,
    /// Enter folders (traverse directory recursively)
    #[structopt(short = "r")]
    pub recursive: bool,
    /// Do not count symbolic links
    #[structopt(short = "s")]
    pub no_count_sym_links: bool,
    /// Do not count folders
    #[structopt(short = "d")]
    pub no_count_folders: bool,
    /// Do not count files
    #[structopt(short = "f")]
    pub no_count_files: bool,
    // Show numbers only
}

impl Default for Options {
    fn default() -> Options {
        Options {
            dir: Path::new("").to_path_buf(),
            recursive: false,
            no_count_folders: false,
            no_count_sym_links: false,
            no_count_files: false,
        }
    }
}