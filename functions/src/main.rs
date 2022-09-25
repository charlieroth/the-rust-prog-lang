fn main() {
    // Expressions evaluate to a value
    // 
    // - Calling a function is an expression
    // - Calling a macro is an expression
    // - A new scope block with curly brackets is an expression

    let y = {
        let x = 3;
        x + 1
    };

    println!("y = {y}");
}

