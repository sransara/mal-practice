use rustyline::error::ReadlineError;
use rustyline::Editor;

mod reader;
mod types;

fn read(editor: &mut Editor<()>) -> Result<String, ReadlineError> {
    editor.readline("> ")
}

fn eval(input: &str) -> &str {
    return input;
}

fn print(input: &str) {
    println!("{}", input)
}

fn repl(mut editor: Editor<()>) {
    loop {
        let readline = read(&mut editor);
        match readline {
            Ok(line) => {
                editor.add_history_entry(line.as_str());
                let output = eval(line.as_str());
                print(output);
            },
            Err(_) => {
                break
            }
        }
    }
}

fn main() {
    let readline_editor = Editor::<()>::new();
    repl(readline_editor)
}
