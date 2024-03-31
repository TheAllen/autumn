use async_trait::async_trait;
use crate::{agents::base::{agent_base::AgentAttributes, agent_traits::{ProjectSpec, SpecialFunctions}}, utils::general::save_code_to_file};

#[derive(Debug)]
pub struct BackendAgent {
    attributes: AgentAttributes,
}

impl BackendAgent {
    pub fn new(objective: String, position: String) -> Self {
        let attributes: AgentAttributes = AgentAttributes::new(objective, position);
        Self { attributes }
    }
}

#[async_trait]
impl SpecialFunctions for BackendAgent {
    fn get_attributes_from_agent(&self) -> &AgentAttributes {
        &self.attributes
    }

    async fn execute(&mut self, proj_spec: &mut ProjectSpec) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_backend_agent() {
        let backend_agent = BackendAgent::new(
            "Build server side application".to_owned(),
            "Backend Agent".to_owned()
        );

        dbg!(backend_agent);
    }
}
