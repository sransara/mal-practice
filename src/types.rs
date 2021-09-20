use crate::{eval::EvalError, menv::MalEnv};

#[derive(Clone)]
pub enum Function {
    Native {
        params: Vec<MalType>,
        body: fn(&mut MalEnv) -> Result<MalType, EvalError>,
    },
    User {
        params: Vec<MalType>,
        body: Box<MalType>,
    },
    Macro {
        params: Vec<MalType>,
        body: Box<MalType>,
    },
}

impl std::fmt::Debug for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Native { params, .. } => f
                .debug_struct("Native")
                .field("params", params)
                .field("body", &"<native>")
                .finish(),
            Self::User { params, body } => f
                .debug_struct("User")
                .field("params", params)
                .field("body", body)
                .finish(),
            Self::Macro { params, body } => f
                .debug_struct("Macro")
                .field("params", params)
                .field("body", body)
                .finish(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum MalType {
    Nil,
    Bool(bool),
    Symbol(String),
    String(String),
    Integer(usize),
    List(Vec<MalType>),
    Function(Function),
}
