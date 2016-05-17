mod error;
mod emit_buffer;
#[cfg(test)]
pub(crate) mod test;

use compiler::parse::Ast;
use compiler::binding::{Bound, BoundRef, SymbolBindSource, LambdaBindings};
use compiler::CompileContext;
use vm::{Instr, ClosureClass};

pub use self::error::EmitError;
pub use self::emit_buffer::EmitBuffer;

pub fn emit_all<'bound, 'ast: 'bound, I> (bound: I,
                    compile_context: &mut CompileContext,
                    out: &mut EmitBuffer,
                    inside_lambda: Option<&LambdaBindings>)
                    -> Result<(), EmitError>
where I: IntoIterator<Item=BoundRef<'bound, 'ast>> {
    for bound in bound {
        try!(emit(&bound, compile_context, out, inside_lambda));
    }
    Ok(())
}

#[allow(unused_variables)]
pub fn emit<'bound, 'ast: 'bound>(bound: &'bound Bound<'bound, 'ast>,
                    compile_context: &mut CompileContext,
                    out: &mut EmitBuffer,
                    inside_lambda: Option<&LambdaBindings>)
                    -> Result<(), EmitError> {
    match bound {
        &Bound::BlockExpression(ref bound_bodies, _) => {
            for body in &bound_bodies[..bound_bodies.len() - 1] {
                try!(emit(body, compile_context, out, inside_lambda));
                out.push(Instr::Pop);
            }
            if let Some(last_body) = bound_bodies.last() {
                try!(emit(last_body, compile_context, out, inside_lambda));
            }
        }
        &Bound::BlockStatement(ref bound_bodies, _) => {
            for body in &bound_bodies[..] {
                try!(emit(body, compile_context, out, inside_lambda));
                out.push(Instr::Pop);
            }
        }
        &Bound::Add(ref l, ref r, _) => {
            try!(emit(l, compile_context, out, inside_lambda));
            try!(emit(r, compile_context, out, inside_lambda));
            out.push(Instr::AddInt);
        }
        &Bound::Sub(ref l, ref r, _) => {
            try!(emit(l, compile_context, out, inside_lambda));
            try!(emit(r, compile_context, out, inside_lambda));
            out.push(Instr::SubInt);
        }
        &Bound::Mul(ref l, ref r, _) => {
            try!(emit(l, compile_context, out, inside_lambda));
            try!(emit(r, compile_context, out, inside_lambda));
            out.push(Instr::MulInt);
        }
        &Bound::Div(ref l, ref r, _) => {
            try!(emit(l, compile_context, out, inside_lambda));
            try!(emit(r, compile_context, out, inside_lambda));
            out.push(Instr::DivInt);
        }
        &Bound::Literal(ast) => {
            match ast {
                &Ast::IntLit(i, _) => {
                    use std::i32::{MIN, MAX};
                    if i >= MIN as i64 && i <= MAX as i64 {
                        out.push(Instr::IntLit(i as i32));
                    } else {
                        out.push(compile_context.add_constant(i.into()));
                    }
                }
                &Ast::BoolLit(b, _) => {
                    out.push(Instr::BoolLit(b));
                }
                &Ast::StringLit(ref s, _) => {
                    out.push(compile_context.add_constant(s.clone().into()));
                }
                &Ast::FloatLit(f, _) => {
                    out.push(compile_context.add_constant(f.into()));
                }
                &Ast::SymbolLit(s, _) => {
                    out.push(Instr::SymbolLit(s))
                }
                _ => panic!("non-literal ast found in Bound::Literal"),
            }
        }
        &Bound::IfExpression(ref cond, ref tru, ref fals, _) => {
            let mut true_code = EmitBuffer::new();
            let mut false_code = EmitBuffer::new();

            try!(emit(&**cond, compile_context, out, inside_lambda));

            out.push(Instr::Ifn);
            let (false_pos, fulfill_false) = out.standin();
            out.push_standin(false_pos);

            try!(emit(&**tru, compile_context, &mut true_code, inside_lambda));
            try!(emit(&**fals, compile_context, &mut false_code, inside_lambda));

            // The true branch needs to jump past the end
            // of the false branch.
            let end = out.len() + true_code.len() + false_code.len() + 1;
            true_code.push(Instr::Jump(end as u32));

            out.merge(true_code);
            let len_with_true_code = out.len();
            out.fulfill(fulfill_false, Instr::Jump(len_with_true_code as u32));
            out.merge(false_code);
        }
        &Bound::IfStatement(ref cond, ref tru, ref fals, _) => {
            let mut true_code = EmitBuffer::new();
            let mut false_code = EmitBuffer::new();

            try!(emit(&**cond, compile_context, out, inside_lambda));
            out.push(Instr::Ifn);
            let (false_pos, fulfill_false) = out.standin();
            out.push_standin(false_pos);

            try!(emit_all(tru.iter().map(|&a|a), compile_context, &mut true_code, inside_lambda));
            let false_length = if let &Some(ref fals) = fals {
                try!(emit_all(fals.iter().map(|&a|a), compile_context, &mut false_code, inside_lambda));
                false_code.len()
            } else { 0 };

            if false_length != 0 {
                let end = out.len() + true_code.len() + false_code.len() + 1;
                true_code.push(Instr::Jump(end as u32));
            }

            out.merge(true_code);
            let len_with_true_code = out.len();
            out.fulfill(fulfill_false, Instr::Jump(len_with_true_code as u32));
            out.merge(false_code);

        },
        &Bound::Lambda { ref arg_symbols, ref body, ref bindings, ..} => {
            const INSTRS_BEFORE_LAMBDA_CODE: u32 = 2;
            let prior_code_len = out.len();

            let closure_class = ClosureClass {
                    code_offset: out.len() as u32 + INSTRS_BEFORE_LAMBDA_CODE,
                    // TODO: take varargs into account
                    arg_count: arg_symbols.len() as u32,
                    local_defines_count: bindings.num_declarations,
                    upvars_count: bindings.num_upvars,
                    has_rest_params: false,
            };

            let cc_id = compile_context.add_closure_class(closure_class);

            out.push(Instr::CreateClosure(cc_id));
            // TODO: load closure with upvars

            // Standin for the end of the lambda code.
            let (eol_standin, eol_fulfill) = out.standin();
            out.push_standin(eol_standin);

            debug_assert_eq!(prior_code_len + INSTRS_BEFORE_LAMBDA_CODE as usize,
                             out.len());

            try!(emit(body, compile_context, out, Some(bindings)));
            out.push(Instr::Ret);

            let next = out.len() as u32;
            out.fulfill(eol_fulfill, Instr::Jump(next));
        }
        &Bound::FnCall(ref funclike, ref args, _) => {
            for arg in args {
                try!(emit(arg, compile_context, out, inside_lambda));
            }
            try!(emit(funclike, compile_context, out, inside_lambda));
            out.push(Instr::Execute(args.len() as u32));
        }
        &Bound::Symbol { symbol, ast, source, } => {
            if let SymbolBindSource::Global(symbol) = source {
                out.push(Instr::GetGlobal(symbol));
            } else {
                let binder = inside_lambda.unwrap();
                out.push(Instr::Dup(binder.compute_stack_offset(source)));
            }
        }
        &Bound::Define(_, source, value, _) => {
            if let SymbolBindSource::Global(_) = source {
                unimplemented!();
            } else {
                let binder = inside_lambda.unwrap();
                try!(emit(value, compile_context, out, inside_lambda));
                out.push(Instr::DupTop);
                out.push(Instr::Assign(binder.compute_stack_offset(source)));
            }
        }
        &Bound::ListLit(_, _) |
        &Bound::MapLit(_, _) => unimplemented!(),
    }
    Ok(())
}

