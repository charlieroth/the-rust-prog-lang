# Crust of Rust: Smart Pointers and Interior Mutability

# Interior Mutability

#### Module `std::cell`, mutable memory location

Weird concept in Rust given its safe nature

[`std::cell`](https://doc.rust-lang.org/std/cell/struct.Cell.html) module 
allows you to do this in a controlled fashion

It does this via _interior mutability_; looks immutable from the outside but 
has methods that allow mutation
