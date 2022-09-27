# Generic Types, Traits and Lifetimes

Rust uses _Generics_ to handle abstract stand-ins for concrete types or other
properties

Generics have no runtime performance implications. This is achieved by
performing monomorphization of the code using generics at compile time

_Monomorphization_ is the process of turning generic code into specific
code by filling in the concrete types that are used when compiled
