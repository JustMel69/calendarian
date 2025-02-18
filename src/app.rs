use egui::ViewportCommand;

use crate::{modals::Modal, project_selection::ProjectSelectionState, status_bar::StatusBar, upstream::{Upstream, UpstreamCmd}};

#[derive(Debug)]
pub enum AppState {
    ProjectSelection(ProjectSelectionState),
}

impl AppState {
    fn update(&mut self, ctx: &egui::Context, interactable: bool) {
        match self {
            AppState::ProjectSelection(x) => x.update(ctx, interactable),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::ProjectSelection(Default::default())
    }
}



#[derive(Default)]
pub struct Application {
    state: AppState,
    status_bar: StatusBar,
    modals: Vec<Box<dyn Modal>>,
    upstream: Upstream,
}

impl Application {
    fn make_ctx(&mut self) -> AppContext<'_> {
        AppContext { upstream: &mut self.upstream, state: &self.state }
    }

    fn handle_upstream(&mut self, ctx: &egui::Context) {
        while let Some(cmd) = self.upstream.pop_cmd() {
            match cmd {
                UpstreamCmd::TryQuit | UpstreamCmd::ForceQuit => ctx.send_viewport_cmd(ViewportCommand::Close),
            }
        }
    }
}

impl eframe::App for Application {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut interactable = true;

            let mut swap = Vec::new();
            std::mem::swap(&mut swap, &mut self.modals);
            for modal in swap.iter_mut().rev() {
                modal.update(ui, self.make_ctx());
                interactable = false;
            }
            std::mem::swap(&mut swap, &mut self.modals);
            
            let mut app_ctx = AppContext { upstream: &mut self.upstream, state: &self.state };
            self.status_bar.update(ctx, &mut app_ctx, interactable);
            self.state.update(ctx, interactable);
        });

        self.handle_upstream(ctx);
    }
}

pub struct AppContext<'a> {
    pub upstream: &'a mut Upstream,
    pub state: &'a AppState,
}
