// traits_generics.rs  
  
pub trait Summary {  
    fn summarize(&self) -> String;  
}  
  
pub struct NewsArticle {  
    pub headline: String,  
    pub location: String,  
    pub author: String,  
    pub content: String,  
}  
  
impl Summary for NewsArticle {  
    fn summarize(&self) -> String {  
        format!("{} by {} in {}", self.headline, self.author, self.location)  
    }  
}  
  
pub fn traits_generics_example() {  
    let article = NewsArticle {  
        headline: String::from("Rust is awesome!"),  
        location: String::from("The Internet"),  
        author: String::from("John Doe"),  
        content: String::from("Rust is a systems programming language."),  
    };  
  
    println!("New article available! {}", article.summarize());  
}  
