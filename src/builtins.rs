use std::collections::HashMap;

use crate::envm::MalEnv;
use crate::eval::{eval, EvalError};
use crate::types::{Function, MalType};

pub fn stdenv<'a>() -> MalEnv<'a> {
    let mut envm: MalEnv = MalEnv {
        parent: None,
        env: HashMap::new(),
    };
    envm.set(
        "add",
        MalType::Function(Function::Builtin {
            params: vec![MalType::Symbol("args...".to_owned())],
            body: |envm| {
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
            },
        }),
    );
    return envm;
}
