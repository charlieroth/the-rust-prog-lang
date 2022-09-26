# What is Ownership?

A set of rules that governs how a Rust program manages memory

Memory is managed through a system of ownership with a set of rules
that the compiler checks

If any of the rules are violated, the program won't compile

Ownership has zero runtime cost


## Ownership Rules

1. Each value has an _owner_
2. There can only be one owner at a time
3. When the owner goes out of scope, the value will be dropped

### Memory and Allocation

The `String` type, in order to support a mutable, growable piece of text, we
need to allocate an amount of memory on the heap, unknown at compile time, to
hold the contents. The means:
- Memory must be requested from the memory allocator at runtime
- We need a way of returning this memory to the allocator when we're done with
the `String`

When a variable goes out of scope, Rust automatically calls the `drop` function
and cleans up the heap memory for that variable

```
// the string "hello" is owned by s1
let s1 = String::from("hello");

// the string "hello" is owned by s2, this is called a "move"
let s2 = s1;

// cannot use s1 here because transfer of ownership to s2
println!("{}, world!", s1);
```

Rust will never automatically create "deep" copies of your data 

Therefore, automatic copying can be assumed to be inexpensive in terms
of runtime performance

### Ways Variables and Data Interact: Clone

When you see a call to `clone`, you know that some arbitrary code is being
executed and that code may be expensive

Rust won't doesn't allow annotating a type with `Copy` trait if the type, or any
of it's parts, has implemented the `Drop` trait
