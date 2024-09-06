use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
    pub title: String,
    pub body: String,
    pub userId: u32,
}