use rustyline::error::ReadlineError;
use rustyline::Editor;

mod reader;
mod types;
mod printer;

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

fn eval(input: types::MalType) -> types::MalType {
    return input;
}

fn print(input: types::MalType) {
    printer::pr_str(input)
}

fn repl(mut editor: Editor<()>) {
    loop {
        let readline = read(&mut editor);
        match readline {
            Ok(line) => {
                let output = eval(line);
                print(output);
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
