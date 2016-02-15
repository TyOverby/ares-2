# Bind operator

Any language that has first-class functions and object methods has trouble
with the binding of the `this` parameter.

Here are how some other languages handle it:

### Javascript

Javascript has an implicit `this` that gets set as the object on the
left-hand-side of an object method call.

```javascript
function foo() {
    console.log(this.x);
}
var obj = {
    x: 5,
    f: foo,
};
obj.f(); // prints 5
```

However, this is awful when trying to compose functions in the form of
methods.

```javascript
// get an array of all 5s
arr.map(obj.f); // x is undefined
```

The fix for this is explicitly binding the function to the object.

```javascript
arr.map(obj.f.bind(f)); // this works now.
```

The binding is ugly.

### Python

Python gets away from this issue by having a clear distinction between functions
and methods.

The `self` parameter is explicit, but if found within a class, is implicitly bound.

```python
class Foo:
    def bar(self, a):
        print(self)
        print(a)

f = Foo()
f.bar(1) // prints <Foo instance>, 1

a = f.bar
a(1) // prints <Foo instance>, 1
```

But this is because python has a distinction between fields and methods.  A field
that is a function will not recieve this treatment.

```python
class Bar:
    def __init__(self):
        self.zoo = lambda self, x: print(self, x)

z = Bar()
z.zoo(5) // Error: lambda takes 2 arguments
```

### Lua

Lua does not have a distinction between hashmaps and objects, and therefore can
not do auto-binding like Python.  Instead, all function properties accessed through
a map have an unbound parameter, and there is no implicit "this".

```lua
obj = {}
obj.f = function (self, a) print(self, a) end
obj.f(1) // prints "1, nil"
obj.f(obj, 1) // prints "<Table>, 1"
obj:f(1) // prints "<Table>, a"
```

The shorthand "a:b()" is simply syntactic sugar for "a.b(a)" and doesn't do binding
at all.  "(a:b)(1)" is a syntax error.

### Ruby

Ruby is pretty much exactly like python.

### Ares

Ok, now that we have some prior art to look at, where does ares stand?

// TODO: write this section

#### Binding operator

In ares, functions have an explicit "this" parameter as the first argument to the
function. (in reality, it can be named whatever you want).


```ares
o.foo()
o.foo

o!foo()
my_list!map(double)!filter(even);
```
