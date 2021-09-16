use crate::{envm::MalEnv, eval::EvalError};

#[derive(Debug, Clone)]
pub enum MalType<'a> {
    Nil,
    Bool(bool),
    Symbol(String),
    String(String),
    Integer(usize),
    List(Vec<MalType<'a>>),
    Fn {
        params: Vec<MalType<'a>>,
        function: fn(&'a mut MalEnv<'a>) -> Result<MalType<'a>, EvalError<'a>>,
    },
}
