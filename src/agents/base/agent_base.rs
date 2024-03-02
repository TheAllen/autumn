use crate::models::general::llm::Message;

use super::agent_traits::BasicAgentTraits;

#[derive(Debug, PartialEq)]
pub enum AgentState {
    Discovery,
    Working,
    UnitTesting,
    Finished
}

#[derive(Debug)]
pub struct AgentAttributes {
    pub objective: String,
    pub position: String,
    pub state: AgentState,
    pub memory: Vec<Message>
}

impl BasicAgentTraits for AgentAttributes {
    fn new(objective: String, position: String) -> Self {
        Self {
            objective,
            position,
            state: AgentState::Discovery,
            memory: Vec::new()
        }
    }

    fn print_agent_attributes(&self) {
        println!("{:?}", self);
    }

    fn print_position(&self) {
        println!("Current agent has position {}", self.position);
    }

    fn update_agent_state(&mut self, new_state: AgentState) {
        self.state = new_state;
    }

    fn get_agent_state(&self) -> &AgentState {
        &self.state
    }

    fn get_agent_position(&self) -> &String {
        &self.position
    }

    fn get_agent_objective(&self) -> &String {
        &self.objective
    }

    fn get_agent_memory(&self) -> &Vec<Message> {
        &self.memory
    }
}