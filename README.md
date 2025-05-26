📝 README.md 見本（完全版）
md
コピーする
編集する
# catx

A Rust-based extended version of the Unix `cat` command.  
This tool provides additional features such as line numbering, double/triple spacing, and robust standard input handling.

---

## ✨ Features

- ✅ Output contents of one or more files
- ✅ Read from standard input using `-`
- ✅ Line numbering for all lines (`-n`)
- ✅ Line numbering for nonblank lines only (`-b`)
- ✅ Double-space output (`-2`)
- ✅ Triple-space output (`-3`)
- ✅ Error handling using `anyhow`
- ✅ Clean command-line interface via `clap`

---

## 🚀 Usage

```bash
catx [OPTIONS] [FILE]...

# Examples:

catx file.txt
catx -n file.txt
catx -b -             # Read from stdin and number nonblank lines
catx -2 file.txt
catx -n -2 file.txt
catx -3 file1.txt - file2.txt
If no filename is given, or - is used, catx will read from standard input.

🔧 Build
This project requires Rust (1.70+ recommended):


cargo build --release
The executable will be generated in target/release/catx.

🧪 Run Tests
bash
コピーする
編集する
cargo test
Tests include:

Line numbering

Double spacing

Error on missing files

📁 Project Structure
catx/
├── src/
│   └── main.rs
├── Cargo.toml
├── Cargo.lock
└── README.md
👤 Author
安陵 哲志（Tetsushi Anyo）
Developed as a submission for System Programming I (2025)
Rust + CLI + UNIX Tooling
