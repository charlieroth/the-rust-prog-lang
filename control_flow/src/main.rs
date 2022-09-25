fn main() {
    // Rust has three kinds of loops: `loop`, `while` and `for`

    let x = 5;
    let mut i = 0;

    loop {
        if i == x {
            break;
        }

        println!("again!");
        i += 1;
    }

    println!("-----------------------------------------------");
    
    i = 0;
    while i < x {
        println!("again!");
        i += 1;
    }
    
    println!("-----------------------------------------------");

    let a = [1, 2, 3, 4, 5];
    for element in a {
        println!("value: {element}");
    }
}
