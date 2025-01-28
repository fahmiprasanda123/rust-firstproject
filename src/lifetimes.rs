// lifetimes.rs  
  
pub fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {  
    if s1.len() > s2.len() {  
        s1  
    } else {  
        s2  
    }  
}  
  
pub fn lifetimes_example() {  
    let string1 = String::from("long string");  
    let string2 = String::from("short");  
  
    let result = longest(string1.as_str(), string2.as_str());  
    println!("The longest string is: {}", result);  
}  
