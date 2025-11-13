use crate::Error;
use crate::Plugin;
use crate::Result;
use livi_external_ui::external_ui::{ExternalUI, ExternalUILibrary};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;

pub type SharedPlugin = Arc<Mutex<Plugin>>;

pub struct World {
    world: livi::World,
    plugins: HashMap<String, SharedPlugin>,
}

impl World {
    pub fn new() -> Self {
        Self {
            world: livi::World::new(),
            plugins: HashMap::new(),
        }
    }

    pub fn get_plugin(&self, plugin_uri: &str) -> Result<SharedPlugin> {
        if self.plugins.contains_key(plugin_uri) {
            return Ok(self.plugins.get(plugin_uri).ok_or(Error::InternalError(
                "Could not find registered plugin".to_string(),
            ))?.clone());
        }
        return Err(Error::PluginNotFoundError("Plugin not loaded yet".to_string()));
    }

    pub fn ensure_plugin<'a>(&'a mut self, plugin_uri: &str) -> Result<()> {
        if self.plugins.contains_key(plugin_uri) {
            return Ok(());
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
            Arc::new(Mutex::new(Plugin {
                plugin: livi_plugin,
                ui: the_ui,
            })),
        );

        Ok(())
    }
}

pub struct SharedWorld {
    pub world: Arc<Mutex<World>>,
}

impl SharedWorld {
    pub fn new() -> Self {
        Self {
            world: Arc::new(Mutex::new(World::new())),
        }
    }

    pub fn plugin_by_uri(&self, uri: &str) -> Result<SharedPlugin> {
        match self.world.lock() {
            Ok(mut world) => {
                world.ensure_plugin(uri)?;
                world.get_plugin(uri)
            },
            Err(e) => Err(Error::InternalError(e.to_string())),
        }
    }
}