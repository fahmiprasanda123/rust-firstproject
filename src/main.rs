#[macro_use]  
extern crate rocket;  

// Mengimpor modul basic  
mod basic; 
mod ownership_borrowing;
  
#[get("/")]  
fn index() -> &'static str {  
    "Hello, world!"  
}  
  
// start basic
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
// end basic

//ownership borrowing start
#[get("/ownership")]  
fn ownership() -> &'static str {  
    ownership_borrowing::ownership_example();  
    "Ownership example executed!"  
}  
  
#[get("/borrowing")]  
fn borrowing() -> &'static str {  
    ownership_borrowing::borrowing_example();  
    "Borrowing example executed!"  
} 
//end ownership borrowing


//untuk launch route
#[launch]  
fn rocket() -> _ {  
    rocket::build()  
        .configure(rocket::Config {  
            address: "0.0.0.0".parse().unwrap(),
            port:8000,  //docker
            // port: 8080,  //cargo run
            ..Default::default()  
        })
        //untuk mengatur route
        .mount("/", routes![index, add, move_player, ownership, borrowing])  
}  
