use std::collections::HashMap;

use crate::envm::MalEnv;
use crate::types::MalType;

#[derive(Debug)]
pub enum EvalError<'a> {
    UndefinedSymbol(String),
    NotFunction(MalType<'a>),
    InvalidType(&'static str, MalType<'a>),
    LengthMismatch(usize, usize),
}

pub fn eval<'a>(
    input: MalType<'a>,
    envm: &'a mut MalEnv<'a>,
) -> Result<MalType<'a>, EvalError<'a>> {
    match &input {
        MalType::List(list) if list.is_empty() => Ok(input),
        MalType::List(list) => {
            let special = &list[0];
            if let MalType::Symbol(keyword) = special {
                match &keyword[..] {
                    "def!" => return eval_defbang(list, envm),
                    "let*" => return eval_letstar(list, envm),
                    "do" => return eval_do(list, envm),
                    "fn*" => return eval_fnstar(list, envm),
                    _ => (),
                }
            }
            return eval_apply(input, envm);
        }
        _ => eval_ast(input, envm),
    }
}

fn eval_apply<'a>(
    input: MalType<'a>,
    envm: &'a mut MalEnv<'a>,
) -> Result<MalType<'a>, EvalError<'a>> {
    let elist = eval_ast(input, envm)?;
    if let MalType::List(items) = &elist {
        let func = &items[0];
        if let MalType::Fn { params, function } = func {
            let args = items[1..].to_vec();
            return function(envm);
        } else {
            return Err(EvalError::NotFunction(func.clone()));
        }
    } else {
        unreachable!()
    }
}

fn eval_defbang<'a>(
    items: &'a Vec<MalType<'a>>,
    envm: &'a mut MalEnv<'a>,
) -> Result<MalType<'a>, EvalError<'a>> {
    if items.len() != 3 {
        return Err(EvalError::LengthMismatch(3, items.len()));
    }
    if let MalType::Symbol(symbol) = &items[1] {
        let value = eval(items[2].clone(), envm)?;
        envm.set(symbol, value.clone());
        return Ok(value);
    } else {
        return Err(EvalError::InvalidType("Symbol", items[1].clone()));
    }
}

fn eval_letstar<'a>(
    items: &'a Vec<MalType<'a>>,
    envm: &'a mut MalEnv<'a>,
) -> Result<MalType<'a>, EvalError<'a>> {
    if items.len() != 3 {
        return Err(EvalError::LengthMismatch(3, items.len()));
    }
    let mut nenvm = MalEnv {
        parent: Some(envm),
        env: HashMap::new(),
    };
    if let MalType::List(bindings) = &items[1] {
        if !bindings.is_empty() && bindings.len() % 2 != 0 {
            return Err(EvalError::InvalidType(
                "Associative array",
                items[1].clone(),
            ));
        }
        let mut bindings = bindings.iter();
        while let Some(symbol) = bindings.next() {
            if let MalType::Symbol(symbol) = symbol {
                let value = bindings.next().unwrap();
                let value = eval(value.clone(), &mut nenvm)?;
                nenvm.set(symbol, value.clone());
            } else {
                return Err(EvalError::InvalidType("Symbol", symbol.clone()));
            }
        }
        return eval(items[2].clone(), &mut nenvm);
    } else {
        return Err(EvalError::InvalidType("Symbol", items[1].clone()));
    }
}

fn eval_do<'a>(
    items: &'a Vec<MalType<'a>>,
    envm: &'a mut MalEnv<'a>,
) -> Result<MalType<'a>, EvalError<'a>> {
    let mut stmts = items.iter();
    let _ = stmts.next(); // "do"
    let mut last_val = MalType::Nil;
    while let Some(stmt) = stmts.next() {
        last_val = eval_ast(stmt.clone(), envm)?;
    }
    return Ok(last_val);
}
fn eval_fnstar<'a>(
    items: &'a Vec<MalType<'a>>,
    envm: &'a mut MalEnv<'a>,
) -> Result<MalType<'a>, EvalError<'a>> {
    if items.len() != 3 {
        return Err(EvalError::LengthMismatch(3, items.len()));
    }
    if let MalType::List(params) = &items[1] {
        let a = |envm| {
            eval(items[2].clone(), envm)
        };
        unimplemented!()
        // return Ok(MalType::Fn {params, function: );
    } else {
        return Err(EvalError::InvalidType("Symbol", items[1].clone()));
    }
}

fn eval_ast<'a>(
    input: MalType<'a>,
    mut envm: &'a mut MalEnv<'a>,
) -> Result<MalType<'a>, EvalError<'a>> {
    match input {
        MalType::Symbol(symbol) => match envm.get(&symbol) {
            Some(result) => Ok(result.clone()),
            None => Err(EvalError::UndefinedSymbol(symbol)),
        },
        MalType::List(list) => {
            let result = Vec::new();
            for item in list {
                let value = eval(item.clone(), &mut envm)?;
                result.push(value);
            }
            return Ok(MalType::List(result));
        }
        x => Ok(x),
    }
}

pub fn stdenv<'a>() -> MalEnv<'a> {
    let mut envm: MalEnv<'a> = MalEnv {
        parent: None,
        env: HashMap::new(),
    };
    envm.set(
        "add",
        MalType::Fn {
            params: vec![
                MalType::Symbol("args".to_owned()),
                MalType::Symbol("...".to_owned()),
            ],
            function: |envm| {
                let args = eval_ast(MalType::Symbol("args".to_owned()), envm)?;
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
        },
    );
    return envm;
}
