mod error;
mod emit_buffer;

use compiler::parse::Ast;
use compiler::binding::{Bound, BoundRef, SymbolBindSource, LambdaBindings};
use compiler::{CompileContext, ShiftMeta};
use vm::{Instr, ClosureClass};
use ares_syntax::SymbolIntern;

pub use self::error::EmitError;
pub use self::emit_buffer::EmitBuffer;

pub fn emit_all<'bound, 'ast: 'bound, I> (bound: I,
                    compile_context: &mut CompileContext,
                    symbol_intern: &SymbolIntern,
                    out: &mut EmitBuffer,
                    inside_lambda: Option<&LambdaBindings>)
                    -> Result<bool, EmitError>
where I: IntoIterator<Item=BoundRef<'bound, 'ast>> {
    let mut last = false;
    for bound in bound {
        last = try!(emit(&bound, compile_context, symbol_intern, out, inside_lambda));
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
                    symbol_intern: &SymbolIntern,
                    out: &mut EmitBuffer,
                    inside_lambda: Option<&LambdaBindings>)
                    -> Result<bool, EmitError> {
    match bound {
        &Bound::BlockExpression(ref bound_bodies, _) => {
            assert!(try!(emit_all(bound_bodies.iter().map(|&a|a), compile_context, symbol_intern, out, inside_lambda)));
            Ok(true)
        }
        &Bound::BlockStatement(ref bound_bodies, _) => {
            if try!(emit_all(bound_bodies.iter().map(|&a|a), compile_context, symbol_intern, out, inside_lambda)) {
                out.push(Instr::Pop);
            }
            Ok(false)
        }
        &Bound::Add(ref l, ref r, _) => {
            try!(emit(l, compile_context, symbol_intern, out, inside_lambda));
            try!(emit(r, compile_context, symbol_intern, out, inside_lambda));
            out.push(Instr::AddInt);
            Ok(true)
        }
        &Bound::Sub(ref l, ref r, _) => {
            try!(emit(l, compile_context, symbol_intern, out, inside_lambda));
            try!(emit(r, compile_context, symbol_intern, out, inside_lambda));
            out.push(Instr::SubInt);
            Ok(true)
        }
        &Bound::Mul(ref l, ref r, _) => {
            try!(emit(l, compile_context, symbol_intern, out, inside_lambda));
            try!(emit(r, compile_context, symbol_intern, out, inside_lambda));
            out.push(Instr::MulInt);
            Ok(true)
        }
        &Bound::Div(ref l, ref r, _) => {
            try!(emit(l, compile_context, symbol_intern, out, inside_lambda));
            try!(emit(r, compile_context, symbol_intern, out, inside_lambda));
            out.push(Instr::DivInt);
            Ok(true)
        }
        &Bound::LessThan(ref l, ref r, _) => {
            try!(emit(l, compile_context, symbol_intern, out, inside_lambda));
            try!(emit(r, compile_context, symbol_intern, out, inside_lambda));
            out.push(Instr::Lt);
            Ok(true)
        }
        &Bound::LessThanOrEqual(ref l, ref r, _) => {
            try!(emit(l, compile_context, symbol_intern, out, inside_lambda));
            try!(emit(r, compile_context, symbol_intern, out, inside_lambda));
            out.push(Instr::Lte);
            Ok(true)
        }
        &Bound::GreaterThan(ref l, ref r, _) => {
            try!(emit(l, compile_context, symbol_intern, out, inside_lambda));
            try!(emit(r, compile_context, symbol_intern, out, inside_lambda));
            out.push(Instr::Gt);
            Ok(true)
        }
        &Bound::GreaterThanOrEqual(ref l, ref r, _) => {
            try!(emit(l, compile_context, symbol_intern, out, inside_lambda));
            try!(emit(r, compile_context, symbol_intern, out, inside_lambda));
            out.push(Instr::Gte);
            Ok(true)
        }
        &Bound::Equal(ref l, ref r, _) => {
            try!(emit(l, compile_context, symbol_intern, out, inside_lambda));
            try!(emit(r, compile_context, symbol_intern, out, inside_lambda));
            out.push(Instr::Eq);
            Ok(true)
        }
        &Bound::NotEqual(ref l, ref r, _) => {
            try!(emit(l, compile_context, symbol_intern, out, inside_lambda));
            try!(emit(r, compile_context, symbol_intern, out, inside_lambda));
            out.push(Instr::Neq);
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
                    out.push(Instr::SymbolLit(s));
                }
                &Ast::NilLit(_) => {
                    out.push(Instr::NilLit);
                }
                _ => panic!("non-literal ast found in Bound::Literal {:?}", ast),
            }
            Ok(true)
        }
        &Bound::IfExpression(ref cond, ref tru, ref fals, _) => {

            try!(emit(&**cond, compile_context, symbol_intern, out, inside_lambda));

            out.push(Instr::Ifn);
            let (false_pos, fulfill_false) = out.standin();
            out.push_standin(false_pos);

            let mut true_code = EmitBuffer::new(out.offset());
            try!(emit(&**tru, compile_context, symbol_intern,  &mut true_code, inside_lambda));
            let (hop_standin, hop_fulfil) = true_code.standin();
            true_code.push_standin(hop_standin);

            let mut false_code = EmitBuffer::new(true_code.offset());
            try!(emit(&**fals, compile_context, symbol_intern, &mut false_code, inside_lambda));

            // The true branch needs to jump past the end
            // of the false branch.
            let end = false_code.offset();
            true_code.fulfill(hop_fulfil, Instr::Jump(end as u32));

            out.merge(true_code);
            let len_with_true_code = out.offset();
            out.fulfill(fulfill_false, Instr::Jump(len_with_true_code as u32));
            out.merge(false_code);
            Ok(true)
        }
        &Bound::IfStatement(ref cond, ref tru, ref fals, _) => {

            try!(emit(&**cond, compile_context, symbol_intern, out, inside_lambda));
            out.push(Instr::Ifn);
            let (false_pos, fulfill_false) = out.standin();
            out.push_standin(false_pos);

            let mut true_code = EmitBuffer::new(out.offset());
            // Emit true code
            try!(emit(tru, compile_context, symbol_intern, &mut true_code, inside_lambda));

            let mut false_code = EmitBuffer::new(true_code.offset() + 1);
            // Emit false code
            let false_length = if let &Some(ref fals) = fals {
                try!(emit(fals, compile_context, symbol_intern, &mut false_code, inside_lambda));
                false_code.offset()
            } else { 0 };

            if false_length != 0 {
                let end = false_code.offset();
                true_code.push(Instr::Jump(end as u32));
            }

            out.merge(true_code);
            let len_with_true_code = out.offset();
            out.fulfill(fulfill_false, Instr::Jump(len_with_true_code as u32));
            out.merge(false_code);

            Ok(false)
        },
        &Bound::Lambda { ref arg_symbols, ref body, ref bindings, ref upvar_list, ref is_shifter, ..} => {
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
                namespace: symbol_intern.precomputed.default_namespace,
                is_shifter: is_shifter.get(),
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

            if !try!(emit(body, compile_context, symbol_intern, out, Some(bindings))) {
                // If the body was a statement, return nil
                out.push(Instr::NilLit);
            }
            out.push(Instr::Ret);

            let next = out.offset() as u32;
            out.fulfill(eol_fulfill, Instr::Jump(next));
            Ok(true)
        }
        &Bound::FnCall(ref funclike, ref args, _) => {
            for arg in args {
                try!(emit(arg, compile_context, symbol_intern, out, inside_lambda));
            }
            try!(emit(funclike, compile_context, symbol_intern, out, inside_lambda));
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
                &SymbolBindSource::Arg{ref upvar, ..} | &SymbolBindSource::LocalDefine{ref upvar, ..} if upvar.get() => {
                    let binder = inside_lambda.unwrap();
                    try!(emit(value, compile_context, symbol_intern, out, inside_lambda));
                    //out.push(Instr::WrapCell);
                    //out.push(Instr::Assign(binder.compute_stack_offset(source)));
                    out.push(Instr::SetCell(binder.compute_stack_offset(source)));
                }
                &SymbolBindSource::Arg{..} | &SymbolBindSource::LocalDefine{..} => {
                    let binder = inside_lambda.unwrap();
                    try!(emit(value, compile_context, symbol_intern, out, inside_lambda));
                    out.push(Instr::Assign(binder.compute_stack_offset(source)));
                }
                &SymbolBindSource::Global(symbol) => {
                    try!(emit(value, compile_context, symbol_intern, out, inside_lambda));
                    out.push(Instr::PutGlobal(symbol));
                }
                &SymbolBindSource::Upvar{..} => {
                    let binder = inside_lambda.unwrap();
                    try!(emit(value, compile_context, symbol_intern, out, inside_lambda));
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
                    try!(emit(value, compile_context, symbol_intern, out, inside_lambda));
                    out.push(Instr::WrapCell);
                    out.push(Instr::Assign(stack_offset));
                }
                &SymbolBindSource::Arg{ref upvar, ..} |
                &SymbolBindSource::LocalDefine{ref upvar, ..} => {
                    let binder = inside_lambda.unwrap();
                    try!(emit(value, compile_context, symbol_intern, out, inside_lambda));
                    out.push(Instr::Assign(binder.compute_stack_offset(&source)));
                }
                &SymbolBindSource::Global(symbol) => {
                    try!(emit(value, compile_context, symbol_intern, out, inside_lambda));
                    out.push(Instr::PutGlobal(symbol));
                }
                &SymbolBindSource::Upvar{..} => panic!("defining an upvar should be impossible"),
            }
            Ok(false)
        }
        &Bound::Reset(ref symbols, ref closure, _) => {
            try!(emit(closure, compile_context, symbol_intern, out, inside_lambda));

            for symbol_expr in symbols {
                try!(emit(symbol_expr, compile_context, symbol_intern, out, inside_lambda));
            }

            out.push(Instr::Reset(symbols.len() as u32));
            out.push(Instr::Execute(0));

            Ok(true)
        }
        &Bound::Shift(ref symbols, ref closure, _) => {
            for symbol_expr in symbols {
                try!(emit(symbol_expr, compile_context, symbol_intern, out, inside_lambda));
            }


            // Emit the shift-closure
            try!(emit(closure, compile_context, symbol_intern, out, inside_lambda));

            // Push (a standin for) the continuation
            let (s, f) = out.standin();
            out.push_standin(s);

            // Execute the closure with the continuation as the argument
            out.push(Instr::Execute(1));

            // When this shift is over, continue at return_pos
            let shift_id = compile_context.add_shift_meta(ShiftMeta {
                return_pos: out.offset() as u32,
                num_symbols: symbols.len() as u32,
            });

            out.fulfill(f, Instr::Shift(shift_id));

            Ok(true)
        }
        &Bound::ListLit(ref exprs, _) => {
            for expr in exprs {
                try!(emit(expr, compile_context, symbol_intern, out, inside_lambda));
            }
            out.push(Instr::ConstructList(exprs.len() as u32));
            Ok(true)
        }
        &Bound::ListAccess(ref target, ref index, _) => {
            try!(emit(target, compile_context, symbol_intern, out, inside_lambda));
            try!(emit(index, compile_context, symbol_intern, out, inside_lambda));
            out.push(Instr::ListIndex);
            Ok(true)
        }
        &Bound::Import {ref defines, ref namespace, ref version, ..} => {
            for define in defines {
                try!(emit(define, compile_context, symbol_intern, out, inside_lambda));
            }

            Ok(false)
        }
        &Bound::ImportThis { ref name, ref namespace, ref version } => {
            Ok(true)
        },
        &Bound::MapLit(..) => unimplemented!(),
    }
}

