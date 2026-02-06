# minigrep

A minimal `grep` implementation — a learning exercise from [The Rust Programming Language](https://doc.rust-lang.org/book/ch12-00-an-io-project.html) Chapter 12.

## Features

- 🔍 Search for text patterns in files
- 🎨 **Highlight** matched keywords in red
- 📄 Support reading from **stdin** (pipe-friendly)
- 🔠 Case-insensitive search via `IGNORE_CASE` env var

## Usage

```bash
# Basic search
cargo run -- <query> <file>

# Example
cargo run -- to poem.txt

# Case-insensitive search
IGNORE_CASE=1 cargo run -- to poem.txt

# Read from stdin
cat poem.txt | cargo run -- to
```

## Example Output
```bash
Are you nobody, too?
How dreary to be somebody!
```
