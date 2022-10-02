# Functional Language Features

## Closures


## Iterators

In Rust iterators are lazy, meaning they have no effect until you call
methods that consume the iterator

#### Creating Iterators

`iter()` provides iteration over immutable references `&T`

`iter_mut()` provides iteration over mutable references `&mut T`

`into_iter()` provides iteration over moved values `T`, immutable references,
or mutable references
