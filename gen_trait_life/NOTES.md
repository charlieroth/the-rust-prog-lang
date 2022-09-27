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
