use livi_external_ui::external_ui::{ExternalUI, ExternalUILibrary};

use crate::{PluginInstanceFactory, Result, SharedWorld};

pub struct Plugin {
    pub plugin: livi::Plugin,
    pub ui: Option<ExternalUI>,
}

impl Plugin {
    pub fn raw<'a>(&'a self) -> &'a livi::Plugin {
        &self.plugin
    }

    pub fn create_instance_factory(&self, world: &SharedWorld) -> Result<PluginInstanceFactory> {
        PluginInstanceFactory::new(world, &self.plugin, self.ui.as_ref())
    }
}