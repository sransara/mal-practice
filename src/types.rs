use crate::{eval::EvalError, menv::MalEnv};

#[derive(Clone)]
pub enum Function {
    Builtin {
        params: Vec<MalType>,
        body: fn(&mut MalEnv) -> Result<MalType, EvalError>,
    },
    UserDefined {
        params: Vec<MalType>,
        body: Box<MalType>,
    },
    Macro {
        params: Vec<MalType>,
        body: Box<MalType>,
    }
}

impl std::fmt::Debug for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Builtin { params, .. } => f
                .debug_struct("Builtin")
                .field("params", params)
                .field("body", &"<builtin>")
                .finish(),
            Self::UserDefined { params, body } => f
                .debug_struct("UserDefined")
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
