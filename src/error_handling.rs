// error_handling.rs  
  
use std::num::ParseIntError;  
  
pub fn parse_number(s: &str) -> Result<i32, ParseIntError> {  
    s.parse::<i32>()  
}  
  
pub fn error_handling_example() {  
    match parse_number("42") {  
        Ok(n) => println!("Parsed number: {}", n),  
        Err(e) => println!("Failed to parse number: {}", e),  
    }  
  
    match parse_number("not a number") {  
        Ok(n) => println!("Parsed number: {}", n),  
        Err(e) => println!("Failed to parse number: {}", e),  
    }  
}  
