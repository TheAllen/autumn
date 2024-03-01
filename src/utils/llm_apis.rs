use dotenv::dotenv;
use crate::models::general::llm::{APIResponse, Message};
use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue};
use std::env;


/// This function is the main way to interface with OpenAI's GPT 4 model
/// 
///
pub async fn call_gpt(message: Vec<Message>) -> Result<String, Box<dyn std::error::Error + Send>> {
    dotenv().ok();
    
    // OpenAI URL
    let url: String = env::var("OPEN_AI_URL").expect("Could not find OPENAI url from .env file");

    // OpenAI Organization
    let org: String = env::var("OPEN_AI_ORG").expect("Could not find OPENAI org from .env file");

    // OpenAI Key
    let key: String = env::var("OPEN_AI_KEY").expect("Could not find OPENAI key from .env file");

    // Create the Headers
    let mut header_map: HeaderMap = HeaderMap::new();

    header_map.insert("authorization",
        HeaderValue::from_str(&format!("Bearer {}", key).as_str())
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new (e) })? // propagate error up if any encountered
    );

    header_map.insert("OpenAI-Organization",
        HeaderValue::from_str(&org)
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?
    );

    let combine: String = vec![url, org, key].join("");

    Ok(combine)
}       

#[cfg(test)]
mod tests{
    use super::*;

    #[tokio::test]
    async fn tests_call_gpt() {

        let test = call_gpt(Vec::new()).await.unwrap();
        dbg!(test);
    }
}