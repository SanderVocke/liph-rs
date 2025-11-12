use crate::{PortBufferHandles, PortsSpec, Result};

pub trait PluginInstanceAPI {
    // UI-related
    fn has_ui() -> Result<bool>;
    fn is_visible() -> Result<bool>;
    fn set_visible(visible: bool) -> Result<()>;

    // Ports-related
    fn ports_spec() -> Result<PortsSpec>;

    // Processing
    fn process(
        ports : &mut PortBufferHandles
    ) -> Result<()>;
}