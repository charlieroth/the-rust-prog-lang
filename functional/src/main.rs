fn main() {
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    println!("{:?}", v);

    let v_even = filter_evens(v);
    println!("{:?}", v_even);
} 

fn filter_evens(v: Vec<i32>) -> Vec<i32> {
    return v.into_iter().filter(|item| {
        return item % 2 == 0;
    }).collect();
}
