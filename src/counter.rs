use std::fmt;
use std::process;
use std::fs;
use std::path::PathBuf;
use std::io::ErrorKind;

use crate::options::Options;

pub struct FileCounter {
    file_count: usize,
    dir_count: usize,
    sym_link_count: usize,
    current_path: PathBuf,
    ops: Options,
}

impl Default for FileCounter {
    fn default() -> FileCounter {
        FileCounter {
            file_count: 0,
            dir_count: 0,
            sym_link_count: 0,
            current_path: PathBuf::new(),
            ops: Options::default(),
        }
    }
}

impl FileCounter {
    pub fn new(ops: Options) -> FileCounter {
        FileCounter {
            current_path: ops.dir.clone(),
            ops,
            ..Default::default()
        }
    }

    pub fn get_file_and_dir_count(&mut self) {
        // If there was an error, if it was a permission erorr then just tell
        // the user and contine.
        match self.current_path.read_dir() {
            Ok(path_read) => {
                for sub in path_read {
                    let pathbuf = sub.unwrap().path();
                    let f_type = fs::symlink_metadata(pathbuf.as_path()).unwrap().file_type();

                    if f_type.is_symlink() {
                        self.sym_link_count += 1;
                    } else if f_type.is_dir() {
                        self.dir_count += 1;
                        if self.ops.recursive {
                            self.current_path = pathbuf;
                            // Traverse the directory
                            self.get_file_and_dir_count();
                        }
                    } else {
                        self.file_count += 1;
                    }
                }
            },
            Err(e) => {
                if e.kind() == ErrorKind::PermissionDenied {
                    eprintln!("filec: {}: Permission Denied", self.current_path.display());
                } else {
                    eprintln!("filec: {}: {}", self.current_path.display(), e);
                    process::exit(e.raw_os_error().unwrap());
                }
            }
        }
    }
}

// For outputting result.
impl fmt::Display for FileCounter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        if self.ops.count_sym_links {
            write!(f, "Files: {}\nFolders: {}\nSymbolic Links: {}", self.file_count, self.dir_count, self.sym_link_count)
        } else if self.ops.count_folders {
            write!(f, "Files: {}\nFolders: {}", self.file_count, self.dir_count)
        } else {
            write!(f, "Files: {}", self.file_count)
        }
    }
}