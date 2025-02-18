use std::path::PathBuf;

#[derive(Debug, Default)]
pub struct Upstream {
    cmd_queue: Vec<UpstreamCmd>,
}

impl Upstream {
    pub fn push_cmd(&mut self, cmd: UpstreamCmd) {
        self.cmd_queue.push(cmd);
    }

    pub fn pop_cmd(&mut self) -> Option<UpstreamCmd> {
        self.cmd_queue.pop()
    }
}


#[derive(Debug)]
pub enum UpstreamCmd {
    TryQuit,
    ForceQuit,
    OpenProject(PathBuf),
    CloseProject,
}
