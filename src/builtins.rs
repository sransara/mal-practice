use crate::eval::{eval, EvalError};
use crate::menv::MalEnv;
use crate::types::{Function, MalType};

macro_rules! builtin {
    ($menv:ident, $name:expr, [ $( $params:expr),* ], $func:expr) => {
        $menv.set(
            $name,
            MalType::Function(Function::Native {
                params: vec![$( $params ), *]
                    .iter()
                    .map(|p: &&str| MalType::Symbol((*p).to_owned()))
                    .collect::<Vec<MalType>>(),
                body: $func,
            }),
        );
    };
}

pub fn stdenv<'a>() -> MalEnv<'a> {
    let mut menv: MalEnv = MalEnv::init(None);

    builtin!(menv, "add", ["args..."], |menv| {
        let args = eval(MalType::Symbol("args".to_owned()), menv)?;
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

    builtin!(menv, "sub", ["arg1", "arg2"], |menv| {
        let arg1 = eval(MalType::Symbol("arg1".to_owned()), menv)?;
        let arg2 = eval(MalType::Symbol("arg2".to_owned()), menv)?;
        match (&arg1, &arg2) {
            (MalType::Integer(arg1), MalType::Integer(arg2)) => Ok(MalType::Integer(arg1 - arg2)),
            (_, MalType::Integer(_)) => Err(EvalError::InvalidType("Integer", arg1)),
            (MalType::Integer(_), _) => Err(EvalError::InvalidType("Integer", arg2)),
            _ => Err(EvalError::InvalidType("Integer", arg1)),
        }
    });

    builtin!(menv, "lte", ["arg1", "arg2"], |menv| {
        let arg1 = eval(MalType::Symbol("arg1".to_owned()), menv)?;
        let arg2 = eval(MalType::Symbol("arg2".to_owned()), menv)?;
        match (&arg1, &arg2) {
            (MalType::Integer(arg1), MalType::Integer(arg2)) => Ok(MalType::Bool(arg1 <= arg2)),
            (_, MalType::Integer(_)) => Err(EvalError::InvalidType("Integer", arg1)),
            (MalType::Integer(_), _) => Err(EvalError::InvalidType("Integer", arg2)),
            _ => Err(EvalError::InvalidType("Integer", arg1)),
        }
    });

    builtin!(menv, "list", ["args..."], |menv| {
        let args = eval(MalType::Symbol("args".to_owned()), menv)?;
        return Ok(args);
    });

    builtin!(menv, "throw", ["err"], |menv| {
        let err = eval(MalType::Symbol("err".to_owned()), menv)?;
        if let MalType::String(err) = err {
            Err(EvalError::Throw(err))
        } else {
            Err(EvalError::InvalidType("String", err))
        }
    });

    return menv;
}
