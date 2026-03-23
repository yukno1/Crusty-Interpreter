// for environment need some kind of hash-mao like thing

use std::collections::HashMap;

pub struct Environment<V> {
    vars: HashMap<String, V>,
}

impl<V> Environment<V> {
    pub fn new() -> Environment<V> {
        Environment {
            vars: HashMap::new(),
        }
    }

    pub fn declare(&mut self, name: &str, value: V) {
        //declare a new var (var name = value)
        self.vars.insert(name.into(), value);
    }

    pub fn lookup(&self, name: &str) -> Option<&V> {
        // look up value of a var (might not exists)
        self.vars.get(name)
    }

    pub fn assign(&mut self, name: &str, value: V) {
        // change already existed val (name = value)
        todo!()
    }
}
