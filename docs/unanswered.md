# Use a bound (but not initialized variable?)

I want this to work:

```ares
let f = fn(a) {
    f(a - 1); // Use f even though we are currently defining f;
};
```

But I don't want this to work:

```ares
let x = {
    x + 5; // Use x even though we are currently defining x;
};
```

### Solution:

```ares
let f = fn recurse(a) { // have another name be visable.
    recurse(a - 1);
};
```

# No Global Blocks

```ares
// In the global environment
{
    let a = 5;
}
// a should be undefined here.
```
