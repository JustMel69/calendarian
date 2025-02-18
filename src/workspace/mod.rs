use std::path::PathBuf;

use crate::project::Project;

#[derive(Debug)]
pub struct WorkspaceState {
    project: Project
}

impl WorkspaceState {
    pub fn new(project_path: PathBuf) -> Self {
        Self {
            project: Project::new(project_path),
        }
    }
    
    pub fn update(&mut self, ctx: &egui::Context, interactable: bool) {
        egui::CentralPanel::default().show(ctx, |ui| ui.add_enabled_ui(interactable, |ui| {
            ui.heading("yes project selected!!!!!!!");
        }));
    }

    pub fn project(&self) -> &Project {
        &self.project
    }
}
