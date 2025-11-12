use crate::Error;
use crate::PluginInstance;
use crate::Result;
use livi_external_ui::external_ui::{ExternalUI, ExternalUILibrary};
use std::collections::HashMap;

pub struct Plugin {
    plugin: livi::Plugin,
    ui: Option<(ExternalUI, ExternalUILibrary)>,
}

pub struct World {
    world: livi::World,
    plugins: HashMap<String, Plugin>,
}

impl World {
    pub fn new() -> Self {
        Self {
            world: livi::World::new(),
            plugins: HashMap::new(),
        }
    }

    pub fn get_plugin<'a>(&'a mut self, plugin_uri: &str) -> Result<&'a Plugin> {
        if self.plugins.contains_key(plugin_uri) {
            return Ok(self.plugins.get(plugin_uri).ok_or(Error::InternalError(
                "Could not find registered plugin".to_string(),
            ))?);
        }

        let livi_plugin = self
            .world
            .plugin_by_uri(plugin_uri)
            .ok_or(Error::PluginNotFoundError(plugin_uri.to_string()))?;

        let mut the_ui: Option<(ExternalUI, ExternalUILibrary)> = None;

        {
            let mut uis = livi_external_ui::ui::plugin_uis(&self.world, &livi_plugin)
                .map_err(|e| Error::UIError(e))?;

            if let Some(ui) = uis.next() {
                if let Some(_) = uis.next() {
                    eprintln!(
                        "Found more than one UI for plugin {}. Only instantiating the first.",
                        plugin_uri
                    );
                }

                if let livi_external_ui::ui::UI::External(ui) = ui {
                    let lib = ui.load().map_err(|e| {
                        Error::UIError(livi_external_ui::ui::LiviUIError::LiviExternalUIError(e))
                    })?;
                    the_ui = Some((ui, lib));
                }
            }
        }

        self.plugins.insert(
            plugin_uri.to_string(),
            Plugin {
                plugin: livi_plugin,
                ui: None,
            },
        );

        return Ok(self.plugins.get(plugin_uri).ok_or(Error::InternalError(
            "Could not find registered plugin".to_string(),
        ))?);
    }
}
