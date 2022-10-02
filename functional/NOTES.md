# Functional Language Features

The implementation of closures and iterators do not effect runtime performance

## Closures

Anonymous functions that can be saved in a variable or passed as arguments to
other functions

Can be used to capture the envrionment they're defined in for later use

## Iterators

In Rust iterators are lazy, meaning they have no effect until you call
methods that consume the iterator

#### Creating Iterators

`iter()` provides iteration over immutable references `&T`

`iter_mut()` provides iteration over mutable references `&mut T`

`into_iter()` provides iteration over moved values `T`, immutable references,
or mutable references
