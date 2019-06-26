use std::fs;
use std::io;
use std::path::Path;
use std::env;
use std::process;

fn get_file_and_dir_count(p: &Path, file_count: &mut usize, dir_count: &mut usize, sym_link_count: &mut usize) -> io::Result<()> {
    for sub in p.read_dir()? {
        let pathbuf = sub.unwrap().path();
        let f_type = fs::symlink_metadata(pathbuf.as_path())?.file_type();

        if f_type.is_symlink() {
            *sym_link_count += 1;
        } else if f_type.is_dir() {
            *dir_count += 1;
            get_file_and_dir_count(pathbuf.as_path(), file_count, dir_count, sym_link_count)?;
        } else {
            *file_count += 1;
        }
    }

    Ok(())
}


fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("filec: Invalid number of arguments given: {}", args.len());
        process::exit(1);
    }

    let dir = Path::new(args[1].as_str());

    let meta = fs::metadata(dir).unwrap_or_else(|err| {
        eprintln!("filec: Could not open folder: {}, {}", dir.display(), err);
        process::exit(1);
    });

    if meta.is_file() {
        eprintln!("filec: File given, expected directory.");
        process::exit(1);
    }

    let (mut file_count, mut dir_count, mut sym_link_count) = (0usize, 0usize, 0usize);

    get_file_and_dir_count(dir, &mut file_count, &mut dir_count, &mut sym_link_count).unwrap_or_else(|err| {
        eprintln!("filec: Could not get count: {}", err);
        process::exit(1);
    });

    println!("Files: {}\nFolders: {}\nSymbolic Links: {}", file_count, dir_count, sym_link_count);

    Ok(())
}