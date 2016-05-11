# Exceptions

What most people think of as exceptions could be implemented via
delimited continuations.

In Ares, they would primarily be utility functions for passing 
Ares "result" objects around the stack frame.

```ares
import success, failure, is_success, is_result, is_failure from std.result;
import has_slot, get_slot, set_slot from std.reflection;
import assert from std.panic;

const ex_symbol = gensym();
const ex_cont = gensym();
const good_result = gensym();

fn throw(error) {
    shift(ex_symbol) { |k|
        error.set_slot(ex_cont, k);
        error
    }
}

fn rethrow(error) {
    shift(ex_symbol) {
        error
    }
}

fn try(f) {
    reset(ex_symbol) {
        f()
    }
}

fn catch(obj)(handler) {
    assert(obj!is_result(), "std.exception.catch called on a non-object", obj);
    if obj!is_success() {
        obj
    } else {
        handler(obj)
    }
}

fn resume(obj, value) {
    assert!(obj!is_failure(), "resume called with an object that is not a failure", obj);
    assert!(obj!has_slot(ex_cont), "resume called with a failure that was not thrown", obj);

    let continuation = obj!get_slot(ex_cont);
    try {
        obj!get_slot(ex_cont)(value);
    }
}
```

Now in use

```
let r = try { 1 + 1 };
assert(r!is_success());
assert(r.value == 2);

fn throws_exception() {
    let replacement = throw(failure('hit_iceburg, "oh the humanity"));
    replacement + 1
}

let r = try {
    print(throws_exception());
    "we're good"
}!catch { |e|
    e!resume(5)
};
// prints 6
assert!(r!is_success());
assert!(r.value == "we're good");

```
