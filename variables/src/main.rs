fn main() {
    // Shadowing is different from marking a variable as `mut`, because we'll
    // get a compile-time error if we accidentally try to reassign to this
    // reassign to this variable without using the `let` keyword
    //
    // The other difference between `mut` and shadowing is that because we're
    // effectively creating a new variable when we use the `let` keyword again,
    // we can change the type of the value but reuse the same name
    let x = 5;
    let x = x + 1;
    {
        // x == 8
        let x = x + 2;
        println!("x in the inner scope is: {x}")
    }
    // x == 6
    println!("x is: {x}")
}
