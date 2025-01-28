// collections.rs  
  
pub fn collections_example() {  
    // Vector  
    let mut numbers = vec![1, 2, 3, 4, 5];  
    numbers.push(6);  
    println!("Numbers: {:?}", numbers);  
  
    // HashMap  
    use std::collections::HashMap;  
  
    let mut scores = HashMap::new();  
    scores.insert(String::from("Alice"), 50);  
    scores.insert(String::from("Bob"), 70);  
    println!("Scores: {:?}", scores);  
}  
