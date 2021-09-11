use std::collections::HashMap;

use crate::types::{self, MalEnv, MalType};

#[derive(Debug)]
pub enum EvalError {
    UndefinedSymbol(String),
    NotFunction(MalType),
    InvalidType(&'static str, MalType),
}

pub fn eval(input: types::MalType, envm: &mut MalEnv) -> Result<MalType, EvalError> {
    match &input {
        MalType::List(list) if list.is_empty() => Ok(input),
        MalType::List(_) => {
            let elist = eval_ast(input, envm)?;
            if let MalType::List(items) = &elist {
                let func = &items[0];
                if let MalType::Fn(_, func) = func {
                    let rest = items[1..].to_vec();
                    return func(rest);
                } else {
                    return Err(EvalError::NotFunction(func.clone()));
                }
            } else {
                unreachable!()
            }
        }
        _ => eval_ast(input, envm),
    }
}

fn eval_ast(input: types::MalType, mut envm: &mut MalEnv) -> Result<MalType, EvalError> {
    match input {
        MalType::Symbol(symbol) => match envm.get(&symbol) {
            Some(result) => Ok(result.clone()),
            None => Err(EvalError::UndefinedSymbol(symbol)),
        },
        MalType::List(list) => {
            let result: Result<Vec<_>, _> = list
                .iter()
                .map(|m| eval(m.clone(), &mut envm))
                .collect();
            match result {
                Ok(result) => Ok(MalType::List(result)),
                Err(err) => Err(err),
            }
        }
        x => Ok(x),
    }
}

pub fn stdenv() -> MalEnv {
    let mut envm = HashMap::new();
    envm.insert("add".to_owned(), MalType::Fn("add".to_owned(), |v| {
        let result = v.iter().try_fold(0, |acc, x| {
            if let MalType::Integer(x) = x {
                Ok(x + acc)
            } else {
                Err(EvalError::InvalidType("Integer", x.clone()))
            }
        })?;
        Ok(MalType::Integer(result))
    }));
    return envm;
}
