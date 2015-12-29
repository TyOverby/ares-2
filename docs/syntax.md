# Syntax

Originally Ares was designed with an s-expression based syntax.
However, the more that I thought about integrating well with rust, the more
I'm convinced that it needs to be imperative and not promote common
functional paridigms.

## Reserved words
```
fn
class // unused
prototype //unused
shift
reset
async // unused
await // unused
yield // unused
return
continue
break
pre
post
```

## Function

### Basic Syntax
Named function with no parameters
```ares
fn named_function() { }
```

Named function with parameters

```ares
fn named_function(a, b, c) { }
```

Named function with rest parameters

```ares
fn named_function(a, b, rest...) { }
```

Anonymous functions

```ares
fn(a, b) {  }
fn(a, b, rest...) {  }
```

### Pre and Post Conditions

Pre and post conditions are used to dynamically enforce invariants
in your code.  These are typically used to sanitize the type of
parameters, but can be arbitrary code as long as the "result" of
the block is of type boolean;

Pre-conditions and post-conditions are

Type-aware preconditions and post conditions are required to be
the first "statements" in a function body, and it is an error
to place them anywhere else.

The interpreter can choose to evaluate the preconditions and
postcondition in any order, and may decide to skip them.  Thus,
pre-conditions and post-conditions should be pure.

The virtual machine will run these checks by default, but they can
be turned off for faster performance.

In the future, if a type-system is added, these pre-conditions will
may be used to provide compile-time errors.

```ares
fn(a, b) {
    pre { typeof(a) == 'int };
    pre { typeof(b) == 'int };
    post { typeof(result) == 'int };

    return a + b;
}
```

"generic" type preconditions and post conditions

```ares
fn(a, b) {
    pre { typeof(a) == typeof(b) }
    post { typeof(a) == typeof(result) }
}
```

## Curried functions

An easier way to generate lambdas for use in common patterns.
```ares
fn im_curried(a, b, c)(d, e, f) {
    a + b + c + d + e + f
}
```

could be lowered to

```ares
fn im_curried(a, b, c) {
    fn(d, e, f) {
        a + b + c + d + e + f
    }
}
```

Thought a more optimized version without implicit closure creation
at every step would be possible if the interpreter specialized.


## Expressions vs Statements

An expression that is terminated with a semicolon is
turned into a statement.

Any expression that is at the end of a block, or
at the end of a function is the "return value" of that
block or function.

```ares
fn(a, b) {
    a + b // no return required.
}

fn(a, b) {
    let res = {
        a + b + 5
    };
    res
}

fn(a, b) {
    // `if` is also an expression
    let x = if a == 0 { b } else { a };
    let x = if a == 0 then b else a;

    let x = if a == 0 then {
        foo
    } 

    x + 10
}
```
