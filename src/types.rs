use serde::{Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(Serialize)]
pub struct Person {
    pub name: String,
    pub age: u32,
    pub favourite_food: Option<String>,
}