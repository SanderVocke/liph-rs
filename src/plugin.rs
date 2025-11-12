use livi_external_ui::external_ui::{ExternalUI, ExternalUILibrary};

pub struct Plugin {
    pub plugin: livi::Plugin,
    pub ui: Option<(ExternalUI, ExternalUILibrary)>,
}

impl Plugin {
    pub fn name(&self) -> String {
        self.plugin.name()
    }
}