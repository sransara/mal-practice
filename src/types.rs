use crate::eval::EvalError;

#[derive(Debug, Clone)]
pub enum MalType {
    Nil,
    Bool(bool),
    Symbol(String),
    String(String),
    Integer(usize),
    List(Vec<MalType>),
    Fn(String, fn(Vec<MalType>) -> Result<MalType, EvalError>),
}

