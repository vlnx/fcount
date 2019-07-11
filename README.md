# fcount - File, folder and symbolic link counter

## Building

Build with
```bash
cargo build --release
```
And the executable will be in `target/release/fcount`.

## Usage

```bash
fcount [flags] [directory]
```

*(Arguments can be in any order.)*

And it should output:
```bash
Files: x
Folders: y
Symbolic Links: z
```

Flags include:

Flag | Description
--- | ---
**-r** | Traverse recursively (count in sub folders, and sub folders of sub folders...).
**-s** | **Do not** count symbolic links.
**-d** | **Do not** count folders.
**-f** | **Do not** count files.
**--help** | Display usage.

Most flags are on as default, as I figured most people would want all three pieces of information most of the time.

Flags can be chained together (unless verbose like `--help`), like this: `-rsd`, or done seperately: `-r -s -d`.

### Examples
```bash
fcount -rs /my/directory
```
Traverse '/my/directory' recursively and do not count symbolic links.

```bash
fcount /my/directory
```
Count all files, folders and symbolic links in this folder, without traversing sub folders.