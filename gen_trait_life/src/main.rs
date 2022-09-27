// Error due to lack of implementation of std::cmp::PartialEq
// but this is how Generics are used in a function definition
fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];
    
    for item in list {
        if item > largest {
            largest = item;
        }
    }

    return largest;
}

fn main() {
    let list = vec![1, 2, 3, 4, 5];
    // Notice there is no generic written in function call syntax
    // Compiler will infer this typing
    let lrg = largest(&list);
    println!("The largest number is {}", lrg);
    
    let list = vec!['c','h','a','r','l','i','e'];
    let lrg = largest(&list);
    println!("The largest number is {}",lrg);
}
