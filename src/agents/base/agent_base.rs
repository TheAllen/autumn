#[derive(Debug, PartialEq)]
pub enum AgentState {
    Discovery,
    Working,
    UnitTesting,
    Finished
}

pub struct BaseAgent {
    objective: String,
    position: String
}