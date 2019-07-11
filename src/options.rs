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
}

impl Options {
    pub fn parse_arguments() -> io::Result<Options> {
        let mut ops = Options::default();
        ops.args = env::args().collect();

        if ops.args.len() < 2 {
            eprintln!("filec: Invalid number of arguments given: {}, expected at least 2", ops.args.len());
            process::exit(1);
        }

        for arg in ops.args.iter() {
            let first = arg.chars().nth(0).unwrap();
            if first == '-' {
                for ch in arg.chars() {
                    match ch {
                        'r' => ops.recursive = false,
                        's' => ops.count_sym_links = false,
                        'd' => ops.count_folders = false,
                        _ => (),
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
            recursive: true,
            count_folders: true,
            count_sym_links: true,
        }
    }
}