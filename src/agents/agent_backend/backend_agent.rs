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
    }, ai_functions::ai_functions::print_backend_webserver_code, utils::{general::{read_code_template, save_code_to_file}, llm_apis::request_task_llm}
};
use dotenv::dotenv;
use std::env;


#[derive(Debug)]
pub struct BackendAgent {
    attributes: AgentAttributes,
}

impl BackendAgent {
    pub fn new(objective: String, position: String) -> Self {
        let attributes: AgentAttributes = AgentAttributes::new(objective, position);
        Self { attributes }
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
                    self.call_initial_backend_code(proj_spec);
                    self.attributes.update_agent_state(AgentState::Working);
                },
                AgentState::Working => {
                },
                AgentState::UnitTesting => {
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
