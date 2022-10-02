// Iterators in Rust are implemented as a trait, `Iter<T>`
fn main() {
    let v = vec![1, 2, 3, 4];

    // Iterators can be consumed by loops or other useful methods
    // on the `Iter<T>` trait
    //
    // Calculate sum of `v` via `.reduce(..)`
    // 
    // Need to use `into_iter()` to allow for iteration over any of the moved
    // values (`T`), immutable references, or mutable references
    let sum = v.into_iter().reduce(|acc, num| {
        return acc + num;
    }).unwrap();
    println!("sum: {}", sum);
} 
