use crate::Result;

pub struct PluginInstance {
    plugin: livi::Instance,
    ui: Option<livi_external_ui::external_ui::ExternalUIInstance>,
}

impl PluginInstance {
    pub fn new(plugin : &crate::world::Plugin, uri: &str) -> Result<Self> {
        todo!();
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
