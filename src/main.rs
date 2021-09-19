use rustyline::error::ReadlineError;
use rustyline::Editor;

mod builtins;
mod menv;
mod eval;
mod printer;
mod reader;
mod types;

#[derive(Debug)]
enum ReadError {
    Readline(ReadlineError),
    Reader(reader::ReaderError),
}

fn read(editor: &mut Editor<()>) -> Result<types::MalType, ReadError> {
    let input = editor
        .readline("> ")
        .map_err(|err| ReadError::Readline(err))?;
    editor.add_history_entry(input.as_str());
    reader::read_str(input.as_str()).map_err(|err| ReadError::Reader(err))
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
