pub mod repository;

pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}