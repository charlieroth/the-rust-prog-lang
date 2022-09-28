fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("v: {:?}", v);
    
    for i in &mut v {
        *i += 10;
    }
    println!("v: {:?}", v);

    let v2 = vec![1, 2, 3];
    println!("v2: {:?}", v2);

    for i in &v2 {
        println!("{}", i);
    }
}
