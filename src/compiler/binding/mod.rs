use typed_arena::Arena;

use compiler::parse::Ast;
use vm::Symbol;

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
        stack_pos: usize
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

enum BoundSymbol {
    OnStack(u32),
    Global(Symbol)
}

struct BoundId(u32);

trait Binder {
    fn add_declaration(symbol: Symbol) -> BoundId;
    fn lookup(symbol: Symbol) -> BoundSymbol;
}

impl <'bound, 'ast: 'bound> Bound<'bound, 'ast> {
    fn bind(ast: &'ast Ast<'ast>, arena: &'bound Arena<Bound<'bound, 'ast>>) -> Bound<'bound, 'ast> {
        match ast {
            &Ast::BoolLit(_, _) |
                &Ast::StringLit(_, _) |
                &Ast::IntLit(_, _)  => {
                    Bound::Literal(ast)
                }
            &Ast::ListLit(ref elements, _) => {
                Bound::ListLit(
                    elements.iter()
                    .map(|element| arena.alloc(Bound::bind(element, arena)) as &_)
                    .collect(),
                    ast)
            }
            &Ast::MapLit(ref elements, _) => {
                Bound::MapLit(
                    elements.iter()
                    .map(|&(ref k, ref v)|
                         (arena.alloc(Bound::bind(k, arena)) as &_,
                         arena.alloc(Bound::bind(v, arena)) as &_))
                    .collect(),
                    ast)
            }
            &Ast::Symbol(ref _symbol, _) => {
                unimplemented!()
            }
            &Ast::Add(ref elements, _) => {
                Bound::Add(
                    elements.iter()
                    .map(|element| arena.alloc(Bound::bind(element, arena)) as &_)
                    .collect(),
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
                    elements.iter()
                    .map(|element| arena.alloc(Bound::bind(element, arena)) as &_)
                    .collect(),
                    ast)
            }
            &Ast::If(ref a, ref b, ref c, _) => {
                Bound::If(
                    arena.alloc(Bound::bind(a, arena)) as &_,
                    arena.alloc(Bound::bind(b, arena)) as &_,
                    arena.alloc(Bound::bind(c, arena)) as &_,
                    ast
                    )
            }
            &Ast::Lambda(ref _args, ref _bodies, _) => {
                unimplemented!();
            }
            _ => unimplemented!()
        }
    }
}
