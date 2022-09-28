use std::collections::HashMap;

fn main() {
    // instantiate hash map
    let mut scores = HashMap::new();

    // insert values into hash map
    scores.insert(String::from("game 01"), 10);
    scores.insert(String::from("game 02"), 20);
    println!("scores: {:?}", scores);

    // access value in hash map
    let game_key = String::from("game 01");
    let score = scores.get(&game_key).copied().unwrap_or(0);
    println!("score of game 01 is: {}", score);
    
    // overwrite value
    scores.insert(String::from("game 01"), 30);

    // iterate hash map
    for (k, v) in &scores {
        println!("{}: {}", k, v);
    }

    // add k/v only if key is not present
    scores.entry(String::from("game 03")).or_insert(90);
    println!("scores: {:?}", scores);
}
