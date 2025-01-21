// Fungsi untuk menjumlahkan dua angka  
pub fn add(a: i32, b: i32) -> i32 {  
    a + b  
}  
  
// Struct untuk menyimpan data orang  
pub struct Person {  
    pub name: String,  
    pub age: u32,  
}  
  
// Enum untuk arah  
pub enum Direction {  
    Up,  
    Down,  
    Left,  
    Right,  
}  
  
// Fungsi untuk memindahkan pemain  
pub fn move_player(direction: Direction) -> String {  
    match direction {  
        Direction::Up => "Moving up!".to_string(),  
        Direction::Down => "Moving down!".to_string(),  
        Direction::Left => "Moving left!".to_string(),  
        Direction::Right => "Moving right!".to_string(),  
    }  
}  
