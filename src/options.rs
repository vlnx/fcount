use std::io;
use std::env;
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
    pub fn parse_arguments() -> io::Result<Options> {
        let mut ops = Options::default();
        ops.args = env::args().collect();

        if ops.args.len() < 2 {
            error_message!(1, "Invalid number of arguments given: {}, expected at least 2", ops.args.len());
        }

        for arg in ops.args.iter() {
            let ch = arg.chars();
            let full = ch.as_str();
            let chars = ch.collect::<Vec<_>>().into_boxed_slice();

            if chars[0] == '-' {
                if chars[1] == '-' {
                    match full {
                        "--help" => {    // Only one '-' will be removed at this point
                            crate::print_usage_and_exit()
                        },
                        _ => (),
                    }
                } else {
                    for c in chars.iter().skip(1) {
                        match c {
                            'r' => ops.recursive = true,
                            's' => ops.count_sym_links = false,
                            'd' => ops.count_folders = false,
                            'f' => ops.count_files = false,
                            x => { error_message!(1, "Invalid option -- '{}'", x); },
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
            count_folders: true,
            count_sym_links: true,
            count_files: true,
        }
    }
}