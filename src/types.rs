#[derive(Debug)]
pub enum MalType {
    Nil,
    True,
    False,
    Keyword(String),
    Symbol(String),
    String(String),
    Integer(usize),
    List(Vec<Box<MalType>>),
}

