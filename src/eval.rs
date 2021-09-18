use std::collections::HashMap;

use crate::envm::MalEnv;
use crate::types::{Function, MalType};

#[derive(Debug)]
pub enum EvalError {
    UndefinedSymbol(String),
    InvalidType(&'static str, MalType),
    LengthMismatch,
    Throw(String),
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
                    "do" => return eval_do(list, envm),
                    "if" => return eval_if(list, envm),
                    "fn*" => return eval_fnstar(list),
                    "quote" => return eval_quote(list),
                    "quasiquote" => return eval_quasiquote(list, envm),
                    _ => (),
                }
            }
            return eval_apply(input, envm);
        }
        _ => eval_ast(input, envm),
    }
}

fn init_function_envm<'a>(
    params: &mut [MalType],
    args: &mut [MalType],
    envm: &'a mut MalEnv,
) -> Result<MalEnv<'a>, EvalError> {
    let mut params = params.iter();
    let mut args = args.iter();
    let mut nenvm = MalEnv {
        parent: Some(envm),
        env: HashMap::new(),
    };
    while let Some(param) = params.next() {
        if let MalType::Symbol(symbol) = param {
            if let Some(symbol) = symbol.strip_suffix("...") {
                if let Some(_) = params.next() {
                    return Err(EvalError::LengthMismatch);
                }
                let rest: Vec<_> = args.by_ref().map(|m| m.clone()).collect();
                nenvm.set(symbol, MalType::List(rest));
            } else if let Some(value) = args.next() {
                nenvm.set(symbol, value.clone());
            } else {
                return Err(EvalError::LengthMismatch);
            }
        } else {
            return Err(EvalError::InvalidType("Symbol", param.clone()));
        }
    }
    if let Some(_) = args.next() {
        return Err(EvalError::LengthMismatch);
    } else {
        return Ok(nenvm);
    }
}
fn eval_apply(input: MalType, mut envm: &mut MalEnv) -> Result<MalType, EvalError> {
    let elist = eval_ast(input, envm)?;
    if let MalType::List(mut items) = elist {
        let func = items[0].clone();

        if let MalType::Function(Function::UserDefined { mut params, body }) = func {
            let mut nenvm = init_function_envm(&mut params, &mut items[1..], &mut envm)?;
            return eval(*body, &mut nenvm);
        } else if let MalType::Function(Function::Builtin { mut params, body }) = func {
            let mut nenvm = init_function_envm(&mut params, &mut items[1..], &mut envm)?;
            return body(&mut nenvm);
        } else {
            return Err(EvalError::InvalidType("Function", func));
        }
    } else {
        unreachable!()
    }
}

fn eval_defbang(items: &[MalType], envm: &mut MalEnv) -> Result<MalType, EvalError> {
    if items.len() != 3 {
        return Err(EvalError::LengthMismatch);
    }
    if let MalType::Symbol(symbol) = &items[1] {
        let value = eval(items[2].clone(), envm)?;
        envm.set(symbol, value.clone());
        return Ok(value);
    } else {
        return Err(EvalError::InvalidType("Symbol", items[1].clone()));
    }
}

fn eval_letstar(items: &[MalType], envm: &mut MalEnv) -> Result<MalType, EvalError> {
    if items.len() != 3 {
        return Err(EvalError::LengthMismatch);
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

fn eval_do(items: &[MalType], envm: &mut MalEnv) -> Result<MalType, EvalError> {
    let mut stmts = items.iter();
    let _ = stmts.next(); // "do"
    let mut last_val = MalType::Nil;
    while let Some(stmt) = stmts.next() {
        last_val = eval_ast(stmt.clone(), envm)?;
    }
    return Ok(last_val);
}

fn eval_if(items: &[MalType], envm: &mut MalEnv) -> Result<MalType, EvalError> {
    if items.len() < 3 || items.len() > 4 {
        return Err(EvalError::LengthMismatch);
    }
    let condition = eval(items[1].clone(), envm)?;
    if let MalType::Bool(condition) = condition {
        if condition {
            return eval(items[2].clone(), envm);
        } else if items.len() == 4 {
            return eval(items[3].clone(), envm);
        } else {
            return Ok(MalType::Nil);
        }
    } else {
        return Err(EvalError::InvalidType("Bool", condition.clone()));
    }
}

fn eval_fnstar(items: &[MalType]) -> Result<MalType, EvalError> {
    if items.len() != 3 {
        return Err(EvalError::LengthMismatch);
    }
    if let MalType::List(params) = &items[1] {
        return Ok(MalType::Function(Function::UserDefined {
            params: params.clone(),
            body: Box::new(items[2].clone()),
        }));
    } else {
        return Err(EvalError::InvalidType("Symbol", items[1].clone()));
    }
}

fn eval_quote(items: &[MalType]) -> Result<MalType, EvalError> {
    if items.len() != 2 {
        return Err(EvalError::LengthMismatch);
    }
    return Ok(items[1].clone());
}

fn eval_unquote(items: &[MalType], envm: &mut MalEnv) -> Result<MalType, EvalError> {
    if items.len() != 2 {
        return Err(EvalError::LengthMismatch);
    }
    eval(items[1].clone(), envm)
}

fn eval_quasiquote(items: &[MalType], envm: &mut MalEnv) -> Result<MalType, EvalError> {
    if items.len() != 2 {
        return Err(EvalError::LengthMismatch);
    }
    let ast = items[1].clone();
    match ast {
        MalType::List(list) => match &list[..] {
            [MalType::Symbol(symbol), ..] if symbol == "unquote" => eval_unquote(&list, envm),
            list => {
                let mut result = vec![];
                for elt in list {
                    if let MalType::List(list) = elt {
                        match &list[..] {
                            [MalType::Symbol(symbol), ..] if symbol == "unquote" => {
                                let unquoted = eval_unquote(&list, envm)?;
                                result.push(unquoted);
                            }
                            [MalType::Symbol(symbol), ..] if symbol == "splice-unquote" => {
                                let unquoted = eval_unquote(&list, envm)?;
                                if let MalType::List(list) = unquoted {
                                    for elt in list {
                                        result.push(elt);
                                    }
                                } else {
                                    return Err(EvalError::InvalidType("List", unquoted));
                                }
                            },
                            _ => result.push(elt.clone()),
                        }
                    } else {
                        result.push(elt.clone());
                    }
                }
                Ok(MalType::List(result))
            }
        },
        _ => Ok(ast),
    }
}

fn eval_ast(input: MalType, mut envm: &mut MalEnv) -> Result<MalType, EvalError> {
    match input {
        MalType::Symbol(symbol) => match envm.get(&symbol) {
            Some(result) => Ok(result.clone()),
            None => Err(EvalError::UndefinedSymbol(symbol)),
        },
        MalType::List(list) => {
            let mut result = Vec::new();
            for item in list {
                let value = eval(item.clone(), &mut envm)?;
                result.push(value);
            }
            return Ok(MalType::List(result));
        }
        x => Ok(x),
    }
}
