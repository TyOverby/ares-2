use typed_arena::Arena;
use std::collections::HashMap;

mod error;
#[cfg(test)]
mod test;
pub use self::error::BindingError;

use compiler::parse::{Ast, AstRef};
use ares_syntax::{Symbol, SymbolIntern};

// 2 concepts, Binders and Bound nodes
//
// Binders are for things that introduce bindings
//
// Bound nodes are for every node that needs binding

pub type BoundRef<'bound, 'ast> = &'bound Bound<'bound, 'ast>;

#[derive(Debug)]
pub enum Bound<'bound, 'ast: 'bound> {
    Literal(AstRef<'ast>),
    Symbol {
        symbol: Symbol,
        source: SymbolBindSource,
        ast: AstRef<'ast>,
    },

    ListLit(Vec<BoundRef<'bound, 'ast>>, AstRef<'ast>),
    MapLit(Vec<(BoundRef<'bound, 'ast>, BoundRef<'bound, 'ast>)>, AstRef<'ast>),
    Add(BoundRef<'bound, 'ast>,
        BoundRef<'bound, 'ast>,
        AstRef<'ast>),
    Sub(BoundRef<'bound, 'ast>,
        BoundRef<'bound, 'ast>,
        AstRef<'ast>),
    Mul(BoundRef<'bound, 'ast>,
        BoundRef<'bound, 'ast>,
        AstRef<'ast>),
    Div(BoundRef<'bound, 'ast>,
        BoundRef<'bound, 'ast>,
        AstRef<'ast>),
    FnCall(BoundRef<'bound, 'ast>, Vec<BoundRef<'bound, 'ast>>, AstRef<'ast>),
    IfExpression(BoundRef<'bound, 'ast>,
       BoundRef<'bound, 'ast>,
       BoundRef<'bound, 'ast>,
       AstRef<'ast>),
    IfStatement(BoundRef<'bound, 'ast>,
       Vec<BoundRef<'bound, 'ast>>,
       Option<Vec<BoundRef<'bound, 'ast>>>,
       AstRef<'ast>),
    Lambda {
        arg_symbols: Vec<Symbol>,
        body: BoundRef<'bound, 'ast>,
        ast: AstRef<'ast>,
        bindings: LambdaBindings,
    },
    Block(Vec<BoundRef<'bound, 'ast>>, AstRef<'ast>),
    Define(Symbol, SymbolBindSource, BoundRef<'bound, 'ast>, AstRef<'ast>),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub enum SymbolBindSource {
    Arg(u32),
    Upvar(u32),
    LocalDefine(u32),
    Global(Symbol),
}

struct BuckStopsHereBinder;

#[derive(Debug, Eq, PartialEq)]
pub struct LambdaBindings {
    pub bindings: HashMap<Symbol, SymbolBindSource>,
    pub num_args: u32,
    pub num_upvars: u32,
    pub num_declarations: u32,
}

struct LambdaBinder<'a> {
    parent: &'a mut Binder,
    args: &'a Vec<Symbol>,
    bindings: LambdaBindings,
}

struct BlockBinder<'a> {
    parent: &'a mut Binder,
    symbol_map: HashMap<Symbol, Symbol>
}

trait Binder {
    fn add_declaration(&mut self, symbol: Symbol, interner: &mut SymbolIntern) -> SymbolBindSource;
    fn already_binds(&self, symbol: Symbol) -> bool;
    fn lookup(&self, symbol: Symbol) -> Option<SymbolBindSource>;
}

impl LambdaBindings {
    fn new() -> LambdaBindings {
        LambdaBindings {
            bindings: HashMap::new(),
            num_args: 0,
            num_upvars: 0,
            num_declarations: 0,
        }
    }

    pub fn compute_stack_offset(&self, bind_source: SymbolBindSource) -> u32 {
        match bind_source {
            SymbolBindSource::Arg(a) => a,
            SymbolBindSource::Upvar(_) => unimplemented!(),
            SymbolBindSource::LocalDefine(a) => self.num_args + self.num_upvars + a,
            SymbolBindSource::Global(_) => unimplemented!(),
        }
    }
}

impl<'a> LambdaBinder<'a> {
    fn new(parent: &'a mut Binder, args: &'a Vec<Symbol>) -> LambdaBinder<'a> {
        let mut bindings = LambdaBindings::new();
        for (i, arg_symbol) in args.iter().enumerate() {
            bindings.bindings.insert(arg_symbol.clone(), SymbolBindSource::Arg(i as u32));
        }
        bindings.num_args = args.len() as u32;

        LambdaBinder {
            parent: parent,
            args: args,
            bindings: bindings,
        }
    }
}

impl<'a> Binder for LambdaBinder<'a> {
    fn add_declaration(&mut self,
                       symbol: Symbol,
                       _interner: &mut SymbolIntern)
                       -> SymbolBindSource {
        assert!(!self.bindings.bindings.contains_key(&symbol));
        let source = SymbolBindSource::LocalDefine(self.bindings.num_declarations);
        self.bindings.bindings.insert(symbol, source);
        self.bindings.num_declarations += 1;
        source
    }

    fn already_binds(&self, symbol: Symbol) -> bool {
        self.bindings.bindings.contains_key(&symbol)
    }

    fn lookup(&self, symbol: Symbol) -> Option<SymbolBindSource> {
        self.bindings.bindings.get(&symbol).cloned()
    }
}

impl <'a> BlockBinder<'a> {
    fn new(parent: &'a mut Binder) -> BlockBinder<'a> {
        BlockBinder {
            parent: parent,
            symbol_map: HashMap::new(),
        }
    }
}

impl <'a> Binder for BlockBinder<'a> {
    fn add_declaration(&mut self, symbol: Symbol, interner: &mut SymbolIntern) -> SymbolBindSource {
        let mask = interner.gensym();
        self.symbol_map.insert(symbol, mask);
        self.parent.add_declaration(mask, interner)
    }

    fn already_binds(&self, symbol: Symbol) -> bool {
        self.symbol_map.contains_key(&symbol) || self.parent.already_binds(symbol)
    }

    fn lookup(&self, symbol: Symbol) -> Option<SymbolBindSource> {
        match self.symbol_map.get(&symbol) {
            Some(&translated) => self.parent.lookup(translated),
            None => self.parent.lookup(symbol)
        }
    }
}

impl Binder for BuckStopsHereBinder {
    fn add_declaration(&mut self,
                       symbol: Symbol,
                       _interner: &mut SymbolIntern)
                       -> SymbolBindSource {
        SymbolBindSource::Global(symbol)
    }

    fn already_binds(&self, _symbol: Symbol) -> bool {
        false
    }

    fn lookup(&self, symbol: Symbol) -> Option<SymbolBindSource> {
        Some(SymbolBindSource::Global(symbol))
    }
}

impl<'bound, 'ast: 'bound> Bound<'bound, 'ast> {
    pub fn bind_top(ast: AstRef<'ast>,
                    arena: &'bound Arena<Bound<'bound, 'ast>>,
                    interner: &mut SymbolIntern)
                    -> Result<BoundRef<'bound, 'ast>, BindingError> {
        let mut buck = BuckStopsHereBinder;
        Bound::bind(ast, arena, &mut buck, interner)
    }

    fn bind_all<I>(asts: I, 
            arena: &'bound Arena<Bound<'bound, 'ast>>,
            binder: &mut Binder,
            interner: &mut SymbolIntern)
            -> Result<Vec<BoundRef<'bound, 'ast>>, BindingError>
    where I: IntoIterator<Item=AstRef<'ast>> {
        let mut out_ok = vec![];
        let mut out_err = vec![];
        for ast in asts {
            match Bound::bind(ast, arena, binder, interner) {
                Ok(o) => out_ok.push(o),
                Err(e) => out_err.push(e),
            }
        }

        if !out_err.is_empty() {
            Err(BindingError::Multiple(out_err))
        } else {
            Ok(out_ok)
        }
    }

    fn bind(ast: AstRef<'ast>,
            arena: &'bound Arena<Bound<'bound, 'ast>>,
            binder: &mut Binder,
            interner: &mut SymbolIntern)
            -> Result<BoundRef<'bound, 'ast>, BindingError> {
        Ok(arena.alloc(match ast {
            &Ast::BoolLit(_, _) |
            &Ast::StringLit(_, _) |
            &Ast::FloatLit(_, _) |
            &Ast::SymbolLit(_, _) |
            &Ast::IntLit(_, _) => Bound::Literal(ast),
            &Ast::ListLit(ref elements, _) => {
                Bound::ListLit(try!(elements.iter()
                                            .map(|element| {
                                                Bound::bind(element, arena, binder, interner)
                                            })
                                            .collect::<Result<Vec<_>, _>>()),
                               ast)
            }
            &Ast::MapLit(ref elements, _) => {
                let mut bound = Vec::with_capacity(elements.len());
                for &(ref k, ref v) in elements {
                    let k = try!(Bound::bind(k, arena, binder, interner));
                    let v = try!(Bound::bind(v, arena, binder, interner));
                    bound.push((k, v));
                }
                Bound::MapLit(bound, ast)
            }
            &Ast::Identifier(symbol, span) => {
                let source = match binder.lookup(symbol) {
                    Some(source) => source,
                    None => return Err(BindingError::CouldNotBind(symbol, span)),
                };

                Bound::Symbol {
                    symbol: symbol,
                    ast: ast,
                    source: source,
                }
            }
            &Ast::Add(ref left, ref right, _) => {
                Bound::Add(try!(Bound::bind(left, arena, binder, interner)),
                           try!(Bound::bind(right, arena, binder, interner)),
                           ast)
            }
            &Ast::Sub(ref left, ref right, _) => {
                Bound::Sub(try!(Bound::bind(left, arena, binder, interner)),
                           try!(Bound::bind(right, arena, binder, interner)),
                           ast)
            }
            &Ast::Mul(ref left, ref right, _) => {
                Bound::Mul(try!(Bound::bind(left, arena, binder, interner)),
                           try!(Bound::bind(right, arena, binder, interner)),
                           ast)
            }
            &Ast::Div(ref left, ref right, _) => {
                Bound::Div(try!(Bound::bind(left, arena, binder, interner)),
                           try!(Bound::bind(right, arena, binder, interner)),
                           ast)
            }
            &Ast::FnCall(ref receiver, ref arguments, _) => {
                let bound_receiver = try!(Bound::bind(receiver, arena, binder, interner));
                let bound_arguments = try!(Bound::bind_all(arguments, arena, binder, interner));
                Bound::FnCall(bound_receiver, bound_arguments, ast)
            }
            &Ast::IfExpression(ref a, ref b, ref c, _) => {
                Bound::IfExpression(try!(Bound::bind(a, arena, binder, interner)) as &_,
                          try!(Bound::bind(b, arena, binder, interner)) as &_,
                          try!(Bound::bind(c, arena, binder, interner)) as &_,
                          ast)
            }
            &Ast::IfStatement(ref a, ref b, ref c, _) => {
                Bound::IfStatement(try!(Bound::bind(a, arena, binder, interner)) as &_,
                          try!(Bound::bind_all(b.iter(), arena, binder, interner)),
                          try!(rearrange(c.as_ref().map(|c| Bound::bind_all(c.iter(), arena, binder, interner)))),
                          ast)
            }
            &Ast::Closure(ref _name, ref args, ref body_block, _) => {
                // TODO: Bind name to "this function"
                assert!(args.len() == 1);
                let mut new_binder = LambdaBinder::new(binder, &args[0]);
                let bound_body = try!(Bound::bind(body_block, arena, &mut new_binder, interner));
                Bound::Lambda {
                    arg_symbols: args[0].clone(),
                    body: bound_body,
                    ast: ast,
                    bindings: new_binder.bindings,
                }
            }
            &Ast::Block(ref bodies, _) => {
                let mut new_binder = BlockBinder::new(binder);
                let bound_bodies = try!(Bound::bind_all(bodies, arena, &mut new_binder, interner));
                Bound::Block(bound_bodies, ast)
            }
            &Ast::Define(symbol, value, _) => {
                if binder.already_binds(symbol) {
                    return Err(BindingError::AlreadyDefined(symbol));
                }
                let source = binder.add_declaration(symbol, interner);
                let bound_value = try!(Bound::bind(value, arena, binder, interner));
                Bound::Define(symbol, source, bound_value, ast)
            }
        }))
    }
}

fn rearrange<T, E>(obj: Option<Result<T, E>>) -> Result<Option<T>, E> {
    match obj {
        None => Ok(None),
        Some(Ok(t)) => Ok(Some(t)),
        Some(Err(e)) => Err(e)
    }
}

