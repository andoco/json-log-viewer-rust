use std::io::{self, BufRead};
use serde_json::{Value};

#[derive(Debug, Clone)]
#[derive(PartialEq)]
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
    use serde_json::{json};

    #[test]
    fn test_format_log_line() {
        let v = json!({"timestamp": "2019-08-25T09:00:00", "level": "info", "message": "a test message"});
        assert_eq!("2019-08-25T09:00:00 [info] a test message", format_log_line(v).unwrap())
    }

    #[test]
    fn test_format_log_line_with_missing_timestamp_returns_error() {
        let v = json!({"level": "info", "message": "a test message"});
        match format_log_line(v) {
            Ok(_) => panic!("expected an error"),
            Err(LogLineError::Field(_, name)) => assert_eq!(String::from("timestamp"), name),
        }
    }

    #[test]
    fn test_format_log_line_with_missing_level_returns_error() {
        let v = json!({"timestamp": "2019-08-25T09:00:00", "message": "a test message"});
        match format_log_line(v) {
            Ok(_) => panic!("expected an error"),
            Err(LogLineError::Field(_, name)) => assert_eq!(String::from("level"), name),
        }
    }

    #[test]
    fn test_format_log_line_with_missing_message_returns_error() {
        let v = json!({"timestamp": "2019-08-25T09:00:00", "level": "info"});
        match format_log_line(v) {
            Ok(_) => panic!("expected an error"),
            Err(LogLineError::Field(_, name)) => assert_eq!(String::from("message"), name),
        }
    }
}
