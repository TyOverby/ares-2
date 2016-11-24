use vm::*;
use ares_syntax::*;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct GlobalName {
    namespace: Symbol,
    name: Symbol,
}

#[derive(Debug)]
pub struct Modules {
    namespace_to_src: HashMap<Symbol, Option<String>>,
    globals: Vec<(GlobalName, Value)>
}

impl Modules {
    pub fn new() -> Modules {
        Modules {
            namespace_to_src: HashMap::new(),
            globals: vec![]
        }
    }

    pub fn load_library(&mut self, name: String, version: String, source: String, interner: &mut SymbolIntern) -> Symbol {
        let symbol = interner.intern(format!("{}@{}", name, version));
        if !self.namespace_to_src.contains_key(&symbol) {
            self.namespace_to_src.insert(symbol, Some(source));
        } else {
            panic!("already loaded");
        }
        return symbol;
    }

    pub fn force_library(&mut self, symbol: Symbol) -> String {
        if let Some(src_opt) = self.namespace_to_src.get_mut(&symbol) {
            if let Some(source) = src_opt.take() {
                return source;
            } else {
                panic!("{:?} already forced", symbol);
            }
        } else {
            panic!("{:?} not loaded", symbol);
        }
    }

    pub fn is_defined(&self, namespace: Symbol, name: Symbol) -> bool {
        self.get(namespace, name).is_some()
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

    pub fn set(&mut self, namespace: Symbol, name: Symbol, value: Value) -> Option<Value> {
        use ::std::mem::swap;

        let mut value = value;
        if let Some(existing) = self.get_mut(namespace, name) {
            swap(&mut value, existing);
            return Some(value);
        }

        self.globals.push((GlobalName {
            namespace: namespace,
            name: name
        }, value));
        return None;
    }
}
