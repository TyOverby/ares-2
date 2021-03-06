use typed_arena::Arena;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::cell::Cell;

mod error;
pub use self::error::BindingError;

use compiler::parse::{Ast, AstRef};
use vm::Modules;
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
    ListAccess(BoundRef<'bound, 'ast>, BoundRef<'bound, 'ast>, AstRef<'ast>),
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
    LessThan(
        BoundRef<'bound, 'ast>,
        BoundRef<'bound, 'ast>,
        AstRef<'ast>),
    LessThanOrEqual(
        BoundRef<'bound, 'ast>,
        BoundRef<'bound, 'ast>,
        AstRef<'ast>),
    GreaterThan(
        BoundRef<'bound, 'ast>,
        BoundRef<'bound, 'ast>,
        AstRef<'ast>),
    GreaterThanOrEqual(
        BoundRef<'bound, 'ast>,
        BoundRef<'bound, 'ast>,
        AstRef<'ast>),
    Equal(
        BoundRef<'bound, 'ast>,
        BoundRef<'bound, 'ast>,
        AstRef<'ast>),
    NotEqual(
        BoundRef<'bound, 'ast>,
        BoundRef<'bound, 'ast>,
        AstRef<'ast>),
    FnCall(BoundRef<'bound, 'ast>, Vec<BoundRef<'bound, 'ast>>, AstRef<'ast>),
    IfExpression(BoundRef<'bound, 'ast>,
       BoundRef<'bound, 'ast>,
       BoundRef<'bound, 'ast>,
       AstRef<'ast>),
    IfStatement(BoundRef<'bound, 'ast>,
       BoundRef<'bound, 'ast>,
       Option<BoundRef<'bound, 'ast>>,
       AstRef<'ast>),
    Lambda {
        arg_symbols: Vec<Symbol>,
        body: BoundRef<'bound, 'ast>,
        ast: AstRef<'ast>,
        bindings: LambdaBindings,
        upvar_list: Vec<SymbolBindSource>,
        is_shifter: Cell<bool>,
    },
    BlockExpression(Vec<BoundRef<'bound, 'ast>>, AstRef<'ast>),
    BlockStatement(Vec<BoundRef<'bound, 'ast>>, AstRef<'ast>),
    Assign(Symbol, SymbolBindSource, BoundRef<'bound, 'ast>, AstRef<'ast>),
    Define(Symbol, SymbolBindSource, BoundRef<'bound, 'ast>, AstRef<'ast>),
    Shift(Vec<BoundRef<'bound, 'ast>>, BoundRef<'bound, 'ast>, AstRef<'ast>),
    Reset(Vec<BoundRef<'bound, 'ast>>, BoundRef<'bound, 'ast>, AstRef<'ast>),
    Import {
        defines: Vec<Bound<'bound, 'ast>>,
        namespace: Symbol,
        version: String,
        ast: AstRef<'ast>,
    },
    ImportThis {
        name: Symbol,
        namespace: Symbol,
        version: String,
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub enum SymbolBindSource {
    Arg {
        position: u32,
        upvar: Rc<Cell<bool>>,
    },
    Upvar {
        position: u32,
        upvar: Rc<Cell<bool>>,
    },
    LocalDefine {
        position: u32,
        upvar: Rc<Cell<bool>>,
    },
    Global(Symbol),
}

impl SymbolBindSource {
    fn set_as_upvar(&self)  {
        match self {
            &SymbolBindSource::Arg{ref upvar, ..} |
            &SymbolBindSource::Upvar{ref upvar, ..} |
            &SymbolBindSource::LocalDefine{ref upvar, ..}  => {
                upvar.set(true)
            }
            _ => {}
        }
    }

    fn new_arg(position: u32) -> SymbolBindSource {
        SymbolBindSource::Arg {
            position: position,
            upvar: Rc::new(Cell::new(false)),
        }
    }
    fn new_upvar(position: u32) -> SymbolBindSource {
        SymbolBindSource::Upvar {
            position: position,
            upvar: Rc::new(Cell::new(false)),
        }
    }
    fn new_local_define(position: u32) -> SymbolBindSource {
        SymbolBindSource::LocalDefine {
            position: position,
            upvar: Rc::new(Cell::new(false)),
        }
    }
    fn new_global(symbol: Symbol) -> SymbolBindSource {
        SymbolBindSource::Global(symbol)
    }
}

struct BuckStopsHereBinder<'a> {
    globals: HashSet<Symbol>,
    modules: Option<&'a Modules>,
    my_module: Symbol
}

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
    upvar_list: Vec<SymbolBindSource>,
}

struct BlockBinder<'a> {
    parent: &'a mut Binder,
    symbol_map: HashMap<Symbol, Symbol>,
}

trait Binder {
    fn add_declaration(&mut self, symbol: Symbol, interner: &mut SymbolIntern) -> SymbolBindSource;
    fn already_binds(&self, symbol: Symbol) -> bool;
    fn lookup(&mut self, symbol: Symbol, from_closure: bool) -> Option<SymbolBindSource>;
    fn module(&self) -> Symbol;
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

    pub fn compute_stack_offset(&self, bind_source: &SymbolBindSource) -> u32 {
        match bind_source {
            &SymbolBindSource::Arg { position, .. } => position,
            &SymbolBindSource::Upvar { position, .. } => self.num_args + position,
            &SymbolBindSource::LocalDefine { position, .. } => self.num_args + self.num_upvars + position,
            &SymbolBindSource::Global(_) => panic!("no stack offset for global"),
        }
    }
}

impl<'a> LambdaBinder<'a> {
    fn new(parent: &'a mut Binder, args: &'a Vec<Symbol>) -> LambdaBinder<'a> {
        let mut bindings = LambdaBindings::new();
        for (i, arg_symbol) in args.iter().enumerate() {
            bindings.bindings.insert(arg_symbol.clone(),
                SymbolBindSource::new_arg(i as u32));
        }
        bindings.num_args = args.len() as u32;

        LambdaBinder {
            parent: parent,
            args: args,
            bindings: bindings,
            upvar_list: Vec::new(),
        }
    }
}

impl<'a> Binder for LambdaBinder<'a> {
    fn add_declaration(&mut self,
                       symbol: Symbol,
                       _interner: &mut SymbolIntern)
                       -> SymbolBindSource {
        assert!(!self.bindings.bindings.contains_key(&symbol));
        let source = SymbolBindSource::new_local_define(self.bindings.num_declarations);
        self.bindings.bindings.insert(symbol, source.clone());
        self.bindings.num_declarations += 1;
        source
    }

    fn already_binds(&self, symbol: Symbol) -> bool {
        self.bindings.bindings.contains_key(&symbol)
    }

    fn lookup(&mut self, symbol: Symbol, from_closure: bool) -> Option<SymbolBindSource> {
        if let Some(local_binding) = self.bindings.bindings.get(&symbol).cloned() {
            if from_closure {
                local_binding.set_as_upvar();
            }
            Some(local_binding)
        } else {
            match self.parent.lookup(symbol, true) {
                None => None,
                Some(g@SymbolBindSource::Global(_)) => Some(g),
                Some(other) => {
                    let upvar_position = self.upvar_list.len();
                    self.upvar_list.push(other);
                    self.bindings.num_upvars += 1;
                    let source = SymbolBindSource::new_upvar(upvar_position as u32);
                    self.bindings.bindings.insert(symbol, source.clone());
                    Some(source)
                }
            }
        }
    }

    fn module(&self) -> Symbol {
        self.parent.module()
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

    fn lookup(&mut self, symbol: Symbol, from_closure: bool) -> Option<SymbolBindSource> {
        match self.symbol_map.get(&symbol) {
            Some(&translated) => self.parent.lookup(translated, from_closure),
            None => self.parent.lookup(symbol, from_closure)
        }
    }

    fn module(&self) -> Symbol {
        self.parent.module()
    }
}

impl <'a> Binder for BuckStopsHereBinder<'a> {
    fn add_declaration(&mut self,
                       symbol: Symbol,
                       _interner: &mut SymbolIntern)
                       -> SymbolBindSource {
        self.globals.insert(symbol);
        SymbolBindSource::Global(symbol)
    }

    fn already_binds(&self, symbol: Symbol) -> bool {
        self.globals.contains(&symbol)
    }

    fn lookup(&mut self, symbol: Symbol, _from_closure: bool) -> Option<SymbolBindSource> {
        if let Some(modules) = self.modules {
            if modules.is_defined(self.my_module, symbol) {
                return Some(SymbolBindSource::Global(symbol))
            }
        }

        if self.already_binds(symbol) {
            Some(SymbolBindSource::Global(symbol))
        } else {
            None
        }
    }

    fn module(&self) -> Symbol {
        self.my_module
    }
}

impl<'bound, 'ast: 'bound> Bound<'bound, 'ast> {
    pub fn bind_top(asts: &[AstRef<'ast>],
                    arena: &'bound Arena<Bound<'bound, 'ast>>,
                    modules: Option<&Modules>,
                    interner: &mut SymbolIntern)
                    -> Result<Vec<BoundRef<'bound, 'ast>>, BindingError> {
        let mut buck = BuckStopsHereBinder {
            globals: HashSet::new(),
            modules: modules,
            // TODO: Pass this in to binding for different namespaces
            my_module: interner.precomputed.default_namespace, 
        };

       asts.iter().map(|ast| Bound::bind(ast, arena, &mut buck, modules, interner)).collect()
    }

    fn bind_all<I>(asts: I, 
            arena: &'bound Arena<Bound<'bound, 'ast>>,
            binder: &mut Binder,
            modules: Option<&Modules>,
            interner: &mut SymbolIntern)
            -> Result<Vec<BoundRef<'bound, 'ast>>, BindingError>
    where I: IntoIterator<Item=AstRef<'ast>> {
        let mut out_ok = vec![];
        let mut out_err = vec![];
        for ast in asts {
            match Bound::bind(ast, arena, binder, modules, interner) {
                Ok(o) => out_ok.push(o),
                Err(e) => out_err.push(e),
            }
        }

        match out_err.len() {
            0 => Ok(out_ok),
            1 => Err(out_err.pop().unwrap()),
            _ => Err(BindingError::Multiple(out_err))
        }
    }

    fn bind(ast: AstRef<'ast>,
            arena: &'bound Arena<Bound<'bound, 'ast>>,
            binder: &mut Binder,
            modules: Option<&Modules>,
            interner: &mut SymbolIntern)
            -> Result<BoundRef<'bound, 'ast>, BindingError> {
        Ok(arena.alloc(match ast {
            &Ast::BoolLit(_, _) |
            &Ast::StringLit(_, _) |
            &Ast::FloatLit(_, _) |
            &Ast::SymbolLit(_, _) |
            &Ast::NilLit(_) |
            &Ast::IntLit(_, _) => Bound::Literal(ast),
            &Ast::ListLit(ref elements, _) => {
                Bound::ListLit(elements.iter()
                                            .map(|element| {
                                                Bound::bind(element, arena, binder, modules, interner)
                                            })
                                            .collect::<Result<Vec<_>, _>>()?,
                               ast)
            }
            &Ast::ListAccess(ref target, ref index, _) => {
                Bound::ListAccess(
                    Bound::bind(target, arena, binder, modules, interner)?,
                    Bound::bind(index, arena, binder, modules, interner)?,
                    ast)
            }
            &Ast::MapLit(ref elements, _) => {
                let mut bound = Vec::with_capacity(elements.len());
                for &(ref k, ref v) in elements {
                    let k = Bound::bind(k, arena, binder, modules, interner)?;
                    let v = Bound::bind(v, arena, binder, modules, interner)?;
                    bound.push((k, v));
                }
                Bound::MapLit(bound, ast)
            }
            &Ast::Identifier(symbol, span) => {
                let source = match binder.lookup(symbol, false) {
                    Some(source) => source,
                    None => return Err(BindingError::CouldNotBind(symbol, span)),
                };

                //println!("source for {} is {:?}", interner.lookup_or_anon(symbol), source);

                Bound::Symbol {
                    symbol: symbol,
                    ast: ast,
                    source: source,
                }
            }
            &Ast::Add(ref left, ref right, _) => {
                Bound::Add(Bound::bind(left, arena, binder, modules, interner)?,
                           Bound::bind(right, arena, binder, modules, interner)?,
                           ast)
            }
            &Ast::Sub(ref left, ref right, _) => {
                Bound::Sub(Bound::bind(left, arena, binder, modules, interner)?,
                           Bound::bind(right, arena, binder, modules, interner)?,
                           ast)
            }
            &Ast::Mul(ref left, ref right, _) => {
                Bound::Mul(Bound::bind(left, arena, binder, modules, interner)?,
                           Bound::bind(right, arena, binder, modules, interner)?,
                           ast)
            }
            &Ast::Div(ref left, ref right, _) => {
                Bound::Div(Bound::bind(left, arena, binder, modules, interner)?,
                           Bound::bind(right, arena, binder, modules, interner)?,
                           ast)
            }
            &Ast::LessThan(ref left, ref right, _) => {
                Bound::LessThan(Bound::bind(left, arena, binder, modules, interner)?,
                                Bound::bind(right, arena, binder, modules, interner)?,
                                ast)
            }
            &Ast::LessThanOrEqual(ref left, ref right, _) => {
                Bound::LessThanOrEqual(Bound::bind(left, arena, binder, modules, interner)?,
                                Bound::bind(right, arena, binder, modules, interner)?,
                                ast)
            }
            &Ast::GreaterThan(ref left, ref right, _) => {
                Bound::GreaterThan(Bound::bind(left, arena, binder, modules, interner)?,
                                Bound::bind(right, arena, binder, modules, interner)?,
                                ast)
            }
            &Ast::GreaterThanOrEqual(ref left, ref right, _) => {
                Bound::GreaterThanOrEqual(Bound::bind(left, arena, binder, modules, interner)?,
                                Bound::bind(right, arena, binder, modules, interner)?,
                                ast)
            }
            &Ast::Equal(ref left, ref right, _) => {
                Bound::Equal(Bound::bind(left, arena, binder, modules, interner)?,
                                Bound::bind(right, arena, binder, modules, interner)?,
                                ast)
            }
            &Ast::NotEqual(ref left, ref right, _) => {
                Bound::NotEqual(Bound::bind(left, arena, binder, modules, interner)?,
                                Bound::bind(right, arena, binder, modules, interner)?,
                                ast)
            }
            &Ast::FnCall(ref receiver, ref arguments, _) => {
                let bound_receiver = Bound::bind(receiver, arena, binder, modules, interner)?;
                let bound_arguments = Bound::bind_all(arguments, arena, binder, modules, interner)?;
                Bound::FnCall(bound_receiver, bound_arguments, ast)
            }
            &Ast::IfExpression(ref a, ref b, ref c, _) => {
                Bound::IfExpression(Bound::bind(a, arena, binder, modules, interner)? as &_,
                          Bound::bind(b, arena, binder, modules, interner)? as &_,
                          Bound::bind(c, arena, binder, modules, interner)? as &_,
                          ast)
            }
            &Ast::IfStatement(ref a, ref b, ref c, _) => {
                Bound::IfStatement(Bound::bind(a, arena, binder, modules, interner)? as &_,
                          Bound::bind(b, arena, binder, modules, interner)?,
                          rearrange(c.map(|c| Bound::bind(c, arena, binder, modules, interner)))?,
                          ast)
            }
            &Ast::Closure(ref _name, ref args, ref body_block, _) => {
                // TODO: Bind name to "this function"
                // args.len() must be 1 for now because that's how many argument lists there are.
                assert!(args.len() == 1);
                let mut new_binder = LambdaBinder::new(binder, &args[0]);
                let bound_body = Bound::bind(body_block, arena, &mut new_binder, modules, interner)?;
                Bound::Lambda {
                    arg_symbols: args[0].clone(),
                    body: bound_body,
                    ast: ast,
                    bindings: new_binder.bindings,
                    upvar_list: new_binder.upvar_list,
                    is_shifter: Cell::new(false),
                }
            }
            &Ast::BlockExpression(ref bodies, _) => {
                let mut new_binder = BlockBinder::new(binder);
                let bound_bodies = Bound::bind_all(bodies, arena, &mut new_binder, modules, interner)?;
                Bound::BlockExpression(bound_bodies, ast)
            }
            &Ast::BlockStatement(ref bodies, _) => {
                let mut new_binder = BlockBinder::new(binder);
                let bound_bodies = Bound::bind_all(bodies, arena, &mut new_binder, modules, interner)?;
                Bound::BlockStatement(bound_bodies, ast)
            }
            &Ast::Assign(symbol, value, _) => {
                match binder.lookup(symbol, false) {
                    Some(source@SymbolBindSource::LocalDefine{..}) |
                    Some(source@SymbolBindSource::Arg{..}) |
                    Some(source@SymbolBindSource::Upvar{..}) => {
                        let value = Bound::bind(value, arena, binder, modules, interner)?;
                        Bound::Assign(symbol, source, value, ast)
                    }
                    Some(source@SymbolBindSource::Global(_)) => {
                        let value = Bound::bind(value, arena, binder, modules, interner)?;
                        Bound::Assign(symbol, source, value, ast)
                    }
                    None => return Err(BindingError::CouldNotBind(symbol, ast.span()))
                }
            }
            &Ast::Define(symbol, value, _) => {
                if binder.already_binds(symbol) {
                    return Err(BindingError::AlreadyDefined(symbol));
                }
                let source = binder.add_declaration(symbol, interner);
                let bound_value = Bound::bind(value, arena, binder, modules, interner)?;
                Bound::Define(symbol, source, bound_value, ast)
            }
            &Ast::Shift(ref symbols, ref closure, _) => {
                let bound_symbols = Bound::bind_all(symbols, arena, binder, modules, interner)?;
                let bound_closure = Bound::bind(closure, arena, binder, modules, interner)?;
                if let &Bound::Lambda{ ref is_shifter, .. } = bound_closure {
                    is_shifter.set(true);
                } else {
                    panic!("shift called without lambda body");
                }
                Bound::Shift(bound_symbols, bound_closure, ast)
            }
            &Ast::Reset(ref symbols, ref closure, _) => {
                let bound_symbols = Bound::bind_all(symbols, arena, binder, modules, interner)?;
                let bound_closure = Bound::bind(closure, arena, binder, modules, interner)?;
                Bound::Reset(bound_symbols, bound_closure, ast)
            }
            &Ast::Import(ref names, ref namespace, ref version, _) => {
                let mut sources = vec![];
                for name in names.iter().cloned() {
                    if binder.already_binds(name) {
                        return Err(BindingError::AlreadyDefined(name));
                    }

                    let source = binder.add_declaration(name, interner);
                    sources.push(source);
                }

                let version = match (*version).clone() {
                    Ast::StringLit(s, _) => s,
                    _ => panic!("version not string"),
                };

                Bound::Import {
                    defines: names.iter().cloned().zip(sources.into_iter()).map(|(name, source)| {
                        let import_this = Bound::ImportThis {
                            name: name,
                            namespace: *namespace,
                            version: version.clone(),
                        };

                        Bound::Define(name, source, arena.alloc(import_this), ast)
                    }).collect(),

                    namespace: namespace.clone(),
                    version: version,
                    ast: ast
                }
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

