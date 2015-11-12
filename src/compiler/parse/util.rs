use vm::SymbolIntern;
use compiler::parse::Ast;

pub fn immediate_value(v: &Ast, interner: &mut SymbolIntern) -> bool {
    match v {
        &Ast::MapLit(ref m, _) =>
            m.iter().all(|&(ref k, ref v)| immediate_value(k, interner) && immediate_value(v, interner)),
        &Ast::Quote(_, _) => true,
        &Ast::SymbolLit(_, _) => false,
        _ => true,
    }
}

pub fn unquote(v: &Ast) -> Ast {
    match v {
        &Ast::ListLit(ref vec, _) => vec[1].clone(),
        v => v.clone(),
    }
}

pub fn can_be_hash_key(v: &Ast, interner: &mut SymbolIntern) -> bool {
    match v {
        &Ast::IntLit(_, _) => true,
        &Ast::BoolLit(_, _) => true,
        &Ast::FloatLit(_, _) => true,
        &Ast::Quote(ref quoted, _) => {
            if let &Ast::SymbolLit(_, _) = &**quoted {
                true
            } else {
                can_be_hash_key(&*quoted, interner)
            }
        },
        _ => false
    }
}
