use crate::menv::MalEnv;
use crate::types::{Function, MalType};

#[derive(Debug)]
pub enum EvalError {
    UndefinedSymbol(String),
    InvalidType(&'static str, MalType),
    LengthMismatch,
    Throw(String),
}

pub fn eval(input: MalType, menv: &mut MalEnv) -> Result<MalType, EvalError> {
    let input = eval_macroexpand(input, menv)?;
    match &input {
        MalType::List(list) if list.is_empty() => Ok(input),
        MalType::List(list) => {
            let special = &list[0];
            if let MalType::Symbol(keyword) = special {
                match &keyword[..] {
                    "def!" => return eval_defbang(list, menv),
                    "let*" => return eval_letstar(list, menv),
                    "do" => return eval_do(list, menv),
                    "if" => return eval_if(list, menv),
                    "fn*" => return eval_fnstar(list, menv),
                    "macro*" => return eval_macrostar(list, menv),
                    "quote" => return eval_quote(list),
                    "quasiquote" => return eval_quasiquote(list, menv),
                    _ => (),
                }
            }
            return eval_apply(input, menv);
        }
        _ => eval_ast(input, menv),
    }
}

fn init_function_menv<'a>(
    params: &[MalType],
    args: &[MalType],
    menv: &'a MalEnv,
) -> Result<MalEnv<'a>, EvalError> {
    let mut params = params.iter();
    let mut args = args.iter();
    let mut nmenv = MalEnv::init(Some(menv));
    while let Some(param) = params.next() {
        if let MalType::Symbol(symbol) = param {
            if let Some(symbol) = symbol.strip_suffix("...") {
                if let Some(_) = params.next() {
                    return Err(EvalError::LengthMismatch);
                }
                let rest: Vec<_> = args.by_ref().map(|m| m.clone()).collect();
                nmenv.set(symbol, MalType::List(rest));
            } else if let Some(value) = args.next() {
                nmenv.set(symbol, value.clone());
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
        return Ok(nmenv);
    }
}

fn eval_macroexpand(input: MalType, mut menv: &mut MalEnv) -> Result<MalType, EvalError> {
    match &input {
        MalType::List(items) if items.is_empty() => Ok(input),
        MalType::List(items) => match &items[0] {
            MalType::Symbol(symbol) => match menv.get(symbol) {
                Some(MalType::Function(Function::Macro { mut params, body })) => {
                    let mut nmenv = init_function_menv(&mut params, &items[1..], &mut menv)?;
                    eval_macroexpand(eval(*body, &mut nmenv)?, &mut nmenv)
                }
                _ => Ok(input),
            },
            _ => Ok(input),
        },
        _ => Ok(input),
    }
}

fn eval_apply(input: MalType, mut menv: &mut MalEnv) -> Result<MalType, EvalError> {
    let elist = eval_ast(input, menv)?;
    if let MalType::List(items) = elist {
        let func = items[0].clone();
        if let MalType::Function(Function::User { params, body }) = func {
            let mut nmenv = init_function_menv(&params, &items[1..], &mut menv)?;
            return eval(*body, &mut nmenv);
        } else if let MalType::Function(Function::Native { params, body }) = func {
            let mut nmenv = init_function_menv(&params, &items[1..], &mut menv)?;
            return body(&mut nmenv);
        } else {
            return Err(EvalError::InvalidType("Function", func));
        }
    } else {
        unreachable!()
    }
}

fn eval_defbang(items: &[MalType], menv: &mut MalEnv) -> Result<MalType, EvalError> {
    if items.len() != 3 {
        return Err(EvalError::LengthMismatch);
    }
    if let MalType::Symbol(symbol) = &items[1] {
        let value = eval(items[2].clone(), menv)?;
        menv.set(symbol, value.clone());
        return Ok(value);
    } else {
        return Err(EvalError::InvalidType("Symbol", items[1].clone()));
    }
}

fn eval_letstar(items: &[MalType], menv: &mut MalEnv) -> Result<MalType, EvalError> {
    if items.len() != 3 {
        return Err(EvalError::LengthMismatch);
    }
    let mut nmenv = MalEnv::init(Some(menv));

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
                let value = eval(value.clone(), &mut nmenv)?;
                nmenv.set(symbol, value.clone());
            } else {
                return Err(EvalError::InvalidType("Symbol", symbol.clone()));
            }
        }
        return eval(items[2].clone(), &mut nmenv);
    } else {
        return Err(EvalError::InvalidType("Symbol", items[1].clone()));
    }
}

