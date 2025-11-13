use crate::{Error, PluginInstance, Result, SharedWorld};
use livi_external_ui::external_ui::{ExternalUI, ExternalUILibrary};
use std::sync::{Arc, Mutex};

pub struct PluginInstanceFactory {
    pub world: SharedWorld,
    pub plugin: livi::Plugin,
    pub ui: Option<Arc<Mutex<ExternalUILibrary>>>,
}

unsafe impl Send for PluginInstanceFactory {}

impl PluginInstanceFactory {
    pub fn raw<'a>(&'a self) -> &'a livi::Plugin {
        &self.plugin
    }

    pub fn new(world: &SharedWorld, plugin: &livi::Plugin, ui: Option<&ExternalUI>) -> Result<Self> {
        let lib: Option<Arc<Mutex<ExternalUILibrary>>> = match ui.map(|ui| {
            ui.load().map_err(|e| {
                Error::UIError(livi_external_ui::ui::LiviUIError::LiviExternalUIError(e))
            })
        }) {
            None => None,
            Some(Err(e)) => {
                return Err(e);
            }
            Some(Ok(lib)) => Some(Arc::new(Mutex::new(lib))),
        };
        Ok(Self {
            plugin: plugin.clone(),
            ui: lib,
            world: world.clone(),
        })
    }

    pub fn instantiate(&self, sample_rate: f64) -> Result<PluginInstance> {
        if let Some(ui) = self.ui.as_ref() {
            match ui.lock() {
                Err(e) => {
                    return Err(Error::InternalError(format!("Could not lock mutex: {e}")));
                }
                Ok(lib) => {
                    return Ok(PluginInstance::new(
                        &self.world,
                        &self.plugin,
                        Some(&lib),
                        sample_rate,
                    )?);
                }
            }
        };
        Ok(PluginInstance::new(
            &self.world,
            &self.plugin,
            None,
            sample_rate,
        )?)
    }
}
