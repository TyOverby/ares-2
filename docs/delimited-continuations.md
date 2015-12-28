# Delimited Continuations

In addition to closures, Ares will feature delimited continuations as a
method for simplifying non-linear code flow.

Similar to their implementation in common-lisp, the operators for delimited
continuations will be named `shift` and `reset`.

## Syntax

`reset` is an expression that may contain shifts inside of it.

```ares
let k = reset {
    let x = shift k {
        k
    }
    x + 1
};

k(5) // 6
```

Would be functionally equivilant to

```ares
let k = fn(x) {
    x + 1
};

k(5) // 6
```

A shortcut for `shift k { k }` could be shortened to `shift` which would allow the
previous example to be written as:

```ares
let k = reset {
    let x = shift;
    x + 1
};

k(5) // 6
```

This might look pretty gnarly compared to the version using the closure, but
compare these:

Using closures:
```ares
let k = fn(a) {
    fn(b) {
        fn(c) {
            a + b + c
        }
    }
};
k(1)(2)(3) // 6
```

Using delimited continuations:
```ares
let k = reset {
    let a = shift;
    let b = shift;
    let c = shift;
    a + b + c
};

k(1)(2)(3) // 6
```

## The "end-goal" of delimited continuations

Allow highly readable asynchronous code to be written in a way that
looks linear and is _obvious_ when looking at it because the boundary
of asynchrony is "delimited" with "reset".

I could imagine an echo server being written that looks like this:

```ares
reset {
    for connection in listen(8000) {
        reset {
            for message in connection {
                connection.send(message)
            }
        }
    }
}
```
