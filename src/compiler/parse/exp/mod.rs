use std::boxed::Box;

pub enum Ast {
    FloatLit(f64),
    IntLit(i64),
    Add(Box<Ast>, Box<Ast>),
    Sub(Box<Ast>, Box<Ast>),
    Mul(Box<Ast>, Box<Ast>),
    Div(Box<Ast>, Box<Ast>),
}

mod syntax;
