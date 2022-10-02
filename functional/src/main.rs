// Iterators in Rust are implemented as a trait, `Iter<T>`
fn main() {
    let v = vec![1, 2, 3, 4];

    // Create an iterator Iter<i32> from Vec<i32>
    let v_iter = v.iter();

    // Iterators can be consumed by loops or other useful methods
    // on the `Iter<T>` trait
    //
    // Calculate sum of `v` via a loop
    let sum: i32 = v_iter.sum();
    println!("sum: {}", sum); 
} 
