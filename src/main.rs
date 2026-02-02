use minigrep::{search, search_case_insensitive};
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
    // 1. 获取读取器（不管是文件还是管道，现在对我们来说都是 `reader`）
    let reader = get_reader(config.file_path.as_ref());
    let mut query = config.query;

    if config.ignore_case {
        query = query.to_lowercase();
    }

    // 2. 逐行读取并搜索
    // 这里我们直接在流上操作，这比 read_to_string 更节省内存！
    for line_result in reader.lines() {
        let mut line = line_result?; // 处理可能的 I/O 错误

        if config.ignore_case {
            line = line.to_lowercase();
        }

        // 调用你的搜索逻辑 (注意：如果你的 search 函数还是接收整个大字符串，可能需要改写成接收单行)
        if line.contains(&query) {
            println!("{}", line);
        }
    }

    Ok(())
}

struct Config {
    query: String,
    file_path: Option<String>,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next(); // 跳过程序名

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        // 尝试获取文件名，如果没有，那就是 None (代表要读 stdin)
        let file_path = args.next();

        // ... 环境变量处理 ignore_case ...
        let ignore_case = std::env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}
