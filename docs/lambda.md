# Lambda

Lambdas (closures) are the most interesting part of the interpreter and
compiler.  This document will go over how they are represented in the runtime,
as well as how they are compiled.

## Runtime
The runtime keeps track of each "class" of lambda that can be instantiated
by a running program.  For example, "(lambda (a) a)" would have a "class"
that represents a lambda which takes one argument.  Similarly,
"(lambda (a) (+ a b))" would have a "class" that represents a lambda
that takes one argument, and has one upvar (in this case, "b").

These classes are stored in CompileContext because they are generated at
compile-time.

ClosureClass {
    code_offset: The position in the code that this lambda begins execution at.
    arg_count: The number of arguments that this lambda takes.
    has_rest_params: True if the remaining args become a list
}

### Closure Creation

At runtime, when a lambda is loaded, the id of the lambda class is given
to the interpreter with the `Instr::CreateLambda(class_id: u32)` instruction.
The lambda isn't done being made yet though. `Instr::LoadLambda(n: u32)`
loads the top `n` upvars into the lambda.  Only then is the `Value::Closure`
created and placed on the stack.

(Side note: All values loaded into the lambda via LoadLambda should be
`Value::Cell`)

Closure {
    class: ClosureClass //  as described above
    upvars: Vec<Value>  // A vec of upvars that this closure closes over
}

### Closure execution

When a closure is executed, the arguments passed in on the stack are kept
where they are, upvars are loaded into the stack, and then zeroed local
declarations are pushed onto the stack.

+-------+
| local |
| local |
| local |
+-------+
| upvar |
| upvar |
| upvar |
+-------+
|  arg  |
|  arg  |
|  arg  |
+-------+
