use serde::Deserialize;

pub mod  client;

#[derive(Debug,Deserialize)]
pub struct GeminiResponse{
    pub candidates : Vec<Candidate>,
}

#[derive(Debug,Deserialize)]
pub struct Candidate{
    pub content : Content
}

#[derive(Debug, Deserialize)]
pub struct Content {
    pub parts: Vec<Part>,
}

#[derive(Debug, Deserialize)]
pub struct Part {
    pub text: String,
}   