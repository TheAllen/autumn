use crate::agents::base::agent_base::{AgentAttributes, AgentState};
use crate::agents::base::agent_traits::{ProjectSpec, SpecialFunction};

#[derive(Debug)]
pub struct ManagerAgent {
    attributes: AgentAttributes,
    project_spec: ProjectSpec,
    agents: Vec<Box<dyn SpecialFunction>>, // list of agents manager is managing
}

impl ManagerAgent {

    pub async fn new(user_input: String) -> Result<Self, Box<dyn std::error::Error>> {
        // Initializing manager agent attributes
        let attributes: AgentAttributes = AgentAttributes::new(
            "manage agents that are building the website for the end user".to_string(),
            "Project Manager".to_string()
        );

        // Creating the project spec object for other agents to use
        let project_spec: ProjectSpec = ProjectSpec::new(
            user_input, // TODO: change the user input once running through GPT API
            None, // Defined by Solutions Architect
            None,
            None,
            None,
            None
        );

        let agents: Vec<Box<dyn SpecialFunction>> = vec![];

        Ok(Self {
            attributes,
            project_spec,
            agents
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn tests_creating_managing_agent() {
        let managing_agent = ManagerAgent::new("Build a simple weather app".to_string()).await;

        dbg!(managing_agent);
    }
}