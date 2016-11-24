# Ares

### A scripting language built for easy integration with Rust, with a focus on first-class controll flow.

First and foremost, Ares is a scripting language built for easy integration
with Rust, C, and any other language that has good C bindings.  Ares has been
heavily influenced by Lua and Lisp, and takes great pride in being minimalist
to the core.

To the goal of having a very small core, Ares has Tagged Delimited Continuations,
a language feature that allow other language features to be built as libraries.
For example, Ares does not support exceptions, generators, or async/await at
the language level, but instead are a part of the standard library.  Tagged
Delimited Continuations are a very powerful construct, but are composable in a
way that you may never actually see them in use!  Instead, a developer will use
them transparently through code that other programmers have written.

## TODO:
* Finish *list* implementation
* Write *object* implementation
* Write *module* loader
* Write *module* API
* Write *event* loop
* Write *event loop* API

## Plan:
Write module loader, from there implement most "methods" on list and object
as standard library modules.
