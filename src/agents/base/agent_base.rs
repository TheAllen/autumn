use crate::agents::base::agent_traits::BasicAgentTraits;
use crate::models::general::llm::Message;

#[derive(Debug, PartialEq)]
pub enum AgentState {
    Discovery,
    Working,
    UnitTesting,
    Finished,
}

#[derive(Debug)]
pub struct AgentAttributes {
    pub objective: String,
    pub position: String,
    pub state: AgentState,
    pub memory: Vec<Message>,
}

impl AgentAttributes {
    pub fn new(objective: String, position: String) -> Self {
        AgentAttributes {
            objective,
            position,
            state: AgentState::Discovery,
            memory: Vec::new(),
        }
    }
}

impl BasicAgentTraits for AgentAttributes {
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
