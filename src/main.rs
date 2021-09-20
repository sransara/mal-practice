use rustyline::error::ReadlineError;
use rustyline::Editor;

mod builtins;
mod eval;
mod menv;
mod printer;
mod reader;
mod types;

#[derive(Debug)]
enum ReadError {
    Readline(ReadlineError),
    Reader(reader::ReaderError),
}

fn read(editor: &mut Editor<()>) -> Result<types::MalType, ReadError> {
    let mut acc = String::new();
    let mut loopback;
    let mut prompt = ">>> ";
    loop {
        match editor.readline(prompt) {
            Ok(input) => {
                editor.add_history_entry(input.as_str());
                acc.push_str(&input);
                acc.push_str("\n");
                loopback = true;
            }
            Err(ReadlineError::Eof) if !acc.is_empty() => loopback = false,
            Err(err) => return Err(ReadError::Readline(err)),
        }
        match reader::read_str(acc.as_str()) {
            Err(reader::ReaderError::Unbalanced(_)) if loopback => prompt = "... ", // loop back
            Err(err) => return Err(ReadError::Reader(err)),
            Ok(x) => return Ok(x),
        }
    }
}

fn eval(input: types::MalType, menv: &mut menv::MalEnv) -> Result<types::MalType, eval::EvalError> {
    eval::eval(input, menv)
}

fn print(input: types::MalType) {
    printer::pr_str(input)
}

fn repl(mut editor: Editor<()>) {
    let mut menv = builtins::stdenv();
    loop {
        let readline = read(&mut editor);
        match readline {
            Ok(line) => {
                let result = eval(line, &mut menv);
                match result {
                    Ok(result) => print(result),
                    Err(err) => println!("{:?}", err),
                }
            }
            Err(ReadError::Reader(err)) => {
                println!("{:?}", err);
            }
            Err(ReadError::Readline(ReadlineError::Eof)) => {
                break;
            }
            Err(ReadError::Readline(err)) => {
                println!("{:?}", err);
                break;
            }
        }
    }
}

fn main() {
    let readline_editor = Editor::<()>::new();
    repl(readline_editor)
}