fn eval_do(items: &[MalType], menv: &mut MalEnv) -> Result<MalType, EvalError> {
    let mut stmts = items.iter();
    let _ = stmts.next(); // "do"
    let mut last_val = MalType::Nil;
    while let Some(stmt) = stmts.next() {
        last_val = eval_ast(stmt.clone(), menv)?;
    }
    return Ok(last_val);
}

fn eval_if(items: &[MalType], menv: &mut MalEnv) -> Result<MalType, EvalError> {
    if items.len() < 3 || items.len() > 4 {
        return Err(EvalError::LengthMismatch);
    }
    let condition = eval(items[1].clone(), menv)?;
    if let MalType::Bool(condition) = condition {
        if condition {
            return eval(items[2].clone(), menv);
        } else if items.len() == 4 {
            return eval(items[3].clone(), menv);
        } else {
            return Ok(MalType::Nil);
        }
    } else {
        return Err(EvalError::InvalidType("Bool", condition.clone()));
    }
}

fn eval_fnstar(items: &[MalType], _menv: &mut MalEnv) -> Result<MalType, EvalError> {
    if items.len() != 3 {
        return Err(EvalError::LengthMismatch);
    }
    if let MalType::List(params) = &items[1] {
        return Ok(MalType::Function(Function::User {
            params: params.clone(),
            body: Box::new(items[2].clone()),
        }));
    } else {
        return Err(EvalError::InvalidType("Symbol", items[1].clone()));
    }
}

fn eval_macrostar(items: &[MalType], _menv: &mut MalEnv) -> Result<MalType, EvalError> {
    if items.len() != 3 {
        return Err(EvalError::LengthMismatch);
    }
    if let MalType::List(params) = &items[1] {
        return Ok(MalType::Function(Function::Macro {
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

fn eval_unquote(items: &[MalType], menv: &mut MalEnv) -> Result<MalType, EvalError> {
    if items.len() != 2 {
        return Err(EvalError::LengthMismatch);
    }
    eval(items[1].clone(), menv)
}

fn eval_quasiquote(items: &[MalType], menv: &mut MalEnv) -> Result<MalType, EvalError> {
    if items.len() != 2 {
        return Err(EvalError::LengthMismatch);
    }
    let ast = items[1].clone();
    if let MalType::List(list) = ast {
        match &list[..] {
            [MalType::Symbol(symbol), ..] if symbol == "unquote" => eval_unquote(&list, menv),
            list => {
                let mut result = vec![];
                for elt in list {
                    if let MalType::List(list) = elt {
                        match &list[..] {
                            [MalType::Symbol(symbol), ..] if symbol == "unquote" => {
                                let unquoted = eval_unquote(&list, menv)?;
                                result.push(unquoted);
                            }
                            [MalType::Symbol(symbol), ..] if symbol == "splice-unquote" => {
                                let unquoted = eval_unquote(&list, menv)?;
                                if let MalType::List(list) = unquoted {
                                    for elt in list {
                                        result.push(elt);
                                    }
                                } else {
                                    return Err(EvalError::InvalidType("List", unquoted));
                                }
                            }
                            _ => result.push(elt.clone()),
                        }
                    } else {
                        result.push(elt.clone());
                    }
                }
                Ok(MalType::List(result))
            }
        }
    } else {
        Ok(ast)
    }
}

fn eval_ast(input: MalType, mut menv: &mut MalEnv) -> Result<MalType, EvalError> {
    match input {
        MalType::Symbol(symbol) => match menv.get(&symbol) {
            Some(result) => Ok(result.clone()),
            None => Err(EvalError::UndefinedSymbol(symbol)),
        },
        MalType::List(list) => {
            let mut result = Vec::new();
            for item in list {
                let value = eval(item.clone(), &mut menv)?;
                result.push(value);
            }
            return Ok(MalType::List(result));
        }
        x => Ok(x),
    }
}
