use std::collections::HashMap;
use crate::types::MalType;

pub struct MalEnv<'a> {
    pub parent: Option<&'a MalEnv<'a>>,
    pub env: HashMap<String, MalType>,
}

impl<'a> MalEnv<'a> {
    pub fn set(&mut self, key: &str, value: MalType) -> Option<MalType> {
        self.env.insert(key.to_owned(), value)
    }

    fn find(&'a self, key: &str) -> Option<&MalEnv> {
        if self.env.contains_key(key) {
            Some(&self)
        }
        else if self.parent.is_some() {
            self.parent.unwrap().find(key)
        }
        else {
            None
        }
    }

    pub fn get(&'a self, key: &str) -> Option<MalType> {
        let envm = self.find(key)?;
        let value = envm.env.get(key)?;
        return Some(value.clone());
    }
}
