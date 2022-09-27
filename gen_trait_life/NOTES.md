# Generic Types, Traits and Lifetimes

## Generic Types
Rust uses _Generics_ to handle abstract stand-ins for concrete types or other
properties

Generics have no runtime performance implications. This is achieved by
performing monomorphization of the code using generics at compile time

_Monomorphization_ is the process of turning generic code into specific
code by filling in the concrete types that are used when compiled

## Traits
A _trait_ defines the functionality a type has and can share with other types

Use traits to define shared behavior in an abstract way

Traits are similar to _interfaces_ in other languages

## Lifetimes
A _lifetime_ is the scope for which a reference is valid

Every reference in a Rust program has a _lifetime_

Lifetimes are a kind of generic that, rather than ensuring that a type has 
the behavior we want, ensures that references are valid as long as we need
them to be

Lifetimes' main purpose are to prevent _dangling references_

Lifetime Annotation Syntax:
```
&i32         // a reference
&'a i32      // a reference with an explicit lifetime
&'a mut i32  // a mutable reference with an explicit lifetime
```

### The Static Lifetime

`'static` is a lifetime denotion that the affected reference can live for the
duration of the program

### Borrow Checker

Compares scopes to determine whether all borrows are valid
