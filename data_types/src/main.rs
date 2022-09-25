#[allow(unused_variables)]
fn main() {
    // Every value in Rust is of a certain data type
    // Two data type subsets: scalar and compound
    //
    // Rust is a statically typed language therefore the types of all
    // variables at compile time. The compiler can almost always infer 
    // the type of a variable but sometime an explicit type declaration
    // is required

    // ==== Scalar Types ====
    // Represents a single value
    // 
    // Four primary scalar types: intergers, floating-point numbers, 
    // booleans, characters
    let x = 2;        // i32
    let y: f32 = 3.0; // f32
    let z = false;    // bool
    let c = 'c';      // char

    // ==== Compound Types ====
    // Can group multiple values into one type
    // 
    // Two primitive compound types:
    // - Tuples
    // - Arrays
    //
    // Arrays are useful when you want your data allocated on the stack
    // rather than the heap or when you want to have a fixed number of
    // elements
    let tup: (i32, f64, u8) = (500, 6.4, 1); // (i32, f64, u8)
    let arr = [1, 2, 3, 4, 5];               // [i32; 5]
}
