use crate::{envm::MalEnv, eval::EvalError};

#[derive(Clone)]
pub enum MalType {
    Nil,
    Bool(bool),
    Symbol(String),
    String(String),
    Integer(usize),
    List(Vec<MalType>),
    BuiltinFunction {
        params: Vec<MalType>,
        body: fn(&mut MalEnv) -> Result<MalType, EvalError>
    },
    Function {
        params: Vec<MalType>,
        body: Box<MalType>,
    },
}

impl std::fmt::Debug for MalType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nil => write!(f, "Nil"),
            Self::Bool(bool) => f.debug_tuple("Bool").field(bool).finish(),
            Self::Symbol(symbol) => f.debug_tuple("Symbol").field(symbol).finish(),
            Self::String(string) => f.debug_tuple("String").field(string).finish(),
            Self::Integer(integer) => f.debug_tuple("Integer").field(integer).finish(),
            Self::List(list) => f.debug_tuple("List").field(list).finish(),
            Self::BuiltinFunction { params, .. } => f.debug_struct("BuiltinFunction").field("params", params).finish(),
            Self::Function { params, body } => f.debug_struct("Function").field("params", params).field("body", body).finish(),
        }
    }
}
