use std::io::{self, BufRead};
use serde_json::{Value};

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

        match print_log_line(v) {
            Ok(_) => {},
            Err(LogLineError::Field(v, f)) => println!("Could not print line with json: {}, Problem Field: {:?}", v, f),
        }
    }
}

fn print_log_line(v: Value) -> Result<(),LogLineError> {
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

        println!("{timestamp} [{level}] {message}", timestamp=timestamp, level=level, message=message);

        Ok(())
}
