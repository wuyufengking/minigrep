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

pub fn search<'a>(query: &'a str, contents: &'a str) -> impl Iterator<Item = &'a str> {
    contents.lines().filter(move |line| line.contains(query))
}

pub fn search_case_insensitive<'a>(
    query: &'a str,
    contents: &'a str,
) -> impl Iterator<Item = &'a str> {
    let query_lower = query.to_lowercase();
    contents
        .lines()
        .filter(move |line| line.to_lowercase().contains(&query_lower))
}

pub fn highlight_matches(line: &str, line_to_check: &str, query_to_check: &str) -> String {
    let len = query_to_check.len();
    let escape_len = RED.len() + RESET.len();
    // 预分配：假设有 1 个匹配的容量
    let mut result = String::with_capacity(line.len() + escape_len);
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

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents).collect::<Vec<_>>()
        );
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
            search_case_insensitive(query, contents).collect::<Vec<_>>()
        );
    }
}
