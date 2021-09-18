use std::collections::HashMap;

use crate::envm::MalEnv;
use crate::eval::{eval, EvalError};
use crate::types::{Function, MalType};

macro_rules! builtin {
    ($envm:ident, $name:expr, [ $( $params:expr),* ], $func:expr) => {
        $envm.set(
            $name,
            MalType::Function(Function::Builtin {
                params: vec![$( $params )*]
                    .iter()
                    .map(|p: &&str| MalType::Symbol((*p).to_owned()))
                    .collect::<Vec<MalType>>(),
                body: $func,
            }),
        );
    };
}

pub fn stdenv<'a>() -> MalEnv<'a> {
    let mut envm: MalEnv = MalEnv {
        parent: None,
        env: HashMap::new(),
    };
    builtin!(envm, "add", ["args..."], |envm| { 
        let args = eval(MalType::Symbol("args".to_owned()), envm)?;
        if let MalType::List(args) = args {
            let result = args.iter().try_fold(0, |acc, x| {
                if let MalType::Integer(x) = x {
                    Ok(x + acc)
                } else {
                    Err(EvalError::InvalidType("Integer", x.clone()))
                }
            })?;
            Ok(MalType::Integer(result))
        } else {
            Err(EvalError::InvalidType("List", args))
        }
    });

    builtin!(envm, "list", ["args..."], |envm| {
        let args = eval(MalType::Symbol("args".to_owned()), envm)?;
        return Ok(args);
    });

    builtin!(envm, "throw", ["err"], |envm| {
        let err = eval(MalType::Symbol("err".to_owned()), envm)?;
        if let MalType::String(err) = err {
            Err(EvalError::Throw(err))
        } else {
            Err(EvalError::InvalidType("String", err))
        }
    });

    return envm;
}
