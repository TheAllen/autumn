use crate::{agents::base::agent_base::AgentState, models::general::llm::Message};
use serde::{Deserialize, Serialize};

pub trait BasicAgentTraits {
    fn new(objective: String, position: String) -> Self;
    fn print_agent_attributes(&self);
    fn print_position(&self);
    fn update_agent_state(&mut self, new_state: AgentState);
    fn get_agent_state(&self) -> &AgentState;
    fn get_agent_position(&self) -> &String;
    fn get_agent_objective(&self) -> &String;
    fn get_agent_memory(&self) -> &Vec<Message>;
}

#[derive(Debug, Seriali)]
pub struct ProjectSpec {
    pub project_description: String,
    pub project_scope: Option<String>,
    pub external_urls: Option<Vec<String>>,
    pub backend_code: Option<String>,
    pub frontend_code: Option<String,
    pub api_endpoint_schema: Option<Vec<RouteObject>>
}
