use std::fmt;
use std::fs;
use std::io::{self, ErrorKind};
use std::path::PathBuf;

use crate::options::Options;

#[derive(Default)]
pub struct FileCounter {
    file_count: usize,
    folder_count: usize,
    sym_link_count: usize,
    total_size: u64,
    current_path: PathBuf,
    ops: Options,
}

impl FileCounter {
    pub fn new(ops: Options) -> FileCounter {
        FileCounter {
            current_path: ops.dir.clone(),
            ops,
            ..Default::default()
        }
    }

    pub fn get_file_and_folder_count(&mut self) -> io::Result<()> {
        // If there was an error, if it was a permission erorr then just tell
        // the user and contine.
        match self.current_path.read_dir() {
            Ok(read_dir) => {
                for sub in read_dir {
                    let pathbuf = sub?.path();
                    let metadata = fs::symlink_metadata(pathbuf.as_path())?;
                    let f_type = metadata.file_type();

                    if f_type.is_symlink() {
                        self.sym_link_count += 1;
                    } else if f_type.is_dir() {
                        self.folder_count += 1;
                        
                        if self.ops.recursive {
                            self.current_path = pathbuf;
                            // Traverse the directory
                            self.get_file_and_folder_count()?;
                        }
                    } else {
                        self.file_count += 1;
                        if self.ops.get_size {
                            self.total_size += metadata.len();
                        }
                    }
                }
            },
            Err(e) => {
                if e.kind() == ErrorKind::PermissionDenied {
                    // Do not want to return from function if it is just a permission error.
                    eprintln!("fcount: {}: Permission denied.", self.current_path.display());
                } else {
                    return Err(e)
                }
            }
        }

        Ok(())
    }
}

// For outputting result.
impl fmt::Display for FileCounter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut needs_newline = false;

        let mut new_line = |f: &mut fmt::Formatter| {
            if needs_newline {
                write!(f, "\n")
            } else {
                needs_newline = true;
                Ok(())
            }
        };

        let entries = [
            (!self.ops.no_count_files, "Files", self.file_count),
            (!self.ops.no_count_folders, "Folders", self.folder_count),
            (!self.ops.no_count_sym_links, "Symbolic Links", self.sym_link_count),
        ];

        for (_, name, val) in entries.iter().filter(|(cond, _, _)| *cond) {
            new_line(f)?;
            if !self.ops.numbers_only {
                write!(f, "{}: ", name)?;
            }
            write!(f, "{}", val)?;
        }

        if self.ops.get_size {
            write!(f, "\nTotal Size: {}", get_display_for_size(self.total_size))?;
        }

        Ok(())
    }
}

fn get_display_for_size(size: u64) -> String {
    let mut size_repr = size as f64;
    let mut division_count: u8 = 0;

    while size_repr > 1024.0 {
        size_repr /= 1024.0;
        division_count += 1;
    }

    match division_count {
        1 => format!("{:.3} KiB", size_repr),
        2 => format!("{:.3} MiB", size_repr),
        3 => format!("{:.3} GiB", size_repr),
        4 => format!("{:.3} TiB", size_repr),
        5 => format!("{:.3} PiB", size_repr),
        _ => format!("{} bytes", size)
    }
}