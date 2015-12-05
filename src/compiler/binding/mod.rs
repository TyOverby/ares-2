use typed_arena::Arena;
use std::collections::HashMap;

mod error;
pub use self::error::BindingError;

use compiler::parse::Ast;
use vm::Symbol;
use vm::SymbolIntern;
use util::iterators_same;

// 2 concepts, Binders and Bound nodes
//
// Binders are for things that introduce bindings
//
// Bounds are for every node that needs binding

//
// Each bound node has a reference to the AST that it was from,
//

enum Bound<'bound, 'ast: 'bound> {
    Literal(&'ast Ast<'ast>),
    Symbol {
        symbol: Symbol,
        ast: &'ast Ast<'ast>,
        source: SymbolBindSource
    },

    ListLit(Vec<&'bound Bound<'bound, 'ast>>, &'ast Ast<'ast>),
    MapLit(Vec<(&'bound Bound<'bound, 'ast>, &'bound Bound<'bound, 'ast>)>, &'ast Ast<'ast>),
    Add(Vec<&'bound Bound<'bound, 'ast>>, &'ast Ast<'ast>),
    Quote {
        quoting: &'ast Ast<'ast>,
        ast: &'ast Ast<'ast>
    },
    List(Vec<&'bound Bound<'bound, 'ast>>, &'ast Ast<'ast>),
    If(&'bound Bound<'bound, 'ast>,
       &'bound Bound<'bound, 'ast>,
       &'bound Bound<'bound, 'ast>,
       &'ast Ast<'ast>),
       Lambda(Vec<Symbol>, Vec<&'bound Bound<'bound, 'ast>>, &'ast Ast<'ast>)
}

#[derive(Clone, PartialEq, Eq)]
enum SymbolBindSource {
    Arg(u32),
    Upvar(u32),
    LocalDefine(u32),
    Global(Symbol),
}

struct BuckStopsHereBinder;

struct LambdaBinder<'a> {
    parent: &'a mut Binder,
    args: Vec<Symbol>,
    bindings: HashMap<Symbol, SymbolBindSource>
}

trait Binder {
    fn add_declaration(&mut self, symbol: Symbol, interner: &mut SymbolIntern);
    fn already_binds(&self, symbol: Symbol) -> bool;
    fn lookup(&self, symbol: Symbol) -> Option<SymbolBindSource>;
}

impl <'a> LambdaBinder<'a> {
    fn new(parent: &'a mut Binder, args: Vec<Symbol>) -> LambdaBinder<'a> {
        let mut bindings = HashMap::new();
        for (i, arg_symbol) in args.iter().cloned().enumerate() {
            bindings.insert(arg_symbol, SymbolBindSource::Arg(i as u32));
        }

        LambdaBinder {
            parent: parent,
            args: args,
            bindings: bindings,
        }
    }
}

impl <'a> Binder for LambdaBinder<'a> {
    fn add_declaration(&mut self, _symbol: Symbol, _interner: &mut SymbolIntern) {
        // ignore for now
    }

    fn already_binds(&self, symbol: Symbol) -> bool {
        self.bindings.contains_key(&symbol)
    }

    fn lookup(&self, symbol: Symbol) -> Option<SymbolBindSource> {
        self.bindings.get(&symbol).cloned()
    }
}

impl Binder for BuckStopsHereBinder {
    fn add_declaration(&mut self, _symbol: Symbol, _interner: &mut SymbolIntern) { }

    fn already_binds(&self, _symbol: Symbol) -> bool { false }

    fn lookup(&self, _symbol: Symbol) -> Option<SymbolBindSource> {
        None
    }
}

impl <'bound, 'ast: 'bound> Bound<'bound, 'ast> {
    fn bind(ast: &'ast Ast<'ast>,
            arena: &'bound Arena<Bound<'bound, 'ast>>,
            binder: &mut Binder,
            interner: &mut SymbolIntern) -> Result<&'bound Bound<'bound, 'ast>, BindingError> {
                Ok(arena.alloc(match ast {
                    &Ast::BoolLit(_, _) |
                    &Ast::StringLit(_, _) |
                    &Ast::IntLit(_, _)  => {
                        Bound::Literal(ast)
                    }
                &Ast::ListLit(ref elements, _) => {
                    Bound::ListLit(
                        try!(elements.iter()
                             .map(|element| Bound::bind(element, arena, binder, interner))
                             .collect::<Result<Vec<_>, _>>()),
                             ast)
                }
                &Ast::MapLit(ref elements, _) => {
                    Bound::MapLit(
                        try!(elements.iter()
                             .map(|&(ref k, ref v)| {
                                 match (Bound::bind(k, arena, binder, interner),
                                 Bound::bind(v, arena, binder, interner)) {
                                     (Ok(k), Ok(v)) => Ok((k, v)),
                                     (Err(e), _) => Err(e),
                                     (_, Err(e)) => Err(e),
                                 }
                             })
                             .collect::<Result<Vec<_>, _>>()),
                             ast)
                }
                &Ast::Symbol(symbol, span) => {
                    let source = match binder.lookup(symbol) {
                        Some(source) => source,
                        None => return Err(BindingError::CouldNotBind(symbol, span))
                    };

                    Bound::Symbol {
                        symbol: symbol,
                        ast: ast,
                        source: source
                    }
                }
                &Ast::Add(ref elements, _) => {
                    Bound::Add(
                        try!(elements.iter()
                             .map(|element| Bound::bind(element, arena, binder, interner))
                             .collect::<Result<Vec<_>, _>>()),
                             ast)
                }
                &Ast::Quote(ref q, _) => {
                    Bound::Quote {
                        quoting: q,
                        ast: ast
                    }
                }
                &Ast::List(ref elements, _) => {
                    Bound::List(
                        try!(elements.iter()
                             .map(|element| Bound::bind(element, arena, binder, interner))
                             .collect()),
                             ast)
                }
                &Ast::If(ref a, ref b, ref c, _) => {
                    Bound::If(
                        try!(Bound::bind(a, arena, binder, interner)) as &_,
                        try!(Bound::bind(b, arena, binder, interner)) as &_,
                        try!(Bound::bind(c, arena, binder, interner)) as &_,
                        ast
                        )
                }
                &Ast::Lambda(ref _args, ref _bodies, _) => {
                    unimplemented!();
                }
                _ => unimplemented!()
                }))
            }

    fn equals_sans_ast(&self, other: &'bound Bound<'bound, 'ast>) -> bool {
        match (self, other) {
            (&Bound::Literal(ref a), &Bound::Literal(ref b)) => a.equals_sans_span(b),

            (&Bound::Symbol {
                symbol: symbol_a,
                source: ref source_a,
                ..
            }, &Bound::Symbol {
                symbol: symbol_b,
                source: ref source_b,
                ..
            }) => symbol_a == symbol_b && source_a == source_b,

            (&Bound::ListLit(ref list_a, _), &Bound::ListLit(ref list_b, _)) |
            (&Bound::List(ref list_a, _), &Bound::List(ref list_b, _)) |
            (&Bound::Add(ref list_a, _), &Bound::Add(ref list_b, _)) => {
                iterators_same(list_a.iter().cloned(), list_b.iter().cloned(), Bound::equals_sans_ast)
            }

            (&Bound::MapLit(ref list_a, _), &Bound::MapLit(ref list_b, _)) => {
                iterators_same(list_a.iter().cloned(), list_b.iter().cloned(), |(ref k1, ref v1), (ref k2, ref v2)| {
                    Bound::equals_sans_ast(k1, k2) && Bound::equals_sans_ast(v1, v2)
                })
            }
            (&Bound::Quote { quoting: quoting_a, ..  }, &Bound::Quote { quoting: quoting_b, ..  }) => {
                quoting_a.equals_sans_span(quoting_b)
            }
            (&Bound::If(ref a1, ref a2, ref a3, _), &Bound::If(ref b1, ref b2, ref b3, _)) =>
                a1.equals_sans_ast(b1) && a2.equals_sans_ast(b2) && a3.equals_sans_ast(b3),
            (&Bound::Lambda(ref args_a, ref bodies_a, _), &Bound::Lambda(ref args_b, ref bodies_b, _)) => {
                iterators_same(args_a.iter().cloned(), args_b.iter().cloned(), |a, b| a == b) &&
                    iterators_same(bodies_a.iter().cloned(), bodies_b.iter().cloned(), Bound::equals_sans_ast)
            }
            _ => false
        }
    }
}
