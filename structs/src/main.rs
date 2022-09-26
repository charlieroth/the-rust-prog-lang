fn main() {
    let w1 = 30;
    let h1 = 50;

    println!(
        "The area of the rectangle is {} square pixels",
        area(w1, h1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    return width * height
}
