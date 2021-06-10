mod hw;
mod mixer;

pub use mixer::Mixer;

#[non_exhaustive]
pub struct MixerController {}

impl MixerController {
    pub(crate) const fn new() -> Self {
        MixerController {}
    }

    pub fn mixer(&mut self) -> Mixer {
        Mixer::new()
    }
}

pub struct SoundChannel {
    data: &'static [u8],
    pos: usize,
    should_loop: bool,
}

impl SoundChannel {
    pub fn new(data: &'static [u8]) -> Self {
        SoundChannel {
            data,
            pos: 0,
            should_loop: false,
        }
    }

    pub fn should_loop(mut self) -> Self {
        self.should_loop = true;
        self
    }
}
