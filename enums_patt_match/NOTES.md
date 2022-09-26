# Enums and Pattern Matching

Define a type by enumerating its possible _variants_

### The `Option` Enum and Its Advantages Over Null Values

The `Option` type (type defined in prelude) encodes the scenario in which a
value could be something or it could be nothing

`null` is a value that means there is no value there

Rust does not have the `null` feature

`Option` enum definition:
```
enum Option<T> {
    None,
    Some(T),
}
```
