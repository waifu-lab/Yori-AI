use crate::types::{ProjectManager, Project};

pub struct ProjectManagerImpl {

}

impl ProjectManager for ProjectManagerImpl {
    fn get_recent_projects(&self) -> Vec<Box<dyn Project>> {
        vec![]
    }

    fn add_recent_project(&self, project: Box<dyn Project>) -> anyhow::Result<()> {
        Ok(())
    }
}