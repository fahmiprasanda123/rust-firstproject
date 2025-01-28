// basic.rs  
  
pub fn add(a: i32, b: i32) -> i32 {  
    a + b  
}  
  
pub enum Direction {  
    Up,  
    Down,  
    Left,  
    Right,  
}  
  
pub fn move_player(direction: Direction) -> String {  
    match direction {  
        Direction::Up => "Moving up!".to_string(),  
        Direction::Down => "Moving down!".to_string(),  
        Direction::Left => "Moving left!".to_string(),  
        Direction::Right => "Moving right!".to_string(),  
    }  
}  
