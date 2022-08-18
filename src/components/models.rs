#[derive(PartialEq, Clone, Default)]
pub struct Drink {
    title: String,
    rank: i32,
    booz: String,
    directions: String,
    ingredients: Vec<String>,
}
