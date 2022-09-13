use sfml::{audio::{SoundBuffer, Sound}, SfBox};

pub struct SoundPlayers<'a> {
    pub correct_ans: Sound<'a>,
    pub incorrect_ans: Sound<'a>,
}

impl <'a>SoundPlayers<'a> {
    pub fn new(sound_buffers: &'a SoundBuffers) -> Self {
        Self {
            correct_ans: Sound::with_buffer(&sound_buffers.correct_ans),
            incorrect_ans: Sound::with_buffer(&sound_buffers.incorrect_ans),
        }
    }
}

pub struct SoundBuffers {
    correct_ans: SfBox<SoundBuffer>,
    incorrect_ans: SfBox<SoundBuffer>,
}

impl SoundBuffers {
    pub fn new() -> Self{
        let message = "Error while loading sounds";
        let correct_ans = SoundBuffer::from_file("res/sounds/correct.wav")
            .expect(message);
        let incorrect_ans = SoundBuffer::from_file("res/sounds/incorrect.wav")
            .expect(message);
        
        Self {
            correct_ans,
            incorrect_ans,
        }
    }
}