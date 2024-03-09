use crate::agents::base::agent_base::{AgentAttributes, AgentState};
use crate::agents::base::agent_traits::{ProjectSpec, SpecialFunction};
use crate::utils::llm_apis::request_task_llm;
use crate::ai_functions::ai_functions::convert_user_input_to_goal;


#[derive(Debug)]
pub struct ManagerAgent {
    attributes: AgentAttributes,
    project_spec: ProjectSpec,
    agents: Vec<Box<dyn SpecialFunction>>, // list of agents manager is managing
}

impl ManagerAgent {

    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Initializing manager agent attributes
        let attributes: AgentAttributes = AgentAttributes::new(
            "manage agents that are building the website for the end user".to_string(),
            "Project Manager".to_string()
        );

        // Creating the project spec object for other agents to use
        let project_spec: ProjectSpec = ProjectSpec::new(
            None,
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

    pub async fn articulate_project_description(&mut self, user_req: String) {

        let project_description: String = request_task_llm(convert_user_input_to_goal, user_req).await;

        self.project_spec.project_description = Some(project_description);

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn tests_creating_managing_agent() {
        let mut managing_agent = ManagerAgent::new().unwrap();
        managing_agent.articulate_project_description("Create a simple todo app".to_string()).await;
        dbg!(managing_agent);
    }
}