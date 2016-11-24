## Modules

### General Strategy

A "module" in ares is a tuple of (name, version).  Loading these modules
is the direct responsibility of the user of the Ares library.
Libraries are loaded directly through source files, though it is feasable
for stdlib libraries to load themselves directly via precompiled assembly.

```rust
ctx.load_lib("name", "1.0", "source code");
```

## Lazyness

A library loaded like above is loaded "lazily".  This means that the
source code is only evaluated when a script or another library requests it.

This allows library authors to load multiple versions of their library without
worrying about side-effects.

## FFI Libraries

A common request would be to load FFI functions into a library.  I expect most
libraries that have FFI functions be consolidated into a single library.
For this use case, a macro could be used to easily load a bunch of rust functions.

```rust
ctx.load_lib("name", "1.0", ffi_lib!{
  "foo": |args, ctx| { ... },
  "bar": |args, ctx| { ... },
});
```
