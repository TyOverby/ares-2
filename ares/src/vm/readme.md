# Vm

The virtual machine contains all of the structs and procedures
to run compiled bytecode.

The `Value` enumeration is provided by this module, as well as the representations
of the various values such as Closures, Object, Lists, Maps, etc.

It also exports the `Vm` struct, which stores the state of the virtual machine
and has the `load_and_execute` method that runs compiled bytecode.

## Vm Design

The Ares Vm is a stack-based virtual machine with a focus on *not* encoding
language semantics into the instruction set.  Hopefully, this conceptual
break will keep the Ares language from picking up complex semantics.

### The Value Stack

The Value Stack, also referred to as simply "the stack", is a stack that
(as its name implies) only only values.  (This is in contrast to other
languages that additionally contain return information on their stack).

The values found on the Value Stack can be classified in 3 different ways:

1. **Function Arguments** are passed in on the stack.  When a closure is
   called, the first values on that closures stack frame are the arguments
   that were passed in.
2. **Function Locals** are given space on the stack.  The amout of locals
   that a function needs are computed at compile time, so an adequite
   amout of space is provided (and zeroed out) on a called functions
   stackframe.
3. **Closed Over Variable** are a necessary for supporting closures.
   These are stored in the closure object, and are pushed on the Value Stack
   when the function is executed.
4. **Temporary Values**.  Intermediate results of computation go on the value stack.
   Some code like "1 + 2 * 3" might be compiled to instructions like
   `PUSH(2), PUSH(3), MUL, PUSH(1), ADD`.  In this computation, all of the constants,
   as well as the result of the multiplication are present on the stack for a short
   period of time.

### The Return Stack

Other vms and ABIs combine the "Value Stack" and "Return Stack" into one concept, but
the Ares Vm keeps them seperate.  The Return Stack is a stack containing Return structs
which stores information about what a closure should do when it returns.  Specifically,
the Return structure keeps track of

1. code_pos: The location of the next instruction that will be executed after the
   called function completes.  
2. stack_frame: The offset into the stack that begins the calling functions stack frame.
   The stack frame offset is to be restored when the called function completes.
3. namespace: The namespace of the calling function.  The namespace is to be restored
   when the calling function completes.

### The Stack Frame

The stack frame is the concept that a function "owns" a chunk of the value stack that
it keeps its arguments, local values, closed-over variables, and temporaries. The stack
during an executing function is organized like this:

```
+----------------+
|Stack frame for |
|calling function|
+----------------+
| arg 1          |\
| arg 2          | | Size known at compile time
| ..             |/
| -------------- |
| local var 1    |\
| local var 2    | | Size known at compile time
| ..             |/
| -------------- |
| closed var 1   |\
| closed var 2   | | Size known at compile time
| ..             |/
| -------------- |
| temp variables |\
| temp variables | | This part grows and shrinks during execution
| temp variables |/
+----------------+
|Stack frame for |
|called function |
+----------------+
```

### Globals / Namespaces

At some level, the VM needs to store global variables to make a repl feasable.
Otherwise, `ctx.eval("let x = 1")` followed by `ctx.eval("print(x)")` would be
much much more difficult to support.  As it is, any name defined in the top level
scope is considered global to that "module".  If not specified, the default module
is the "~root~" module.  A file can put its contents in a specific namespace by
placing a namespace statement at the top of the file.  This is what it looks like:

```ares
namespace foo;
let x = 5;
```

This declares that the remainder of the file should be loaded in the "foo" namespace.
