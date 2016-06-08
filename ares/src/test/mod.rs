mod matrix_driver;
pub mod bind;
pub mod emit;

pub use self::matrix_driver::*;
pub use self::bind::*;
pub use self::emit::*;
use ::ares_syntax::SymbolIntern;
use ::typed_arena::Arena;
use ::vm::Modules;
use ::host::{AresResult, AresError};
use ::compiler::{CompileError, CompileContext};
use ::compiler::binding::{Bound, BoundRef};
use ::compiler::emit::{EmitBuffer, emit_all};

pub enum TestResult {
    NotRan,
    Good,
    Error(AresError),
    Bad(String),
    Panic(Box<::std::any::Any + Send + 'static>),
}

fn do_parsing<'ast>(
    program: &str,
    parse_arena: &'ast Arena<Ast<'ast>>,
    interner: &mut SymbolIntern) -> AresResult<Vec<Ast<'ast>>> {

    let ast = match ::compiler::parse::parse(program, interner, parse_arena) {
        Ok(ast) => ast,
        Err(error) => return Err(From::<CompileError>::from(From::from(error)))
    };

    Ok(ast)
}

fn do_binding<'bound, 'ast: 'bound>(
    program: &str,
    parse_arena: &'ast Arena<Ast<'ast>>,
    bind_arena: &'bound Arena<Bound<'bound, 'ast>>,
    interner: &mut SymbolIntern,
    modules: Option<&Modules>) -> AresResult<Vec<BoundRef<'bound, 'ast>>> {

    let asts = try!(do_parsing(program, parse_arena, interner));
    let asts = asts.into_iter().map(|ast| parse_arena.alloc(ast) as &_);
    let asts: Vec<_> = asts.collect();
    let bound = match Bound::bind_top(&asts[..], bind_arena, modules, interner) {
        Ok(bound) => bound,
        Err(error) => return Err(From::<CompileError>::from(From::from(error)))
    };

    Ok(bound)
}

fn do_emitting<'bound, 'ast: 'bound>(
    program: &str,
    parse_arena: &'ast Arena<Ast<'ast>>,
    bind_arena: &'bound Arena<Bound<'bound, 'ast>>,
    interner: &mut SymbolIntern,
    modules: Option<&Modules>,
    compile_context: &mut CompileContext,
    emit_buffer: &mut EmitBuffer) -> AresResult<()> {

    let bound = try!(do_binding(program, parse_arena, bind_arena, interner, modules));
    if let Err(error) = emit_all(bound, compile_context, emit_buffer, None) {
        return Err(From::<CompileError>::from(From::from(error)));
    }

    Ok(())
}
