use std::io::{self, BufRead};
use serde_json::{Value,json};

#[derive(Debug, Clone)]
enum LogLineError {
    Field(Value,String)
}

//type LogLineResult<T> = std::result::Result<T, LogLineError>;

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line_string = line.unwrap();
        let line_str = line_string.as_str();
        let v: Value = serde_json::from_str(line_str).unwrap();

        match format_log_line(v) {
            Ok(s) => println!("{}", s),
            Err(LogLineError::Field(v, f)) => println!("Could not print line with json: {}, Problem Field: {:?}", v, f),
        }
    }
}

fn format_log_line(v: Value) -> Result<String,LogLineError> {
    let timestamp = match v["timestamp"].as_str() {
        Some(s) => s,
        None => return Err(LogLineError::Field(v, String::from("timestamp"))),
    };

    let level = match v["level"].as_str() {
        Some(s) => s,
        None => return Err(LogLineError::Field(v, String::from("level"))),
    };

    let message = match v["message"].as_str() {
        Some(s) => s,
        None => return Err(LogLineError::Field(v, String::from("message"))),
    };

    let line = format!("{timestamp} [{level}] {message}", timestamp=timestamp, level=level, message=message);

    Ok(String::from(line))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_log_line() {
        let v = json!({"timestamp": "2019-08-25T09:00:00", "level": "info", "message": "a test message"});
        assert_eq!("2019-08-25T09:00:00 [info] a test message", format_log_line(v).unwrap())
    }
}
