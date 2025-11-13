use livi_external_ui::external_ui::{
    ExternalUIInstance, ExternalUIInstanceRunner, ExternalUILibrary,
};

use crate::{Error, Result, SharedWorld};

pub struct PluginInstance {
    plugin: livi::Instance,
    ui: Option<(ExternalUIInstance, Box<ExternalUIInstanceRunner>)>,
}

impl PluginInstance {
    pub fn new<'a>(
        world: &SharedWorld,
        plugin: &livi::Plugin,
        ui: Option<&'a ExternalUILibrary>,
        sample_rate: f64,
    ) -> Result<Self> {
        let buffer_size = 8192;
        let features = world
            .world
            .lock()
            .map_err(|e| Error::InternalError(format!("Could not lock mutex: {e}")))?
            .raw()
            .build_features(livi::FeaturesBuilder {
                min_block_length: 1,
                max_block_length: buffer_size,
            });
        let plugin_instance = unsafe {
            plugin
                .instantiate(features, sample_rate)
                .map_err(|e| Error::InstantiatePluginError(e))?
        };
        let ui = match ui.map(|ui| ui.instantiate(&plugin_instance)) {
            Some(Ok(ui)) => Some(ui),
            Some(Err(e)) => {
                return Err(Error::ExternalUIError(e));
            }
            None => None,
        };
        Ok(Self {
            plugin: plugin_instance,
            ui: ui,
        })
    }
}

impl crate::PluginInstanceAPI for PluginInstance {
    fn has_ui() -> Result<bool> {
        todo!()
    }

    fn is_visible() -> Result<bool> {
        todo!()
    }

    fn set_visible(visible: bool) -> Result<()> {
        todo!()
    }

    fn ports_spec() -> Result<crate::PortsSpec> {
        todo!()
    }

    fn process(ports: &mut crate::PortBufferHandles) -> Result<()> {
        todo!()
    }
}
