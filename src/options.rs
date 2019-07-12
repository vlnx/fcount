use std::io;
use std::path::{Path, PathBuf};
use std::process;

pub struct Options {
    args: Vec<String>,
    pub dir: PathBuf,
    pub recursive: bool,
    pub no_count_sym_links: bool,
    pub no_count_folders: bool,
    pub no_count_files: bool,
}

impl Options {
    pub fn parse_arguments(args: Vec<String>) -> io::Result<Options> {
        let mut ops = Options::default();
        ops.args = args;

        if ops.args.len() < 2 {
            error_message!(
                1,
                "Invalid number of arguments given: {}, expected at least 2",
                ops.args.len()
            );
        }

        for arg in ops.args.iter() {
            match arg.as_str() {
                "--help" => crate::print_usage_and_exit(),
                _ => (),
            }

            let mut chars = arg.chars();
            // If first character is '-', then it is a flag.
            if chars.next().unwrap() == '-' {
                for c in chars {
                    // '-' is removed by chars.next()
                    match c {
                        'r' => ops.recursive = true,
                        's' => ops.no_count_sym_links = true,
                        'd' => ops.no_count_folders = true,
                        'f' => ops.no_count_files = true,
                        x => {
                            error_message!(1, "Invalid option -- '{}'", x);
                        }
                    }
                }
            } else {
                ops.dir = Path::new(arg).to_path_buf();
            }
        }

        Ok(ops)
    }
}

impl Default for Options {
    fn default() -> Options {
        Options {
            args: vec![],
            dir: Path::new("").to_path_buf(),
            recursive: false,
            no_count_folders: false,
            no_count_sym_links: false,
            no_count_files: false,
        }
    }
}
