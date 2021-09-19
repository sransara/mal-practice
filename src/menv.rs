use crate::types::MalType;
use std::collections::HashMap;

#[derive(Debug)]
pub struct MalEnv<'a> {
    parent: Option<&'a MalEnv<'a>>,
    symbol_table: HashMap<String, MalType>,
}

impl<'a> MalEnv<'a> {
    pub fn init(parent: Option<&'a MalEnv<'a>>) -> MalEnv {
        return MalEnv {
            parent,
            symbol_table: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: &str, value: MalType) {
        self.symbol_table.insert(key.to_owned(), value);
    }

    fn find(&'a self, key: &str) -> Option<&MalEnv> {
        if self.symbol_table.contains_key(key) {
            Some(&self)
        } else if self.parent.is_some() {
            self.parent.unwrap().find(key)
        } else {
            None
        }
    }

    pub fn get(&'a self, key: &str) -> Option<MalType> {
        let menv = self.find(key)?;
        let value = menv.symbol_table.get(key)?;
        return Some(value.clone());
    }
}
