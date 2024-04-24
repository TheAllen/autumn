use async_trait::async_trait;
use crate::{
    agents::base::{
        agent_base::{
            AgentAttributes, 
            AgentState
        }, 
        agent_traits::{
            BasicAgentTraits,
            ProjectSpec, 
            SpecialFunctions
        }
    }, 
    ai_functions::ai_functions::{print_backend_webserver_code, print_improved_webserver_code}, 
    utils::{command_line::{confirm_safe_code, PrintMessage}, general::{read_code_template, save_code_to_file}, llm_apis::request_task_llm}
};
use dotenv::dotenv;
use std::{env, process::{Command, Stdio}};


#[derive(Debug)]
pub struct BackendAgent {
    attributes: AgentAttributes,
    bug_errors: Option<String>,
    bug_count: u8
}

impl BackendAgent {
    pub fn new(objective: String, position: String) -> Self {
        let attributes: AgentAttributes = AgentAttributes::new(objective, position);
        Self { 
            attributes, 
            bug_errors: None, 
            bug_count: 0 
        }
    }

    async fn call_initial_backend_code(&mut self, proj_spec: &mut ProjectSpec) {
        dotenv().ok();
        let filepath: String = env::var("CODE_FILEPATH").expect("Could not find CODE_FILEPATH value from .env");
        let template_code: String = read_code_template(&filepath);

        let user_req: String = format!(
            "CODE TEMPLATE: {} \n PROJECT DESCRIPTION: {} \n",
            template_code, proj_spec.project_description.as_ref().expect("Project description is missing").to_string()
        );

        let content = request_task_llm(
            print_backend_webserver_code, 
            user_req, 
            &self.attributes.position, 
            get_function_string!(print_backend_webserver_code)
        ).await;

        let output_file: String = env::var("CODE_OUTPUT_FILEPATH")
            .expect("Could not find CODE_OUTPUT_FILEPATH value from .env");
        
        save_code_to_file(&output_file, &content);
        proj_spec.backend_code = Some(content);
    }

    async fn improve_backend_code(&mut self, proj_spec: &mut ProjectSpec) {
        dotenv().ok();

        let msg_context: String = format!(
            "CODE TEMPLATE: {:?} \n PROJECT SPECIFICATIONS: {:?} \n",
            proj_spec.backend_code, proj_spec
        );

        // Get LLM response
        let content: String = request_task_llm(
            print_improved_webserver_code,
            msg_context,
            &self.attributes.position,
            get_function_string!(print_improved_webserver_code)
        ).await;

        let output_file: String = env::var("CODE_OUTPUT_FILEPATH")
            .expect("Could not find CODE_OUTPUT_FILEPATH value from .env");

        save_code_to_file(&output_file, &content);
        proj_spec.backend_code = Some(content);
    }

    async fn fix_backend_bugs(&mut self, proj_spec: &mut ProjectSpec) {

    }
}

#[async_trait]
impl SpecialFunctions for BackendAgent {
    fn get_attributes_from_agent(&self) -> &AgentAttributes {
        &self.attributes
    }

    async fn execute(
        &mut self, 
        proj_spec: &mut ProjectSpec
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Keep execution until AgentState::Finished state has been reached
        // Take project description and code template and generate backend code
        while self.attributes.state != AgentState::Finished {
            match self.attributes.state {
                AgentState::Discovery => {
                    self.call_initial_backend_code(proj_spec).await;
                    self.attributes.update_agent_state(AgentState::Working);
                    continue;
                },
                AgentState::Working => {
                    if self.bug_count == 0 {
                        self.improve_backend_code(proj_spec).await;
                    } else {
                        self.fix_backend_bugs(proj_spec).await;
                    }
                    self.attributes.update_agent_state(AgentState::UnitTesting);
                    continue;
                },
                AgentState::UnitTesting => {
                    PrintMessage::Testing.print_agent_msg(
                        &self.attributes.position, 
                        "Testing backend code: Ensuring safe code..."
                    );

                    let is_safe_code: bool = confirm_safe_code();
                    if !is_safe_code {
                        // Exit program
                        panic!("Better work on some AI alignment.");
                    }

                    PrintMessage::Testing.print_agent_msg(
                        &self.attributes.position, 
                        "Backend code united testing: Building web server..."
                    );

                    // Runs the command `cargo build`
                    let build_backend_server: std::process::Output = Command::new("cargo")
                        .arg("build")
                        .current_dir(env::var("CODE_OUTPUT_FILEPATH").expect("Could not find CODE_OUTPUT_FILEPATH in .env file"))
                        .stdout(Stdio::piped())
                        .stdout(Stdio::piped())
                        .output()
                        .expect("Failed to build backend application code");

                    // Determin if build errors
                    if build_backend_server.status.success() {
                        self.bug_count = 0;
                        PrintMessage::Testing.print_agent_msg(
                            self.attributes.position.as_str(), 
                            "Test server build successfully..."
                        );
                    } else {

                    }
                },
                _ => {
                    self.attributes.update_agent_state(AgentState::Finished);
                }
            }
        }
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_backend_agent() {
        let backend_agent = BackendAgent::new(
            "Build server side application".to_owned(),
            "Backend Agent".to_owned()
        );

        dbg!(backend_agent);
    }
}
