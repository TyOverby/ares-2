use typed_arena::Arena;
use std::collections::HashMap;

mod error;
pub use self::error::BindingError;

use compiler::parse::{Ast, AstRef};
use ares_syntax::{Symbol, SymbolIntern};
use util::iterators_same;

// 2 concepts, Binders and Bound nodes
//
// Binders are for things that introduce bindings
//
// Bounds are for every node that needs binding

// Each bound node has a reference to the AST that it was from,
//

pub type BoundRef<'bound, 'ast> = &'bound Bound<'bound, 'ast>;

#[derive(Debug)]
pub enum Bound<'bound, 'ast: 'bound> {
    Literal(AstRef<'ast>),
    Symbol {
        symbol: Symbol,
        ast: AstRef<'ast>,
        source: SymbolBindSource,
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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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

    fn equals_sans_ast(&self, other: BoundRef<'bound, 'ast>) -> bool {
        match (self, other) {
            (&Bound::Literal(ref a), &Bound::Literal(ref b)) => {
                let res = a.equals_sans_span(b);
                res
            }

            (&Bound::Symbol {
                symbol: symbol_a,
                source: source_a,
                ..
            },
             &Bound::Symbol {
                symbol: symbol_b,
                source: source_b,
                ..
            }) => symbol_a == symbol_b && source_a == source_b,

            (&Bound::ListLit(ref list_a, _),
             &Bound::ListLit(ref list_b, _)) => {
                iterators_same(list_a.iter(), list_b.iter(), |a, b| {
                    Bound::equals_sans_ast(a, b)
                })
            }
            (&Bound::Add(ref la, ref ra, _), &Bound::Add(ref lb, ref rb, _)) => {
                Bound::equals_sans_ast(la, lb) && Bound::equals_sans_ast(ra, rb)
            }

            (&Bound::MapLit(ref list_a, _), &Bound::MapLit(ref list_b, _)) => {
                iterators_same(list_a.iter(), list_b.iter(), |&(k1, v1), &(k2, v2)| {
                    Bound::equals_sans_ast(k1, k2) && Bound::equals_sans_ast(v1, v2)
                })
            }
            (&Bound::IfExpression(ref a1, ref a2, ref a3, _),
             &Bound::IfExpression(ref b1, ref b2, ref b3, _)) => {
                a1.equals_sans_ast(b1) && a2.equals_sans_ast(b2) && a3.equals_sans_ast(b3)
            }
            (&Bound::Lambda{arg_symbols: ref args_a, body: ref body_a, bindings: ref bindings_a, ast: ref _asta },
             &Bound::Lambda{arg_symbols: ref args_b, body: ref body_b, bindings: ref bindings_b, ast: ref _astb }) => {
                let mut res = iterators_same(args_a.iter(), args_b.iter(), |a, b| a == b);
                res &= body_a.equals_sans_ast(body_b);
                res &= bindings_a == bindings_b;
                res
            }
            (&Bound::Define(symbol_a, source_a, bound_a, _),
             &Bound::Define(symbol_b, source_b, bound_b, _)) => {
                let defines_eq = symbol_a == symbol_b && source_a == source_b &&
                                 bound_a.equals_sans_ast(bound_b);
                defines_eq
            }
            (&Bound::Block(ref bodies_a, _),
             &Bound::Block(ref bodies_b, _)) => {
                iterators_same(bodies_a.iter(), bodies_b.iter(), |&a, &b| Bound::equals_sans_ast(a, b))
            }
            _ => false,
        }
    }

    fn format(&self, level: u32, interner: &SymbolIntern, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        use self::Bound::*;
        fn gen_indent(level: u32) -> String {
            let mut buf = String::new();
            for _ in 0 .. (level * 4) {
                buf.push(' ');
            }
            buf
        }
        fn label(name: &str, level: u32, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error>{
            try!(f.write_str(&gen_indent(level)));
            try!(f.write_str(name));
            f.write_str(":\n")
        }

        match self {
            &Literal(_l) => {
                try!(label("LITERAL", level, f));
                unimplemented!();
            }
            &Symbol { .. } => unimplemented!(),
            &ListLit(ref list, _) => {
                try!(label("LIST", level, f));
                for child in list {
                    try!(child.format(level + 1, interner, f));
                }
                Ok(())
            },
            &MapLit(ref pairs, _) => {
                try!(label("MAP", level, f));
                for &(ref k, ref v) in pairs {
                    try!(label("MAP-PAIR", level + 1, f));
                    try!(k.format(level + 2, interner, f));
                    try!(v.format(level + 2, interner, f));
                }
                Ok(())
            }
            &Add(ref l, ref r, _) => {
                try!(label("ADD", level, f));
                try!(l.format(level + 1, interner, f));
                try!(r.format(level + 1, interner, f));
                Ok(())
            },
            &Sub(ref l, ref r, _) => {
                try!(label("SUB", level, f));
                try!(l.format(level + 1, interner, f));
                try!(r.format(level + 1, interner, f));
                Ok(())
            },
            &Mul(ref l, ref r, _) => {
                try!(label("MUL", level, f));
                try!(l.format(level + 1, interner, f));
                try!(r.format(level + 1, interner, f));
                Ok(())
            },
            &Div(ref l, ref r, _) => {
                try!(label("DIV", level, f));
                try!(l.format(level + 1, interner, f));
                try!(r.format(level + 1, interner, f));
                Ok(())
            },
            &FnCall(ref rec, ref args, _) => {
                try!(label("FN-CALL", level, f));

                try!(label("RECEIVER", level + 1, f));
                try!(rec.format(level + 2, interner, f));

                try!(label("ARGS", level + 1, f));
                for arg in args {
                    try!(arg.format(level + 2, interner, f));
                }
                Ok(())
            }
            &IfExpression(ref cond, ref tru, ref fals, _) => {
                try!(label("IF-EXPRESSION", level, f));

                try!(label("COND", level + 1, f));
                try!(cond.format(level + 2, interner, f));

                try!(label("TRUE", level + 1, f));
                try!(tru.format(level+2, interner, f));

                try!(label("FALSE", level + 1, f));
                try!(fals.format(level+2, interner, f));
                Ok(())
            }
            &IfStatement(ref cond, ref tru, ref fals, _) => {
                try!(label("IF-STATEMENT", level, f));

                try!(label("COND", level + 1, f));
                try!(cond.format(level + 2, interner, f));

                try!(label("TRUE", level + 1, f));
                for statement in tru {
                    try!(statement.format(level+2, interner, f));
                }

                if let Some(fals) = fals.as_ref() {
                    try!(label("FALSE", level + 1, f));
                    for statement in fals {
                        try!(statement.format(level+2, interner, f));
                    }
                }
                Ok(())
            }
            &Lambda {..} => {
                unimplemented!();
            }
            &Block(ref bodies, _) => {
                try!(label("BLOCK", level, f));
                for body in bodies {
                    try!(body.format(level + 1, interner, f));
                }
                Ok(())
            }
            &Define(ref name, ref source, ref value, _) => {
                try!(label("DEFINE", level, f));

                try!(label("NAME", level + 1, f));
                try!(f.write_str(&gen_indent(level + 2)));
                try!(f.write_str(&interner.lookup_or_anon(*name)));
                try!(f.write_str("\n"));

                try!(label("SOURCE", level + 1, f));
                match *source {
                    SymbolBindSource::Arg(a) => {
                        try!(label("ARG", level + 2, f));
                        try!(f.write_str(&gen_indent(level + 3)));
                        try!(f.write_str(&a.to_string()));
                    }
                    SymbolBindSource::Upvar(a) => {
                        try!(label("UPVAR", level + 2, f));
                        try!(f.write_str(&gen_indent(level + 3)));
                        try!(f.write_str(&a.to_string()));
                    }
                    SymbolBindSource::LocalDefine(a) => {
                        try!(label("LOCAL-DEFINE", level + 2, f));
                        try!(f.write_str(&gen_indent(level + 3)));
                        try!(f.write_str(&a.to_string()));
                    }
                    SymbolBindSource::Global(a) => {
                        try!(label("GLOBAL", level + 2, f));
                        try!(f.write_str(&gen_indent(level + 3)));
                        try!(f.write_str(&interner.lookup_or_anon(a)));
                    }
                }

                try!(label("VALUE", level + 1, f));
                try!(value.format(level + 1, interner, f));
                Ok(())
            }
        }
    }
}

fn rearrange<T, E>(obj: Option<Result<T, E>>) -> Result<Option<T>, E> {
    match obj {
        None => Ok(None),
        Some(Ok(t)) => Ok(Some(t)),
        Some(Err(e)) => Err(e)
    }
}

#[cfg(test)]
mod test {
    use super::{Bound, SymbolBindSource, LambdaBindings};
    use compiler::parse::{Ast, Span};
    use compiler::parse::test::ok_parse_1;
    use typed_arena::Arena;

    fn str_eq(actual: &str, expected: &str) {
        use itertools::{Itertools, EitherOrBoth};
        fn isnt_whitespace(s: &&str) -> bool { !s.chars().all(char::is_whitespace) }
        let mut padding = None;
        for eob in actual.lines().filter(isnt_whitespace).zip_longest(
                   expected.lines().filter(isnt_whitespace)) {
            match eob {
                EitherOrBoth::Both(l, r) => {
                    let w_left = (l.len() - l.trim_left().len()) as i32;
                    let w_right = (r.len() - r.trim_left().len()) as i32;
                    let d = w_left - w_right;

                    if padding.is_none() {
                        padding = Some(d);
                    } else {
                        assert!(padding.unwrap() == d,
                                "indentation isn't the same at {} {}",
                                l, r);
                    }
                    assert_eq!(l.trim(), r.trim());
                }
                EitherOrBoth::Left(l) => {
                    panic!("actual has more lines: {}", l);
                }
                EitherOrBoth::Right(l) => {
                    panic!("expected has more lines: {}", l);
                }
            }
        }
    }

    fn bound_form(program: &str, bound_representation: &str) {
        let bind_arena = Arena::new();
        let (ast, mut interner) = ok_parse_1(program);
        let bound = Bound::bind_top(ast, &bind_arena, &mut interner).unwrap();

        let mut buffer = Vec::with_capacity(bound_representation.len());
        bound.format(0, &interner, &mut buffer);

        str_eq(String::from_bytes(buffer).unwrap(), bound_representation)
    }

    #[test]
    fn operators() {
        let parse_arena = Arena::new();
        let bind_arena = Arena::new();
        let (ast, mut interner) = ok_parse_1("1 + 2");
        let bound = Bound::bind_top(ast, &bind_arena, &mut interner);

        let should = bind_arena.alloc(Bound::Add(bind_arena.alloc(Bound::Literal(parse_arena.alloc(Ast::IntLit(1, Span::dummy())))),
                                                 bind_arena.alloc(Bound::Literal(parse_arena.alloc(Ast::IntLit(2, Span::dummy())))),
                                                 parse_arena.alloc(Ast::dummy())));

        let (ast, mut interner) = ok_parse_1("1 - 2");
        let bound = Bound::bind_top(ast, &bind_arena, &mut interner);

        let should = bind_arena.alloc(Bound::Sub(bind_arena.alloc(Bound::Literal(parse_arena.alloc(Ast::IntLit(1, Span::dummy())))),
                                                 bind_arena.alloc(Bound::Literal(parse_arena.alloc(Ast::IntLit(2, Span::dummy())))),
                                                 parse_arena.alloc(Ast::dummy())));
    }

    #[test]

    #[test]
    fn bind_lambda_one_arg() {
        let parse_arena = Arena::new();
        let bind_arena = Arena::new();
        let (ast, mut interner) = ok_parse_1("fn(a) { a }");
        let bound = Bound::bind_top(ast, &bind_arena, &mut interner);

        let should = bind_arena.alloc(Bound::Lambda {
            arg_symbols: vec![interner.intern("a")],
            body: bind_arena.alloc(Bound::Block(vec![bind_arena.alloc(Bound::Symbol {
                                   symbol: interner.intern("a"),
                                   ast: parse_arena.alloc(Ast::dummy()),
                                   source: SymbolBindSource::Arg(0),
                               })], parse_arena.alloc(Ast::dummy()))),
            ast: parse_arena.alloc(Ast::dummy()),
            bindings: LambdaBindings {
                bindings: vec![(interner.intern("a"), SymbolBindSource::Arg(0))]
                              .into_iter()
                              .collect(),
                num_args: 1,
                num_upvars: 0,
                num_declarations: 0,
            },
        });
        assert!(should.equals_sans_ast(bound.unwrap()));
    }

    #[test]
    fn bind_lambda_two_args() {
        let parse_arena = Arena::new();
        let bind_arena = Arena::new();
        let (ast, mut interner) = ok_parse_1("fn(a, b) { a + b }");
        let bound = Bound::bind_top(ast, &bind_arena, &mut interner);

        let should = bind_arena.alloc(Bound::Lambda {
            arg_symbols: vec![interner.intern("a"), interner.intern("b")],
            body: bind_arena.alloc(Bound::Block(vec![bind_arena.alloc(Bound::Add(
                                  bind_arena.alloc(Bound::Symbol {
                                      symbol: interner.intern("a"),
                                      ast: parse_arena.alloc(Ast::dummy()),
                                      source: SymbolBindSource::Arg(0)
                                  }),
                                  bind_arena.alloc(Bound::Symbol {
                                      symbol: interner.intern("b"),
                                      ast: parse_arena.alloc(Ast::dummy()),
                                      source: SymbolBindSource::Arg(1)
                                  }),
                                                           parse_arena.alloc(Ast::dummy())))], parse_arena.alloc(Ast::dummy()))),
            ast: parse_arena.alloc(Ast::dummy()),
            bindings: LambdaBindings {
                bindings: vec![(interner.intern("a"), SymbolBindSource::Arg(0)),
                               (interner.intern("b"), SymbolBindSource::Arg(1))]
                              .into_iter()
                              .collect(),
                num_args: 2,
                num_upvars: 0,
                num_declarations: 0,
            },
        });
        assert!(should.equals_sans_ast(bound.unwrap()));
    }

    /*
    #[test]
    fn bind_lambda_with_define() {
        let parse_arena = Arena::new();
        let bind_arena = Arena::new();
        let (ast, mut interner) = ok_parse_1("(lambda () (define x 5) x)", &parse_arena);
        let bound = Bound::bind_top(ast, &bind_arena, &mut interner);
        let x = interner.intern("x");

        let should = bind_arena.alloc(Bound::Lambda {
            arg_symbols: vec![],
            body: bind_arena.alloc(Bound::Block(vec![
                    bind_arena.alloc(
                        Bound::Define(x, SymbolBindSource::LocalDefine(0), bind_arena.alloc(
                                Bound::Literal(
                                    parse_arena.alloc(Ast::IntLit(5, Span::dummy())))
                                ), parse_arena.alloc(Ast::dummy()))),
                    bind_arena.alloc(
                        Bound::Symbol {
                            symbol: x,
                            ast: parse_arena.alloc(Ast::dummy()),
                            source: SymbolBindSource::LocalDefine(0)
                        })
                ], parse_arena.alloc(Ast::dummy()))),
            ast: parse_arena.alloc(Ast::dummy()),
            bindings: LambdaBindings {
                bindings: vec![(x, SymbolBindSource::LocalDefine(0))].into_iter().collect(),
                num_args: 0,
                num_upvars: 0,
                num_declarations: 1,
            },
        });

        println!("parsed {:#?}", bound);
        println!("manual {:#?}", should);

        assert!(should.equals_sans_ast(bound.unwrap()));
    }
    */
}
