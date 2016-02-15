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

export fn generator(f) {
    let continuation = reset(generator_symbol) {
        f();
        done
    };

    return fn(passback) {
        let [value, k] = continuation(passback);
        continuation = k;
        return value;
    }
}
```

Usage

```ares
fn powers_of_two() {
    x = 1;
    while true {
        yield(x);
        x *= 2;
    }
}

for x in generator(powers_of_two) {
    print(x);
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

## Cooperating coroutines
Coroutines are a way to schedule execution of multiple "threads" of activity
onto a single OS thread (or in this case, a single interpreter.)  Good coroutine
implementations will switch contexts on IO, and also whenever the programmers
yields control of the coroutine.

Because of how ares is written, all functions are *implicitly* cooperating.
They yield activity whenever any blocking operation is submitted and an implicit
reschedule occurs with the library function "next_tick()".

However, sometimes it would be nice to not go back out to the event loop in order to
schedule threads that don't need to do io.

In that case, we can write something like this:

This function will create a coroutine manager that can spawn
coroutines.

```ares
fn coro() {
    object {
        enqueued: [],
        running: false,
        cancled: false,
        outstanding: 0,
        enqueue: fn(self, f) {
            let [future, completer] = future();
            self.enqueued.push([f, completer]);
            future
        },
        spawn: fn(self, f) {
            let future = self.enqueue(f);
            if !running {
                self.run();
            }
        },
        cancel: fn(self) {
            self.cancled = true;
        },
        run: fn(self) {
            if self.running || self.cancled { return; }
            self.running = true;
            for [f, c] in self.enqueued.consume() {
                if self.cancled { break; }
                async {
                    self.outstanding += 1;
                    let r = f();
                    self.outstanding -= 1;
                    if c != nil {
                        c.complete(r);
                    }
                }
            }
            self.running = false;
        },
        yield: fn(self) {
            shift('async) k {
                self.enqueued.push([k, nil]);
                self.run();
            }
        }
    }
}
```

Here's an example of them in use:

This code will print "ping", "pong" for 3 seconds.

```ares
let co = coro();

co.enqueue {
    while true {
        print("ping");
        co.yield();
    }
};

co.enqueue {
    while true {
        print("pong");
        co.yield();
    }
};

co.enqueue {
    sleep(3_000);
    co.cancel();
}
co.run();
```

## "Actors"
Actors can be implemented via delimited continuations in a way that preserves
the sequential processing model that we are all used to when programming in
typical procedural languages.  Before we dive into the implementation of actors,
we first need to build an async-aware message-queue.

```ares
fn async_queue() {
    object {
        // objects on the queue
        q: [],
        // Waiting continuations
        waiting: [],
        // Send a message into the queue
        send: fn(self, m) {
            if self.waiting.is_empty() {
                self.q.push(m);
            } else {
                self.waiting.shift()(m);
            }
        },
        // Receive a message out of the queue.
        // This function may be synchronous or asynchronous
        // depending on if an item is already in the queue or not
        recv: fn(self) {
            if self.q.is_empty() {
                shift('async) k {
                    self.waiting.push(k);
                }
            } else {
                self.q.pop()
            }
        }
    }
}
```

Now we have an event queue that is synchronous when messages are buffered, but will
wait for a message asynchronously if there isn't anything buffered.

This event-queue is the backbone for actor communication.

```ares
fn actor(f) {
    let queue = async_queue();
    async {
        f(queue)
    };
    queue
}
```

That simple function creates a queue and passes it to the "actor function" in which it
is executed inside of an async block.

```ares
let a1 = actor(fn (messages) {
    while true {
        let m = messages.recv();
        print("a1 got: " + m);
    }
});

let a2 = actor(fn (messages) {
    while true {
        let m = messages.recv();
        print("a2 got: " + m);
        a1.send(m);
    }
});

a2.send("1");
a2.send("2");
a2.send("3");
```

This is sufficient for message passing, but it would be handy to have
an actual "actor" representation outside of just a queue.  This actor
object could track dead/alive state and also track which actor a message comes
from.

```ares
fn actor(f) {
    let queue = async_queue();
    let actor = object {
        queue: queue,
        dead: false,
        send: fn(self, message, sender) {
            if self.dead {
                return 'dead;
            }
            self.queue.send([message, sender]);
            return 'sent;
        },
        recv: fn(self) {
            self.queue.recv()
        },
    };

    async {
        f(actor);
        actor.dead = true;
    };

    actor
}
```

Now actors can send and receive messages from actors and optionally
pass along an actor reference to inform the receiver of which actor
sent the message.

```ares
let a1 = actor(fn(self) {
    while true {
        let [message, from] = self.recv();
        from.send("hi, I got " + message);
    }
});

let a2 = actor(fn(self) {
    while true {
        let [message, from] = self.recv();
        if from != null {
            a1.send(message, self);
        } else {
            print("got message back from a1");
        }
    }
});

a2.send("hi");
a2.send("bye");
```
