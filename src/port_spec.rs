pub struct PortSpec {
    pub buffer_size : usize
}

pub struct PortsSpec {
    pub audio_inputs : Vec<PortSpec>,
    pub audio_outputs : Vec<PortSpec>,
    pub midi_inputs : Vec<PortSpec>,
    pub midi_outputs : Vec<PortSpec>,
}