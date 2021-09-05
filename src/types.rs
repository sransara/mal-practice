use std::fmt;

#[derive(Debug)]
pub enum MalType {
    Nil,
    Quote(Box<MalType>),
    String(String),
    List(Vec<Box<MalType>>),
}

impl fmt::Display for MalType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        // write!(f, "({}, {})", self.0, self.1)
        unimplemented!();
    }
}
