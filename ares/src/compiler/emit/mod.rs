mod error;
mod emit_buffer;

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
                    -> Result<bool, EmitError>
where I: IntoIterator<Item=BoundRef<'bound, 'ast>> {
    let mut last = false;
    for bound in bound {
        last = try!(emit(&bound, compile_context, out, inside_lambda));
        if last {
            out.push(Instr::Pop);
        }
    }

    // Pop the last pop;
    if last {
        out.pop();
        Ok(true)
    } else {
        Ok(false)
    }
}

#[allow(unused_variables)]
pub fn emit<'bound, 'ast: 'bound>(bound: &'bound Bound<'bound, 'ast>,
                    compile_context: &mut CompileContext,
                    out: &mut EmitBuffer,
                    inside_lambda: Option<&LambdaBindings>)
                    -> Result<bool, EmitError> {
    match bound {
        &Bound::BlockExpression(ref bound_bodies, _) => {
            assert!(try!(emit_all(bound_bodies.iter().map(|&a|a), compile_context, out, inside_lambda)));
            Ok(true)
        }
        &Bound::BlockStatement(ref bound_bodies, _) => {
            if try!(emit_all(bound_bodies.iter().map(|&a|a), compile_context, out, inside_lambda)) {
                out.push(Instr::Pop);
            }
            Ok(false)
        }
        &Bound::Add(ref l, ref r, _) => {
            try!(emit(l, compile_context, out, inside_lambda));
            try!(emit(r, compile_context, out, inside_lambda));
            out.push(Instr::AddInt);
            Ok(true)
        }
        &Bound::Sub(ref l, ref r, _) => {
            try!(emit(l, compile_context, out, inside_lambda));
            try!(emit(r, compile_context, out, inside_lambda));
            out.push(Instr::SubInt);
            Ok(true)
        }
        &Bound::Mul(ref l, ref r, _) => {
            try!(emit(l, compile_context, out, inside_lambda));
            try!(emit(r, compile_context, out, inside_lambda));
            out.push(Instr::MulInt);
            Ok(true)
        }
        &Bound::Div(ref l, ref r, _) => {
            try!(emit(l, compile_context, out, inside_lambda));
            try!(emit(r, compile_context, out, inside_lambda));
            out.push(Instr::DivInt);
            Ok(true)
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
                _ => panic!("non-literal ast found in Bound::Literal {:?}", ast),
            }
            Ok(true)
        }
        &Bound::IfExpression(ref cond, ref tru, ref fals, _) => {
            let mut true_code = EmitBuffer::new(0);
            let mut false_code = EmitBuffer::new(0);

            try!(emit(&**cond, compile_context, out, inside_lambda));

            out.push(Instr::Ifn);
            let (false_pos, fulfill_false) = out.standin();
            out.push_standin(false_pos);

            try!(emit(&**tru, compile_context, &mut true_code, inside_lambda));
            try!(emit(&**fals, compile_context, &mut false_code, inside_lambda));

            // The true branch needs to jump past the end
            // of the false branch.
            let end = out.offset() + true_code.offset() + false_code.offset() + 1;
            true_code.push(Instr::Jump(end as u32));

            out.merge(true_code);
            let len_with_true_code = out.offset();
            out.fulfill(fulfill_false, Instr::Jump(len_with_true_code as u32));
            out.merge(false_code);
            Ok(true)
        }
        &Bound::IfStatement(ref cond, ref tru, ref fals, _) => {
            let mut true_code = EmitBuffer::new(0);
            let mut false_code = EmitBuffer::new(0);

            try!(emit(&**cond, compile_context, out, inside_lambda));
            out.push(Instr::Ifn);
            let (false_pos, fulfill_false) = out.standin();
            out.push_standin(false_pos);

            // Emit true code
            try!(emit(tru, compile_context, &mut true_code, inside_lambda));

            // Emit false code
            let false_length = if let &Some(ref fals) = fals {
                try!(emit(fals, compile_context, &mut false_code, inside_lambda));
                false_code.offset()
            } else { 0 };

            if false_length != 0 {
                let end = out.offset() + true_code.offset() + false_code.offset() + 1;
                true_code.push(Instr::Jump(end as u32));
            }

            out.merge(true_code);
            let len_with_true_code = out.offset();
            out.fulfill(fulfill_false, Instr::Jump(len_with_true_code as u32));
            out.merge(false_code);

            Ok(false)
        },
        &Bound::Lambda { ref arg_symbols, ref body, ref bindings, ref upvar_list, ..} => {
            // Push all needed upvars onto the stack for the closure to take hold of.
            if !upvar_list.is_empty() {
                let binder = inside_lambda.unwrap();
                for upvar in upvar_list {
                    out.push(Instr::Dup(binder.compute_stack_offset(&upvar)));
                }
            }

            let (create_closure_standin, create_closure_fulfill) = out.standin();
            let (eol_standin, eol_fulfill) = out.standin();
            out.push_standin(create_closure_standin);
            out.push_standin(eol_standin);

            let closure_class = ClosureClass {
                code_offset: out.offset() as u32,
                // TODO: take varargs into account
                arg_count: arg_symbols.len() as u32,
                local_defines_count: bindings.num_declarations,
                upvars_count: bindings.num_upvars,
                has_rest_params: false,
            };

            let cc_id = compile_context.add_closure_class(closure_class);
            out.fulfill(create_closure_fulfill, Instr::CreateClosure(cc_id));

            // Convert any closed over argument into a cell
            for (_, binding) in bindings.bindings.iter() {
                if let &SymbolBindSource::Arg{ ref upvar, .. } = binding {
                    if upvar.get() {
                        let position = bindings.compute_stack_offset(binding);
                        out.push(Instr::Dup(position));
                        out.push(Instr::WrapCell);
                        out.push(Instr::Assign(position));
                    }
                }
            }

            try!(emit(body, compile_context, out, Some(bindings)));
            out.push(Instr::Ret);

            let next = out.offset() as u32;
            out.fulfill(eol_fulfill, Instr::Jump(next));
            Ok(true)
        }
        &Bound::FnCall(ref funclike, ref args, _) => {
            for arg in args {
                try!(emit(arg, compile_context, out, inside_lambda));
            }
            try!(emit(funclike, compile_context, out, inside_lambda));
            out.push(Instr::Execute(args.len() as u32));
            Ok(true)
        }
        &Bound::Symbol { symbol, ast, ref source, } => {
            match source {
                &SymbolBindSource::Global(symbol) => {
                    out.push(Instr::GetGlobal(symbol));
                }
                &SymbolBindSource::Arg{ref upvar, ..} | &SymbolBindSource::LocalDefine{ref upvar, ..} if upvar.get() => {
                    let binder = inside_lambda.unwrap();
                    out.push(Instr::Dup(binder.compute_stack_offset(&source)));
                    out.push(Instr::UnwrapCell);
                }
                &SymbolBindSource::Arg{..} | &SymbolBindSource::LocalDefine{..} => {
                    let binder = inside_lambda.unwrap();
                    out.push(Instr::Dup(binder.compute_stack_offset(&source)));
                }
                &SymbolBindSource::Upvar{..} => {
                    let binder = inside_lambda.unwrap();
                    out.push(Instr::Dup(binder.compute_stack_offset(&source)));
                    out.push(Instr::UnwrapCell);
                }
            }
            Ok(true)
        }
        &Bound::Assign(_, ref source, value, _) => {
            match source {
                &SymbolBindSource::Arg{..} | &SymbolBindSource::LocalDefine{..} => {
                    let binder = inside_lambda.unwrap();
                    try!(emit(value, compile_context, out, inside_lambda));
                    out.push(Instr::DupTop);
                    out.push(Instr::Assign(binder.compute_stack_offset(source)));
                }
                &SymbolBindSource::Global(symbol) => {
                    try!(emit(value, compile_context, out, inside_lambda));
                    out.push(Instr::PutGlobal(symbol));
                }
                &SymbolBindSource::Upvar{..} => {
                    let binder = inside_lambda.unwrap();
                    try!(emit(value, compile_context, out, inside_lambda));
                    out.push(Instr::SetCell(binder.compute_stack_offset(source)))
                }
            }
            Ok(false)
        }
        &Bound::Define(_, ref source, value, _) => {
            match source {
                &SymbolBindSource::Arg{ref upvar, ..} |
                &SymbolBindSource::LocalDefine{ref upvar, ..} if upvar.get() => {
                    let binder = inside_lambda.unwrap();
                    let stack_offset = binder.compute_stack_offset(&source);
                    try!(emit(value, compile_context, out, inside_lambda));
                    out.push(Instr::WrapCell);
                    out.push(Instr::Assign(stack_offset));
                }
                &SymbolBindSource::Arg{ref upvar, ..} |
                &SymbolBindSource::LocalDefine{ref upvar, ..} => {
                    let binder = inside_lambda.unwrap();
                    try!(emit(value, compile_context, out, inside_lambda));
                    out.push(Instr::Assign(binder.compute_stack_offset(&source)));
                }
                &SymbolBindSource::Global(symbol) => {
                    try!(emit(value, compile_context, out, inside_lambda));
                    out.push(Instr::PutGlobal(symbol));
                }
                &SymbolBindSource::Upvar{..} => panic!("defining an upvar should be impossible"),
            }
            Ok(false)
        }
        &Bound::ListLit(_, _) |
        &Bound::MapLit(_, _) => unimplemented!(),
    }
}

