# Reflection

Reflection is a method for abstraction developers to access raw properties on
objects as well as re-defining core behaviors such as method calls,
field access, and operators like Indexing ("obj[1]") and Calling ("obj()").


## Custom Setters

By overwriting field sets, you can run arbitrary code whenever a field is
being set to a new value.  Here's an example of a message being printed
on every write.

```ares
import slot_set, custom_setter from std.reflection;

let obj = object {
    a: 5
};

obj!custom_setter(fn (field_name, value) {
    obj!slot_set(field_name, value);
    print(field_name + " set to " + value);
});

obj.a = 10; // 'a set to 10
obj.b = 20; // 'b set to 20
```

This can be used (abused?) to build a nice data-binding abstraction.

```ares
import get_slot, set_slot, custom_setter from std.reflection;
import on_next_tick from std.async;
import remove_where from std.list;
import fail from std.failure;

const key = gensym();

fn make_observable(object) {
    // make an invisible field on the object to store bindings.
    let listeners = [];

    obj!set_slot(key, listeners);
    obj!custom_setter(fn (field_name, value) {
        obj!set_slot(field_name, value);
        for [name, callback, _] in listeners {
            if name == field_name {
                on_next_tick {
                    callback(value);
                }
            }
        }
    });
}

fn listen(object, field_symbol)(callback) {
    let listeners = object!get_slot(key);
    if listeners == null { fail("object is not listenable."); }

    let subscribe_key = gensym();
    listeners.push([field_symbol, callback, subscribe_key]);
    return subscribe_key;
}

fn unsubscribe(object, subscribe_key) {
    let listeners = object!get_slot(key);
    if listeners == null { fail("object is not listenable."); }
    listeners!remove_where((fn (trio) { trio[2] == subscribe_key }))
}
```

Now we can fairly easily write code that detects changes in an object.

```ares
let obj = object {
    a: 5,
    b: 6
};

// Make the object observable
obj!make_observable();

// Listen to changes on field "a"
obj!listen('a) {
    print(obj.a);
};

// Alternate syntax for the same thing
var unsub = obj!listen('b)(fn (value) {
    print(value);
});

// Make some changes to the object
obj.a = 10;
obj.b = "hi";

// Unsubscribe the "b" listener.
obj!unsubscribe(unsub);
```
