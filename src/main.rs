#[macro_use]  
extern crate rocket;  
  
mod basic; // Mengimpor modul basic  
mod ownership_borrowing; // Mengimpor modul ownership_borrowing  
mod lifetimes; // Mengimpor modul lifetimes  
mod error_handling; // Mengimpor modul error_handling  
mod traits_generics; // Mengimpor modul traits_generics  
mod collections; // Mengimpor modul collections  
  
#[get("/")]  
fn index() -> &'static str {  
    "Hello, world!"  
}  
  
#[get("/lifetimes")]  
fn lifetimes_example() -> &'static str {  
    lifetimes::lifetimes_example();  
    "Lifetimes example executed!"  
}  
  
#[get("/error")]  
fn error_handling_example() -> &'static str {  
    error_handling::error_handling_example();  
    "Error handling example executed!"  
}  
  
#[get("/traits")]  
fn traits_example() -> &'static str {  
    traits_generics::traits_generics_example();  
    "Traits example executed!"  
}  
  
#[get("/collections")]  
fn collections_example() -> &'static str {  
    collections::collections_example();  
    "Collections example executed!"  
}  
  
// Start basic  
#[get("/add/<a>/<b>")]  
fn add(a: i32, b: i32) -> String {  
    let sum = basic::add(a, b);  
    format!("The sum of {} and {} is: {}", a, b, sum)  
}  
  
#[get("/move/<direction>")]  
fn move_player(direction: String) -> String {  
    let dir = match direction.as_str() {  
        "up" => basic::Direction::Up,  
        "down" => basic::Direction::Down,  
        "left" => basic::Direction::Left,  
        "right" => basic::Direction::Right,  
        _ => return "Invalid direction".to_string(),  
    };  
    basic::move_player(dir)  
}    
// End basic  
  
// Ownership borrowing start  
#[get("/ownership")]  
fn ownership_example() -> &'static str {  
    ownership_borrowing::ownership_example();  
    "Ownership example executed!"  
}  
  
#[get("/borrowing")]  
fn borrowing_example() -> &'static str {  
    ownership_borrowing::borrowing_example();  
    "Borrowing example executed!"  
}  
// End ownership borrowing  
  
// Untuk launch route  
#[launch]  
fn rocket() -> _ {  
    rocket::build()  
        .configure(rocket::Config {  
            address: "0.0.0.0".parse().unwrap(),  
            port: 8000,  //docker  
            // port: 8080,  //cargo run  
            ..Default::default()  
        })  
        // Untuk mengatur route  
        .mount("/", routes![index, add, move_player, ownership_example, borrowing_example, lifetimes_example, error_handling_example, traits_example, collections_example])  
}  
