use std::fs;
use fancy_regex::Regex;

#[derive(Debug)]
struct Log {
    commit_id: String,
    author: String,
    email: String,
    date: String,
    message: String,
}

fn main() -> anyhow::Result<()> {
    let logs = fs::read_to_string("./fixtures/log.txt")?;
    let mut log_structs: Vec<Log> = Vec::new();

    let re = Regex::new(r"(?=\ncommit)")?;
    let log_strs = re.split(&logs);
    let re = Regex::new(r"commit (?P<commit_id>[a-z0-9]+)\nAuthor: (?P<author>.*) <(?P<email>.*)>\nDate: (?P<date>.*)\n\n(?P<message>(.|\n)*)")?;

    for log in log_strs {
        let captures = re.captures(log?)?;

        match captures {
            Some(captures) => {
                let commit_id = captures.name("commit_id").unwrap().as_str();
                let author = captures.name("author").unwrap().as_str();
                let email = captures.name("email").unwrap().as_str();
                let date = captures.name("date").unwrap().as_str();
                let message = captures.name("message").unwrap().as_str();

                let log = Log {
                    commit_id: commit_id.to_string(),
                    author: author.to_string(),
                    email: email.to_string(),
                    date: date.to_string(),
                    message: message.to_string(),
                };

                log_structs.push(log);
            }
            None => {}
        }
    }

    println!("共有 log：{:?} 条", log_structs.len());
    println!("第1条 log：{:?}", log_structs[0]);
    println!("第2条 log：{:?}", log_structs[1]);
    println!("第3条 log：{:?}", log_structs[2]);
    println!("第4条 log：{:?}", log_structs[3]);

    Ok(())
}
