use crate::{agents::base::{agent_base::AgentAttributes, agent_traits::{ProjectScope, ProjectSpec}}, utils::llm_apis::{request_task_llm, request_task_llm_deserialized}};
use crate::ai_functions::ai_functions::print_project_scope;

#[derive(Debug)]
pub struct ArchitectAgent {
    attributes: AgentAttributes
}

impl ArchitectAgent {
    pub fn new(objective: String, position: String) -> Self {
        let attributes: AgentAttributes = AgentAttributes::new(objective, position);
        Self {
            attributes
        }
    }

    // Generate project scope and update project specification
    async fn generate_project_scope(&mut self, project_spec: &mut ProjectSpec) {
        let project_description = project_spec.project_description.as_ref().expect("Project description not defined yet!");

        let project_scope = request_task_llm_deserialized::<ProjectScope>(
            print_project_scope,
            project_description.clone(),
            &self.attributes.position,
            get_function_string!(print_project_scope)
        ).await;

        project_spec.project_scope = Some(project_scope);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_create_architect_agent() {
        let architect: ArchitectAgent = ArchitectAgent::new(
            "Gathers information and design solutions for website development".to_string(),
            "Solutions Architect".to_string()
        );
        dbg!(architect);
    }

    #[tokio::test]
    async fn tests_creation_project_scope() {
        let mut project_spec: ProjectSpec = ProjectSpec::new(
            Some("build a website that handles users logging in and logging out and accepts payments".to_string()),
            None,
            None,
            None,
            None,
            None
        );

        let mut architect: ArchitectAgent = ArchitectAgent::new(
            "Gathers information and design solutions for website development".to_string(),
            "Solutions Architect".to_string()
        );

        architect.generate_project_scope(&mut project_spec).await;

        dbg!(project_spec);
    }
}
