# File Count

## Building

Build with
```bash
cargo build --release
```
And the executable will be in `target/release/filec`.

## Usage

```bash
filec [directory] [flags]
```
*(Arguments can be in any order.)*

And it should output:
```bash
Files: x
Folders: y
Symbolic Links: z
```

Flags include:

**-r** : **Do not** traverse recursively (only count in current folder).\\
**-s** : **Do not** count symbolic links.\\
**-d** : **Do not** count folders.\\


Most flags are on as default, as I figured most people would wan't all three pieces of information most of the time.