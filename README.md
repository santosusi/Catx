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


