use crate::{PortSpec, PortsSpec};

pub struct PortBufferHandles<'a> {
    audio_outputs: &'a[&'a mut [f32]],
    audio_inputs: &'a[&'a [f32]],
    midi_outputs: &'a[&'a [u8]],
    midi_inputs: &'a[&'a [u8]],
}

impl PortBufferHandles<'_> {
    pub fn check_against(&self, spec: &PortsSpec) -> bool {
        if self.audio_inputs.len() != spec.audio_inputs.len() {
            return false;
        }
        if self.audio_outputs.len() != spec.audio_outputs.len() {
            return false;
        }
        if self.midi_inputs.len() != spec.midi_inputs.len() {
            return false;
        }
        if self.midi_outputs.len() != spec.midi_outputs.len() {
            return false;
        }

        let check_f32 = |spec : &PortSpec, buffer : &[f32]| -> bool {
            if (buffer.len()*std::mem::size_of::<f32>()) != spec.buffer_size {
                return false;
            }
            return true;
        };

        let check_midi = |spec : &PortSpec, buffer : &[u8]| -> bool {
            if buffer.len() != spec.buffer_size {
                return false;
            }
            return true;
        };

        for (idx, spec) in spec.audio_inputs.iter().enumerate() {
            if !check_f32(spec, self.audio_inputs[idx]) {
                return false;
            }
        }
        for (idx, spec) in spec.audio_outputs.iter().enumerate() {
            if !check_f32(spec, self.audio_outputs[idx]) {
                return false;
            }
        }
        for (idx, spec) in spec.midi_inputs.iter().enumerate() {
            if !check_midi(spec, self.midi_inputs[idx]) {
                return false;
            }
        }
        for (idx, spec) in spec.midi_outputs.iter().enumerate() {
            if !check_midi(spec, self.midi_outputs[idx]) {
                return false;
            }
        }

        return true;
    }
}