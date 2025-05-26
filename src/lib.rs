use clap::{Arg, Command};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    pub files: Vec<String>,
    pub number_lines: bool,
    pub number_nonblank_lines: bool,
    pub double_space: bool,
    pub triple_space: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("catx")
        .version("1.0")
        .disable_version_flag(true) // ← 自動生成される --version を無効化！
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust cat clone")
        .arg(
            Arg::new("version")
                .short('v')
                .long("version")
                .help("Print version information")
                .action(clap::ArgAction::Version),
        )
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .help("Input file(s)")
                .num_args(1..) // 1つ以上の引数を受け取る（複数対応）
                .default_value("-"),
        )
        .arg(
            Arg::new("number")
                .short('n')
                .long("number")
                .help("Number all output lines")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("number-nonblank")
                .short('b')
                .long("number-nonblank")
                .help("Number nonempty output lines")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("double_space")
                .short('2')
                .help("Double space output")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("triple_space")
                .short('3')
                .help("Triple space output")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    Ok(Config {
        files: matches
            .get_many::<String>("files")
            .unwrap()
            .cloned()
            .collect(),
        number_lines: matches.get_flag("number"),
        number_nonblank_lines: matches.get_flag("number-nonblank"),
        double_space: matches.get_flag("double_space"),
        triple_space: matches.get_flag("triple_space"),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(reader) => {
                let mut line_number = 0;

                for line_result in reader.lines() {
                    let line = line_result?;

                    if config.number_nonblank_lines && line.trim().is_empty() {
                        println!(); // 空行だけなら何もせず表示
                        continue;
                    }

                    if config.number_nonblank_lines || config.number_lines {
                        line_number += 1;
                        println!("{:>6}\t{}", line_number, line);
                    } else {
                        println!("{}", line);
                    }

                    // ダブル・トリプルスペース対応
                    if config.triple_space {
                        println!();
                        println!();
                    } else if config.double_space {
                        println!();
                    }
                }
            }
        }
    }
    Ok(())
}

pub fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}





