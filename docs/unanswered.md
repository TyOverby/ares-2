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
