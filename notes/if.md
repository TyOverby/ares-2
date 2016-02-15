
## Statements
if a then { b } else { c };
if a then { b } else { c }
if a then { d(); }

## Expressions

is

`if a then b else c()`

`if a then b else (c())` Or `(if a then b else c)()`

    "if" <cond: Expr> "then" <tru: Expr> "else" <fals: Expr> =>
        Ast::IfExpression(arena.alloc(cond),
                          arena.alloc(tru),
                          arena.alloc(fals),
                          Span(0, 0)),
