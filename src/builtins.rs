use crate::eval::{eval, EvalError};
use crate::menv::MalEnv;
use crate::types::{Function, MalType};

macro_rules! builtin {
    ($menv:ident, $name:expr, [ $( $params:expr),* ], $func:expr) => {
        $menv.set(
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
