use lazy_static::lazy_static;
use regex::{CaptureMatches, Regex};
use std::{iter::Peekable, result::Result};

use crate::types::MalType;

#[derive(Debug, Clone)]
pub enum ReaderError {
    EOL,
    Ignore,
    Unbalanced(&'static str, usize),
}

pub fn read_str(input: &str) -> Result<MalType, ReaderError> {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r#"(?x)
            [\s,]*
            (
                ~@ |
                [\[\]{}()'`~^@] |
                "(?:\\.|[^\\"])*"? |
                ;.* |
                [^\s\[\]{}('"`,;)]*
            )
            "#
        )
        .unwrap();
    }
    let reader = RE.captures_iter(input);
    let mut peekable = reader.peekable();
    read_form(&mut peekable)
}

fn read_form(reader: &mut Peekable<CaptureMatches>) -> Result<MalType, ReaderError> {
    let captured = reader.peek().unwrap();
    let matched = captured.get(1).unwrap();
    match matched.as_str() {
        "(" => read_list(reader),
        "" => Err(ReaderError::EOL),
        text if text.starts_with(";") => Err(ReaderError::Ignore),
        _ => read_atom(reader),
    }
}

fn read_list(reader: &mut Peekable<CaptureMatches>) -> Result<MalType, ReaderError> {
    let mut collector = Vec::new();
    let list = reader.next().unwrap().get(1).unwrap();
    let list_read_err = Err(ReaderError::Unbalanced("(", list.start()));
    while let Some(captured) = reader.peek() {
        let matched = captured.get(1).unwrap();
        if matched.as_str() == ")" {
            let _ = reader.next();
            return Ok(MalType::List(collector));
        }
        match read_form(reader) {
            Ok(item) => collector.push(item),
            Err(ReaderError::EOL) | Err(ReaderError::Ignore) => return list_read_err,
            err => return err,
        };
    }
    return list_read_err;
}

fn read_atom(reader: &mut Peekable<CaptureMatches>) -> Result<MalType, ReaderError> {
    let atom = reader.next().unwrap().get(1).unwrap();
    lazy_static! {
        static ref STRING: Regex = Regex::new(r#"^"((?:\\.|[^\\"])*)"$"#).unwrap();
        static ref INTEGER: Regex = Regex::new(r#"^(\d+)$"#).unwrap();
    }
    match atom.as_str() {
        "true" => Ok(MalType::Bool(true)),
        "false" => Ok(MalType::Bool(false)),
        "nil" => Ok(MalType::Nil),
        text if text.starts_with('"') => {
            if let Some(captured) = STRING.captures(text) {
                let text = captured.get(1).unwrap().as_str().to_owned();
                Ok(MalType::String(text))
            } else {
                Err(ReaderError::Unbalanced("\"", atom.start()))
            }
        }
        text if INTEGER.is_match(text) => Ok(MalType::Integer(text.parse().unwrap())),
        text => Ok(MalType::Symbol(text.to_owned())),
    }
}
