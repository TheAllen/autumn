use async_trait::async_trait;
use crate::{agents::base::agent_base::{AgentAttributes, AgentState}, models::general::llm::Message};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

pub trait BasicAgentTraits {
    fn print_agent_attributes(&self);
    fn print_position(&self);
    fn update_agent_state(&mut self, new_state: AgentState);
    fn get_agent_state(&self) -> &AgentState;
    fn get_agent_position(&self) -> &String;
    fn get_agent_objective(&self) -> &String;
    fn get_agent_memory(&self) -> &Vec<Message>;
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct RouteObject {
    pub is_route_dynamic: String,
    pub method: String,
    pub request_body: serde_json::Value,
    pub response: serde_json::Value,
    pub route: String,
}

#[derive(Debug, Serialize)]
pub struct ProjectSpec {
    pub project_description: Option<String>,
    pub project_scope: Option<ProjectScope>,
    pub external_urls: Option<Vec<String>>,
    pub backend_code: Option<String>,
    pub frontend_code: Option<String>,
    pub api_endpoint_schema: Option<Vec<RouteObject>>,
}

impl ProjectSpec {
    pub fn new(
        project_description: Option<String>,
        project_scope: Option<ProjectScope>,
        external_urls: Option<Vec<String>>,
        backend_code: Option<String>,
        frontend_code: Option<String>,
        api_endpoint_schema: Option<Vec<RouteObject>>,
    ) -> Self {
        Self {
            project_description,
            project_scope,
            external_urls,
            backend_code,
            frontend_code,
            api_endpoint_schema,
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct ProjectScope {
    pub is_crud_required: bool,
    pub is_user_login_and_logout: bool,
    pub is_external_urls_required: bool
}

// TODO: Double check if the async_trait is necessary for this
#[async_trait]
pub trait SpecialFunction: Debug {

    // Manager can get attributes of other agents
    fn get_attributes_from_agent(&self) -> &AgentAttributes;

    // The function in which all agents will execute their logic in
    async fn execute(
        &mut self, 
        proj_spec: &mut ProjectSpec
    ) -> Result<(), Box<dyn std::error::Error>>;
}
