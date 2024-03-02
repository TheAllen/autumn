use crate::agents::base::agent_base::AgentState;

pub trait BasicAgentTraits {
    fn new(objective: String, position: String) -> Self;
    fn print_object(&self) -> String;
    fn print_position(&self) -> String;
    fn update_agent_state(&mut self, new_state: AgentState);
}