#[derive(Debug, Default)]
pub struct ProjectSelectionState {
    
}

impl ProjectSelectionState {
    pub fn update(&mut self, ctx: &egui::Context, interactable: bool) {
        egui::CentralPanel::default().show(ctx, |ui| ui.add_enabled_ui(interactable, |ui| {
            ui.heading("No project selected.");
        }));
    }
}
