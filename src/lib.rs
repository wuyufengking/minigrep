const RED: &str = "\x1b[31m";
const RESET: &str = "\x1b[0m";

pub struct Config {
    pub query: String,
    pub file_path: Option<String>,
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


pub fn search<'a> (query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

pub fn highlight_matches(line: &str, line_to_check: &str, query_to_check: &str) -> String {
    let mut result = String::new();
    let len = query_to_check.len();
    let mut last_index = 0;

    for (start_index, _) in line_to_check.match_indices(&query_to_check) {
        result.push_str(&line[last_index..start_index]);
        result.push_str(RED);
        result.push_str(&line[start_index..start_index + len]);
        result.push_str(RESET);
        last_index = start_index + len;
    }

    result.push_str(&line[last_index..]);

    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}