use crate::utils::general::save_code_to_file;

pub struct BackendAgent {
    attributes: AgentAttributes,
}

impl BackendAgent {
    pub fn new(objective: String, position: String) -> Self {
        let attributes: AgentAttributes = AgentAttributes::new(objective, position);
        Self { attributes }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_backend_agent() {}
}
