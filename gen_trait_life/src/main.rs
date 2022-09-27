fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    
    for number in list {
        if number > largest {
            largest = number;
        }
    }

    return largest;
}

fn main() {
    let list = vec![1, 2, 3, 4, 5];
    let lrg = largest(&list);
    println!("The largest number is {}", lrg);
    
    let list = vec![83, 32, 54, 2, 39];
    let lrg = largest(&list);
    println!("The largest number is {}",lrg);
}
