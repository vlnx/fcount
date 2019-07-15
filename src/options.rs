use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug, Default)]
#[structopt(name = "fcount")]
/// Count files, folders and symbolic links in a folder.
pub struct Options {
    /// The starting directory. If empty, searches current directory.
    #[structopt(name = "directory", default_value = ".", parse(from_os_str))]
    pub dir: PathBuf,
    /// Enter folders (traverse directory recursively)
    #[structopt(short = "r")]
    pub recursive: bool,
    /// Get total size of files (use -r to get size of folders too). -u for usage.
    #[structopt(short = "u", long = "--size")]
    pub get_size: bool,
    /// Do not count symbolic links
    #[structopt(short = "s")]
    pub no_count_sym_links: bool,
    /// Do not count folders
    #[structopt(short = "d")]
    pub no_count_folders: bool,
    /// Do not count files
    #[structopt(short = "f")]
    pub no_count_files: bool,
    /// Show numbers only, seperated by new lines, in order: files, folders, symbolic links
    #[structopt(short = "n", long = "--numbers")]
    pub numbers_only: bool,
}