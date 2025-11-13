use livi::event::LV2AtomSequence;

use crate::{PortConnections, PortsSpec, Result};

pub trait PluginInstanceAPI {
    // UI-related
    fn has_ui() -> Result<bool>;
    fn is_visible() -> Result<bool>;
    fn set_visible(visible: bool) -> Result<()>;

    // Ports-related
    fn ports_spec() -> Result<PortsSpec>;

    // Processing
    unsafe fn run<
        'a,
        AudioInputs,
        AudioOutputs,
        MidiInputs,
        MidiOutputs,
        CVInputs,
        CVOutputs,
    >(
        &mut self,
        samples: usize,
        ports: PortConnections<
            'a,
            AudioInputs,
            AudioOutputs,
            MidiInputs,
            MidiOutputs,
            CVInputs,
            CVOutputs,
        >,
    ) -> Result<()>
    where
        AudioInputs: ExactSizeIterator + Iterator<Item = &'a [f32]>,
        AudioOutputs: ExactSizeIterator + Iterator<Item = &'a mut [f32]>,
        MidiInputs: ExactSizeIterator + Iterator<Item = &'a [u8]>,
        MidiOutputs: ExactSizeIterator + Iterator<Item = &'a mut [u8]>,
        CVInputs: ExactSizeIterator + Iterator<Item = &'a [f32]>,
        CVOutputs: ExactSizeIterator + Iterator<Item = &'a mut [f32]>;
}