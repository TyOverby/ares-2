mod parse;
mod emit;
mod error;
mod compile_context;
mod binding;

use typed_arena;
pub use compiler::error::CompileError;
use compiler::emit::EmitBuffer;
use ares_syntax::SymbolIntern;
use vm::Instr;

pub use self::compile_context::CompileContext;

pub fn compile(source: &str,
               compile_context: &mut CompileContext,
               interner: &mut SymbolIntern)
               -> Result<Vec<Instr>, CompileError> {

    let ast_arena: typed_arena::Arena<parse::Ast> = typed_arena::Arena::new();
    let bound_arena: typed_arena::Arena<binding::Bound> = typed_arena::Arena::new();

    let mut out = EmitBuffer::new();
    let asts = try!(parse::parse(source, interner, &ast_arena));
    for ast in &asts {
        let bound = binding::Bound::bind_top(ast, &bound_arena, interner);
        try!(emit::emit(try!(bound), compile_context, &mut out, None));
        // Pop because an expression just completed, so we don't
        // want to just leave the result on the stack.
        out.push(Instr::Pop);
    }
    if out.len() != 0 {
        // Pop the last pop.
        out.pop();
    }

    Ok(out.into_instructions())
}
/*
macro_rules! ast {
    ($arena: expr, $intern: expr, BoolLit($value: expr)) => {
        $arena.alloc(Ast::BoolLit($value, Span::dummy()))
    };
    ($arena: expr, $intern: expr, StringLit($value: expr)) => {
        $arena.alloc(Ast::StringLit($value.to_string(), Span::dummy()))
    };
    ($arena: expr, $intern: expr, FloatLit($value: expr)) => {
        $arena.alloc(Ast::FloatLit($value, Span::dummy()))
    };
    ($arena: expr, $intern: expr, IntLit($value: expr)) => {
        $arena.alloc(Ast::IntLit($value, Span::dummy()))
    };
    ($arena: expr, $intern: expr, Symbol($value: ident)) => {
        $arena.alloc(Ast::Symbol($intern.intern(stringify!($value)), Span::dummy()))
    };
    ($arena: expr, $intern: expr, ListLit($($name:tt $args: tt),*)) =>  {
        $arena.alloc(Ast::ListLit(vec![
            $(ast!($arena, $intern, $name $args)),*
        ], Span::dummy()))
    };
    ($arena: expr, $intern: expr, Add($($name:tt $args: tt),*)) =>  {
        $arena.alloc(Ast::Add(vec![
            $(ast!($arena, $intern, $name $args)),*
        ], Span::dummy()))
    };
    ($arena: expr, $intern: expr, Quote($name:tt $args: tt)) =>  {
        $arena.alloc(Ast::Quote(ast!($arena, $intern, $name $args), Span::dummy()))
    };
    ($arena: expr, $intern: expr, List($($name:tt $args: tt),*)) =>  {
        $arena.alloc(Ast::List(vec![
            $(ast!($arena, $intern, $name $args)),*
        ], Span::dummy()))
    };

    ($arena: expr, $intern: expr, If($cond_name:tt $cond_args:tt, $tru_name:tt $tru_args:tt, $fal_name:tt $fal_args:tt)) => {
        $arena.alloc(Ast::If(
                ast!($arena, $intern, $cond_name $cond_args),
                ast!($arena, $intern, $tru_name $tru_args),
                ast!($arena, $intern, $fal_name $fal_args),
                Span::dummy()))
    };

    ($arena: expr, $intern: expr, Lambda(($($symbol: ident),*), $($name:tt $arg:tt),*)) => {
        $arena.alloc(Ast::Lambda(
                vec![$($intern.intern(stringify!($symbol))),*],
                ast!($arena, $intern, Block($($name $arg),*)),
                Span::dummy()))
    };

    ($arena: expr, $intern: expr, Define($name: ident, $value_name:tt $value_args:tt)) => {
        $arena.alloc(Ast::Define(
                $intern.intern(stringify!($name)),
                ast!($arena, $intern, $value_name $value_args),
                Span::dummy()))
    };

    ($arena: expr, $intern: expr, Block($($name:tt $args: tt),*)) =>  {
        $arena.alloc(Ast::Block(vec![
            $(ast!($arena, $intern, $name $args)),*
        ], Span::dummy()))
    };
}

mod test_macro {
    use typed_arena::Arena;
    use vm::SymbolIntern;
    use compiler::parse::{Ast, Span};

    fn test_ast_macro() {
        let arena: Arena<Ast> = Arena::new();
        let mut inter: SymbolIntern = SymbolIntern::new();
        let _ = ast!(arena, inter, BoolLit(true));
        let _ = ast!(arena, inter, StringLit("hi"));
        let _ = ast!(arena, inter, FloatLit(1.23));
        let _ = ast!(arena, inter, IntLit(13));
        let _ = ast!(arena, inter, Symbol(x));
        let _ = ast!(arena, inter, ListLit(BoolLit(true)));
        let _ = ast!(arena, inter, ListLit(IntLit(5), BoolLit(true)));
        let _ = ast!(arena, inter, ListLit(IntLit(5), ListLit(BoolLit(true))));
        let _ = ast!(arena, inter, Add(IntLit(13), Add(IntLit(5), IntLit(2))));
        let _ = ast!(arena, inter, Quote(Add(IntLit(13), Add(IntLit(5), IntLit(2)))));
        let _ = ast!(arena, inter, If(IntLit(5), ListLit(BoolLit(false)), BoolLit(true)));
        let _ = ast!(arena, inter, Define(x, Symbol(y)));
        let _ = ast!(arena, inter, Lambda((a, b, c), Symbol(x), BoolLit(false)));
    }
}*/
