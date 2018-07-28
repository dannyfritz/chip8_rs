#[derive(Debug)]
pub enum AudioEvent {
    Play,
    Stop,
}

pub struct AudioSink {
    pub event: Option<AudioEvent>,
}

impl Default for AudioSink {
    fn default() -> AudioSink {
        AudioSink {
            event: Some(AudioEvent::Stop),
        }
    }
}

impl AudioSink {
    pub fn new() -> AudioSink {
        AudioSink::default()
    }
}
