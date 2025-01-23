pub fn ownership_example() {  
    let s1 = String::from("Hello, Rust!"); // s1 adalah pemilik string  
    let s2 = s1; // s2 sekarang menjadi pemilik, s1 tidak lagi valid  
  
    // println!("{}", s1); // Ini akan menyebabkan kesalahan karena s1 tidak valid  
    println!("{}", s2); // Ini valid  
}  

pub fn borrowing_example() {  
    let s1 = String::from("Hello, Borrowing!");  
  
    // Borrowing immutable  
    let len = calculate_length(&s1); // Meminjam s1  
    println!("The length of '{}' is {}.", s1, len); // s1 masih valid  
  
    // Borrowing mutable  
    let mut s2 = String::from("Hello");  
    change(&mut s2); // Meminjam s2 secara mutable  
    println!("Changed string: {}", s2); // s2 telah diubah  
}  
  
fn calculate_length(s: &String) -> usize {  
    s.len() // Mengembalikan panjang string  
}  
  
fn change(s: &mut String) {  
    s.push_str(", World!"); // Mengubah string yang dipinjam  
}  