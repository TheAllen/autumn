use crate::{agents::base::agent_base::AgentState, models::general::llm::Message};

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