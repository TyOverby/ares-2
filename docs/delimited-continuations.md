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

## Generators via delimited continuations

Generators can be implemented with delimited continuations.

```ares
fn done() { [nil, done] }

export fn yield(value) {
    return shift k { [value, k] };
}

export fn generator(f, args...) {
    let continuation = reset {
        f.apply(args);
        fn done() { [nil, done] }
    };

    return fn(passback?) {
        let [value, k] = continuation(passback);
        continuation = k;
        return value;
    }
}
```

Then, you can write your generators and use them like this:

```ares
import exp.gen.yield;
import exp.gen.generator;

fn even_numbers() {
    yield(0);
    yield(2);
    yield(4);
    yield(6);
    yield(8);
}

for number in generator(even_numbers) {
    print(number);
}
```

Or even pass values back into the generator.

```ares
import exp.gen.yield;
import exp.gen.generator;

fn what_you_give() {
    let start = 0;
    while true {
        start = yield(start);
    }
}

let gen = generator(what_you_give);

let i = 0;
while true {
    print(gen(i));
    i += 1;
}
```

## Named continuations?

It might be handy to provide "named continuations" in order to allow for
shifts to transfer across multiple resets.

An example would be trying to mix generators with asynchronous io.  Putting
async io inside of a generator could shift inside of the wrong reset.  By pairing
shifts and resets you could get rid of this problem for well-written APIs.

With named delimited continuations, generators would look like this:

```ares
fn done() { [nil, done] }

const generator_symbol = gensym();

export fn yield(value) {
    return shift(generator_symbol) k { [value, k] };
}

export fn generator(f, args...) {
    let continuation = reset(generator_symbol) {
        f.apply(args);
        fn done() { [nil, done] }
    };

    return fn(passback?) {
        let [value, k] = continuation(passback);
        continuation = k;
        return value;
    }
}
```

## Exceptions

Delimited continuations can also implement resumable exception handling.

```
const all_good = gensym();
const exception = gensym();

fn try(body, catch) {
    let result = nil;
    let ex = reset(exception) {
        result = f();
        all_good
    };

    if ex == all_good {
        return result;
    } else {
        return catch(ex[0], ex[1]);
    }
}

fn raise(error) {
    return shift(exception) k {
        [error, k]
    }
}
```

## Call/cc esque?

If we allowed for a "global" reset, you could get call/cc functionality
without needing to add a new special form.

```
// Imagine that the entire program was wrapped in
// one large `reset ('global) {  }`

fn call_cc(f) {
    shift('global) k {
        f(k)
    }
}
```

This would let you implement things like forking in a really pretty way.

Imagine we have a function called "run_on_thread" which took a function, and
the arguments that get passed to that function when executed on another thread.
The function would return the thread ID of the thread that started.
An implementation of fork could be:

```ares
fn fork() {
    call_cc(fn(k) {
        let thread_id = run_on_thread(k, 0);
        k(thread_id)
    })
}

// without call_cc
fn fork2() {
    shift('global) k {
        let thread_id = run_on_thread(k, 0);
        k(thread_id)
    }
}
```

Then, in your program, you could write something like this:

```ares
do_something();
let child = fork();
if child == 0 {
    print("I'm the child");
} else {
    print("my child is ", child);
}
do_other_things();
```
