#[macro_use]  
extern crate rocket;  
  
mod basic; // Mengimpor modul basic  
  
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

#[launch]  
fn rocket() -> _ {  
    rocket::build()  
        .configure(rocket::Config {  
            address: "0.0.0.0".parse().unwrap(),
            port:8000,  //docker
            // port: 8080,  //cargo run
            ..Default::default()  
        })  
        .mount("/", routes![index, add, move_player])  
}  
