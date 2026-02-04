use minigrep::{Config, highlight_matches};
use std::borrow::Cow;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(args.into_iter()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

fn get_reader(filename: Option<&String>) -> Box<dyn BufRead> {
    match filename {
        Some(f) => {
            let file = File::open(f).unwrap_or_else(|err| {
                eprintln!("无法打开文件 {}: {}", f, err);
                std::process::exit(1);
            });
            // 文件需要 BufReader 来提供 lines() 方法和缓冲
            Box::new(BufReader::new(file))
        }
        None => {
            // 如果没有文件名，就读 stdin
            Box::new(BufReader::new(io::stdin()))
        }
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let reader = get_reader(config.file_path.as_ref());

    // 使用 Cow 避免不必要的 clone
    let query_to_check: Cow<str> = if config.ignore_case {
        Cow::Owned(config.query.to_lowercase())
    } else {
        Cow::Borrowed(&config.query)
    };

    for line_result in reader.lines() {
        let line = line_result?;

        // 使用 Cow 避免不必要的 clone
        let line_to_check: Cow<str> = if config.ignore_case {
            Cow::Owned(line.to_lowercase())
        } else {
            Cow::Borrowed(&line)
        };

        if line_to_check.contains(query_to_check.as_ref()) {
            println!(
                "{}",
                highlight_matches(&line, &line_to_check, &query_to_check)
            );
        }
    }

    Ok(())
}
