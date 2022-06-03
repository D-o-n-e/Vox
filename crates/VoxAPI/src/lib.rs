use serde::{Deserialize, Serialize};
use serde_json::*;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct class{
    pub methods: Vec<String>,
    pub properties: Vec<String>
}
pub trait Script{
    fn _ready(&self);
}

pub fn parse_api() -> HashMap<String, class>{
    let data = include_str!("api.json");
    let parsed_data: HashMap<String, class> = from_str(data).unwrap();
    parsed_data
}
