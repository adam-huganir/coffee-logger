use std::fmt::Display;

#[derive(Clone, Debug)]
enum Origin {
    Single(String),
    Blend(Vec<String>),
}

#[derive(Clone, Debug)]
enum RoastLevel {
    City,
    Light,
    Medium,
    Dark,
}

#[derive(Clone)]
struct Coffee {
    brand: String,
    country: String,
    origin: String,
    kind: Origin,
    roast: RoastLevel,
}

impl Display for Coffee {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}