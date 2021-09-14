use std::collections::HashMap;

use crate::types::MalType;
use crate::envm::MalEnv;

#[derive(Debug)]
pub enum EvalError {
    UndefinedSymbol(String),
    NotFunction(MalType),
    InvalidType(&'static str, MalType),
    LengthMismatch(usize, usize)
}

pub fn eval(input: MalType, envm: &mut MalEnv) -> Result<MalType, EvalError> {
    match &input {
        MalType::List(list) if list.is_empty() => Ok(input),
        MalType::List(list) => {
            let special = &list[0];
            if let MalType::Symbol(keyword) = special {
                match &keyword[..] {
                    "def!" => return eval_defbang(list, envm),
                    "let*" => return eval_letstar(list, envm),
                    _ => (),
                }
            }
            return eval_apply(input, envm);
        }
        _ => eval_ast(input, envm),
    }
}

fn eval_apply(input: MalType, envm: &mut MalEnv)  -> Result<MalType, EvalError> {
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

fn eval_defbang(items: &Vec<MalType>, envm: &mut MalEnv)  -> Result<MalType, EvalError> {
    if items.len() != 3 {
        return Err(EvalError::LengthMismatch(3, items.len()));
    }
    if let MalType::Symbol(symbol) = &items[1] {
        let value = eval(items[2].clone(), envm)?;
        envm.set(symbol, value.clone());
        return Ok(value);
    }
    else {
        return Err(EvalError::InvalidType("Symbol", items[1].clone()))
    }
}

fn eval_letstar(items: &Vec<MalType>, envm: &mut MalEnv)  -> Result<MalType, EvalError> {
    if items.len() != 3 {
        return Err(EvalError::LengthMismatch(3, items.len()));
    }
    let mut nenvm = MalEnv { parent: Some(envm), env: HashMap::new() };
    if let MalType::List(bindings) = &items[1] {
        if !bindings.is_empty() && bindings.len() % 2 != 0 {
            return Err(EvalError::InvalidType("Associative array", items[1].clone()));
        }
        let mut bindings = bindings.iter();
        while let Some(symbol) = bindings.next() {
            if let MalType::Symbol(symbol) = symbol {
                let value = bindings.next().unwrap();
                let value = eval(value.clone(), &mut nenvm)?;
                nenvm.set(symbol, value.clone());
            }
            else {
                return Err(EvalError::InvalidType("Symbol", symbol.clone()));
            }
        }
        return eval(items[2].clone(), &mut nenvm);
    } else {
        return Err(EvalError::InvalidType("Symbol", items[1].clone()));
    }
    
}

fn eval_ast(input: MalType, mut envm: &mut MalEnv) -> Result<MalType, EvalError> {
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

pub fn stdenv<'a>() -> MalEnv<'a> {
    let mut envm = MalEnv { parent: None, env: HashMap::new() };
    envm.set("add", MalType::Fn("add".to_owned(), |v| {
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
