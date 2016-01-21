use super::*;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct GlobalName {
    namespace: Symbol,
    name: Symbol,
}

#[derive(Debug)]
pub struct Globals {
    globals: Vec<(GlobalName, Value)>
}

impl Globals {
    pub fn new() -> Globals {
        Globals {
            globals: vec![]
        }
    }

    pub fn get(&self, namespace: Symbol, name: Symbol) -> Option<&Value> {
        self.globals.iter().filter_map(|&(ref global_name, ref value)| {
            if global_name.namespace == namespace && global_name.name == name {
                Some(value)
            } else {
                None
            }
        }).next()
    }

    pub fn get_mut(&mut self, namespace: Symbol, name: Symbol) -> Option<&mut Value> {
        self.globals.iter_mut().filter_map(|&mut (ref global_name, ref mut value)| {
            if global_name.namespace == namespace && global_name.name == name {
                Some(value)
            } else {
                None
            }
        }).next()
    }

    pub fn set(&mut self, namespace: Symbol, name: Symbol, value: Value) {
        if let Some(existing) = self.get_mut(namespace, name) {
            *existing = value;
            return;
        }

        self.globals.push((GlobalName {
            namespace: namespace,
            name: name
        }, value));
    }
}
