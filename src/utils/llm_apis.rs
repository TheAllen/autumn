use dotenv::dotenv;
use crate::models::general::llm::{APIResponse, ChatCompletion, Message};
use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue};
use std::env;


/// This function is the main way to interface with OpenAI's GPT 4 model
/// 
///
pub async fn call_gpt(messages: Vec<Message>) -> Result<String, Box<dyn std::error::Error + Send>> {
    dotenv().ok();
    
    // OpenAI URL
    let url: String = env::var("OPEN_AI_URL").expect("Could not find OPENAI url from .env file");

    // OpenAI Organization
    let org: String = env::var("OPEN_AI_ORG").expect("Could not find OPENAI org from .env file");

    // OpenAI Key
    let key: String = env::var("OPEN_AI_KEY").expect("Could not find OPENAI key from .env file");

    // LLM model
    let model: String = env::var("LLM_MODEL").expect("Could not find LLM model from .env file");

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

    let client: Client = Client::builder()
        .default_headers(header_map)
        .build()
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;

    let chat_completion: ChatCompletion = ChatCompletion::new(model, messages);

    let res: APIResponse = client
        .post(&url)
        .json(&chat_completion)
        .send()
        .await.map_err(|e| -> Box<dyn std::error::Error + Send> {Box::new(e)})?
        .json()
        .await.map_err(|e| -> Box<dyn std::error::Error + Send> {Box::new(e)})?;

    let content = res.choices[0].message.content.clone();

    Ok(content)
}       

// Converts function structure to static string reference
pub fn api_instruction_wrapper(func: fn(& str) -> &'static str, user_input: &str) -> Message {
    let ai_func: &str = func(user_input);

    // Instruction to the LLM
    let msg: String = format!(
        "FUNCTION: {}
        INSTRUCTION: You are a function printer, You ONLY print the results of functions.
        Nothing else. No commentary. Here is the input of the function: {}.
        Print out what the function will return.",
        ai_func, user_input
    );

    Message {
        role: "system".to_string(),
        content: msg
    }
}


#[cfg(test)]
mod tests{
    use super::*;
    use crate::ai_functions::ai_functions::print_project_two_scope;

    #[tokio::test]
    async fn tests_call_gpt() {
        let msg: Message = Message {
            role: "user".to_string(),
            content: "Hi - this is just a test. Give me the shortest response possible".to_string(),
        };
        let test = call_gpt(vec![msg]).await;
        
        if let Ok(res) = test {
            dbg!(res);
        } else {
            panic!("Something went wrong with tests_cal_gpt");
        }
    }

    #[test]
    fn tests_api_wrapper() {
        let func_str = api_instruction_wrapper(print_project_two_scope, "TESTING");
        dbg!(func_str);
        
    }
}