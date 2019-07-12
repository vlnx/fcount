# fcount - File, folder and symbolic link counter

## Building

Build with
```bash
cargo build --release
```
And the executable will be in `target/release/fcount`.

## Usage

```bash
fcount [FLAGS] <directory>
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
**-h, --help** | Displays help.
**-f** | **Do not** count files.
**-d** | **Do not** count folders.
**-s** | **Do not** count symbolic links.
**-n** | Show numbers only, seperated by lines, in order: files, folders, symbolic links.
**-r** | Traverse recursively (count in sub folders, and sub folders of sub folders...).
**-V, --version** | Displays version information.

All counts are enabled as default, as I figured most people want all counts most of the time.

Flags can be chained together (unless verbose like `--help`), like this: `-rsd`, or done seperately: `-r -s -d`.

---
### Examples

#### Traverse '/my/directory' recursively and do not count symbolic links:
```bash
fcount -rs /my/directory
```
Output:
```bash
Files: x
Folders: y
```

#### Count all files, folders and symbolic links in this folder, without traversing sub folders:
```bash
fcount /my/directory
```
Output:
```bash
Files: x
Folders: y
Symbolic Links: z
```

#### Count all files and folders, without displaying 
```bash
fcount -rns /my/directory
```
Output:
```bash
x
y
```