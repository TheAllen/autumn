use dotenv::dotenv;
use serde::de::DeserializeOwned;
use crate::agents::base::agent_traits::ProjectScope;
use crate::models::general::llm::{APIResponse, ChatCompletion, Message};
use crate::utils::command_line::PrintMessage;
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
fn api_instruction_wrapper(func: fn(&str) -> &'static str, user_input: &str) -> Message {
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

// Request to GPT or LLM to get response in string
pub async fn request_task_llm(
    ai_func: fn(&str) -> &'static str,
    user_req: String,
    agent_position: &str,
    agent_operation: &str
) -> String {
    let req_str: Message = api_instruction_wrapper(ai_func, &user_req);

    // Make a request to LLM GPT
    let llm_res = call_gpt(vec![req_str.clone()]).await;

    match llm_res {
        Ok(res) => res,
        Err(e) => {
            println!("Error calling the LLM: {}", e);
            println!("Calling the GPT again...");
            call_gpt(vec![req_str.clone()]).await.expect("Failed to call LLM twice")
        },
    }
}

// Request to GPT or LLM to get response in flexible types
pub async fn request_task_llm_deserialized<T: DeserializeOwned>(
    ai_func: fn(&str) -> &'static str,
    user_req: String,
    agent_position: &str,
    agent_operation: &str
) -> T {
    let llm_res_str = request_task_llm(ai_func, user_req, agent_position, agent_operation).await;
    
    let deserialized_obj: T = serde_json::from_str(llm_res_str.as_str()).expect("Failed to decode LLM response.");

    deserialized_obj
}


#[cfg(test)]
mod tests{

    use crate::ai_functions::ai_functions::print_project_scope;

    use super::*;

    #[tokio::test]
    async fn example_call_gpt() {
        let sample_request_gpt = request_task_llm(
            print_project_scope, 
            "Build me a simple todo app with get and post request endpoints".to_string(),
            "Project Manager",
            get_function_string!(print_project_scope)
        ).await;
        dbg!(sample_request_gpt);
    }

    #[tokio::test]
    async fn tests_call_gpt() {
        let msg: Message = Message {
            role: "user".to_string(),
            content: "Hi, this is just a test. Give me the shortest response possible".to_string(),
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
        let func_str = api_instruction_wrapper(print_project_scope, "TESTING");
        dbg!(func_str);
    }

    #[tokio::test]
    async fn tests_request_task_llm() {
        let project_req = "I want to build a application that allows me to forecast stock and crypto data".to_string();
        let wrapped_req = request_task_llm(print_project_scope, project_req, "Project Manager", get_function_string!(print_project_scope)).await;
        dbg!(wrapped_req);
    }
}