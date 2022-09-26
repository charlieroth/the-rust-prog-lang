fn main() {
    let rectangle = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels",
        area(rectangle)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    return dimensions.0 * dimensions.1;
}
