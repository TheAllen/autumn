use std::time::Duration;

use async_trait::async_trait;
use reqwest::Client;

use crate::{agents::base::{agent_base::{AgentAttributes, AgentState}, agent_traits::{BasicAgentTraits, ProjectScope, ProjectSpec, SpecialFunctions}}, ai_functions::ai_functions::print_site_urls, utils::{command_line::PrintMessage, llm_apis::{request_task_llm, request_task_llm_deserialized}}};
use crate::ai_functions::ai_functions::print_project_scope;
use crate::utils::general::check_status_code;

#[derive(Debug)]
pub struct ArchitectAgent {
    attributes: AgentAttributes
}

impl ArchitectAgent {
    pub fn new(objective: String, position: String) -> Self {
        let attributes: AgentAttributes = AgentAttributes::new(objective, position);
        Self {
            attributes
        }
    }

    // Generate project scope and update project specification
    async fn generate_project_scope(&mut self, project_spec: &mut ProjectSpec) -> ProjectScope {
        let project_description = project_spec.project_description.as_ref().expect("Project description not defined yet!");

        let project_scope = request_task_llm_deserialized::<ProjectScope>(
            print_project_scope,
            project_description.to_string(),
            &self.attributes.position,
            get_function_string!(print_project_scope)
        ).await;

        project_spec.project_scope = Some(project_scope.clone());
        self.attributes.update_agent_state(AgentState::Finished);
        project_scope
    }

    async fn generate_possible_external_urls(&self, project_spec: &mut ProjectSpec, msg_context: Option<String>) {
        
        let external_urls: Vec<String> = request_task_llm_deserialized::<Vec<String>>(
            print_site_urls, 
            msg_context.expect("Project description is missing!").to_string(), 
            self.attributes.get_agent_position(), 
            get_function_string!(print_site_urls)
        ).await;

        project_spec.external_urls = Some(external_urls);
    }

    // Check the validity of the external APIs
    async fn verify_possible_external_urls(&self, project_spec: &mut ProjectSpec) {

        let mut exclude_urls: Vec<String> = Vec::new();

        let client: Client = match Client::builder()
            .timeout(Duration::from_secs(5))
            .build()
            {
                Ok(c) => c,   
                Err(_) => panic!("Something went wrong when creating the Client")
            };

        // Find faulty urls from the provided urls
        let urls: &Vec<String> = project_spec
            .external_urls
            .as_ref()
            .expect("No URL object in Project Spec");

        for url in urls {
            let str_msg: String = format!("Testing URL endpoint: {}", url);
            PrintMessage::Testing.print_agent_msg(self.attributes.get_agent_position(), &str_msg);

            // check URL
            match check_status_code(&client, url).await {
                Ok(status_code) => {
                    if status_code != 200 {
                        exclude_urls.push(url.clone());
                    }
                },
                Err(e) => println!("Error checking {}: {}", url, e)
            }
        }

        // Exclude any faulty urls
        if exclude_urls.len() > 0 {
            let new_urls: Vec<String> = project_spec
                .external_urls
                .as_ref()
                .unwrap()
                .iter()
                .filter(|url| !exclude_urls.contains(&url))
                .cloned()
                .collect();
            project_spec.external_urls = Some(new_urls);
        }

    }
}

#[async_trait]
impl SpecialFunctions for ArchitectAgent {

    fn get_attributes_from_agent(&self) ->  &AgentAttributes {
        &self.attributes
    }

    async fn execute(&mut self, proj_spec: &mut ProjectSpec) -> Result<(), Box<dyn std::error::Error>> {
        // ! Warning - be careful of infinite loop !
        while self.attributes.state != AgentState::Finished {
            match self.attributes.state {
                AgentState::Discovery => { 
                    let project_scope = self.generate_project_scope(proj_spec).await;

                    // Check if there are external URLs we have to check
                    if project_scope.is_external_urls_required {
                        self.generate_possible_external_urls(proj_spec, proj_spec.project_description.clone()).await;
                        self.attributes.update_agent_state(AgentState::UnitTesting);
                    }

                },
                AgentState::UnitTesting => { // This stage will check verify the external URLs provided by GPT
                    self.verify_possible_external_urls(proj_spec).await;
                    self.attributes.update_agent_state(AgentState::Finished);
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
    fn tests_create_architect_agent() {
        let architect: ArchitectAgent = ArchitectAgent::new(
            "Gathers information and design solutions for website development".to_string(),
            "Solutions Architect".to_string()
        );
        dbg!(architect);
    }

    #[tokio::test]
    async fn tests_creation_project_scope() {
        let mut project_spec: ProjectSpec = ProjectSpec::new(
            Some("build a website that handles users logging in and logging out and accepts payments".to_string()),
            None,
            None,
            None,
            None,
            None
        );

        let mut architect: ArchitectAgent = ArchitectAgent::new(
            "Gathers information and design solutions for website development".to_string(),
            "Solutions Architect".to_string()
        );

        architect.generate_project_scope(&mut project_spec).await;

        dbg!(project_spec);
    }
}
