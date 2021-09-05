use std::{iter::Peekable, result::Result};
use lazy_static::lazy_static;
use regex::{CaptureMatches, Match, Regex};

use crate::types::MalType;

#[derive(Debug, Clone)]
pub enum ReaderError {
    LexerError,
    ParserError,
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
    let peekable = reader.peekable();
    read_form(peekable)
}

fn read_form(mut reader: Peekable<CaptureMatches>) -> Result<MalType, ReaderError> {
    if let Some(captured) = reader.peek() {
        let matched = captured.get(1).unwrap();
        match matched.as_str() {
            "(" => read_list(reader),
            "" =>  unimplemented!(),
            _ => unimplemented!(),
        }
    } else {
        return Err(ReaderError::LexerError);
    }
}

fn read_list(mut reader: Peekable<CaptureMatches>) -> Result<MalType, ReaderError> {
    let mut collector = Vec::new();
    let _captured = reader.next().unwrap();
    while let Some(captured) = reader.peek() {
        let matched = captured.get(1).unwrap();
        match matched.as_str() {
            ")" => return Ok(MalType::List(collector)),
            _ => if let Ok(item) = read_form(reader) {
                collector.push(Box::new(item));
            } else {

            },
        }
    }
    unimplemented!()
}

fn read_atom() {}
