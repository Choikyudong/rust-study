fn main() {
    println!("Hello, world!");
}

pub trait Summary {
    fn summarize(&self) -> String;
}

pub trait Animal {
    fn name(&self) -> String;
}

pub struct  Dog {
    pub name: String
}

impl Animal for Dog {
    fn name(&self) -> String {
        "Dog".to_string()
    }
}
