use crate::agents::agent_architect::architect_agent::ArchitectAgent;
use crate::agents::base::agent_base::{AgentAttributes, AgentState};
use crate::agents::base::agent_traits::{ProjectSpec, SpecialFunctions};
use crate::utils::llm_apis::request_task_llm;
use crate::ai_functions::ai_functions::convert_user_input_to_goal;
use crate::utils::command_line::PrintMessage;


#[derive(Debug)]
pub struct ManagerAgent {
    attributes: AgentAttributes,
    project_spec: ProjectSpec,
    agents: Vec<Box<dyn SpecialFunctions>>, // list of agents manager is managing
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

        let agents: Vec<Box<dyn SpecialFunctions>> = vec![];

        Ok(Self {
            attributes,
            project_spec,
            agents
        })
    }

    // Step 1. Generate a project description for Solutions Architect agent to interpret
    pub async fn articulate_project_description(&mut self, user_req: String, agent_operation: &str) {

        let project_description: String = request_task_llm(
            convert_user_input_to_goal, 
            user_req,
            &self.attributes.position,
            get_function_string!(convert_user_input_to_goal)
        ).await;
        let agent_pos: String = self.attributes.position.clone();

        PrintMessage::Info.print_agent_msg(&agent_pos, agent_operation);

        self.project_spec.project_description = Some(project_description);

    }

    fn add_agent(&mut self, agent: Box<dyn SpecialFunctions>) {
        self.agents.push(agent);
    }
    
    pub async fn execute_workflow(&mut self) {

        // Adding all the agents:
        // 1. Solutions Architect
        // 2. Backend Developer
        // 3. Frontend Developer
        self.add_agent(Box::new(
            ArchitectAgent::new(
                "Gathers information and design solutions for website development".to_owned(),
                "Solutions Architect".to_owned()
            ))
        );

        for agent in &mut self.agents {
            // Execute agents workflow
            let _agent_res: Result<(), Box<dyn std::error::Error>> = agent.execute(&mut self.project_spec).await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn tests_creating_managing_agent() {
        let mut managing_agent = ManagerAgent::new().unwrap();
        managing_agent.articulate_project_description("Create a simple todo app".to_string(), get_function_string!(convert_user_input_to_goal)).await;
        dbg!(managing_agent);
    }
}