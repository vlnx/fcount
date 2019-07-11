use std::io;
use std::process;
use std::path::{Path, PathBuf};

pub struct Options {
    args: Vec<String>,
    pub dir: PathBuf,
    pub recursive: bool,
    pub count_folders: bool,
    pub count_sym_links: bool,
    pub count_files: bool,
}

impl Options {
    pub fn parse_arguments(args: Vec<String>) -> io::Result<Options> {
        let mut ops = Options::default();
        ops.args = args;

        if ops.args.len() < 2 {
            error_message!(1, "Invalid number of arguments given: {}, expected at least 2", ops.args.len());
        }

        for arg in ops.args.iter() {
            match arg.as_str() {
                "--help" => {
                    crate::print_usage_and_exit()
                },
                _ => (),
            }

            let mut chars = arg.chars();
            // If first character is '-', then it is a flag.
            if chars.next().unwrap() == '-' {
                for c in chars {    // '-' is removed by chars.next()
                    match c {
                        'r' => ops.recursive = true,
                        's' => ops.count_sym_links = false,
                        'd' => ops.count_folders = false,
                        'f' => ops.count_files = false,
                        x => { error_message!(1, "Invalid option -- '{}'", x); },
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
            count_folders: true,
            count_sym_links: true,
            count_files: true,
        }
    }
}