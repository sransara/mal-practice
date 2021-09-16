use rustyline::error::ReadlineError;
use rustyline::Editor;

mod reader;
mod types;
mod printer;
mod eval;
mod envm;

#[derive(Debug)]
enum ReadError {
    Readline(ReadlineError),
    Reader(reader::ReaderError),
}

fn read(editor: &mut Editor<()>) -> Result<types::MalType, ReadError> {
    let input = editor.readline("> ").map_err(|err| ReadError::Readline(err))?;
    editor.add_history_entry(input.as_str());
    reader::read_str(input.as_str()).map_err(|err| ReadError::Reader(err))
}

fn eval<'a>(input: types::MalType<'a>, envm: &'a mut envm::MalEnv<'a>) -> Result<types::MalType<'a>, eval::EvalError<'a>> {
    eval::eval(input, envm)
}

fn print(input: types::MalType) {
    printer::pr_str(input)
}

fn repl(mut editor: Editor<()>) {
    let mut envm = eval::stdenv();
    loop {
        let readline = read(&mut editor);
        match readline {
            Ok(line) => {
                let result = eval(line, &mut envm);
                match result {
                    Ok(result) => print(result),
                    Err(err) => println!("{:?}", err),
                }
            }
            Err(ReadError::Reader(err)) => {
                println!("{:?}", err);
            },
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
