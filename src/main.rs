use portaudio::{self as PA, PortAudio, stream::{OutputCallbackArgs, CallbackResult as PAResult}};

use synth::osc::{Osc, OscSettings, TriangleOsc};

const CHANNELS: i32 = 1;
const SAMPLE_RATE: f32 = 44_100.0;

fn main() -> Result<(), PA::Error> {
    let pa = PortAudio::new()?;
    let defaults = pa.default_output_stream_settings::<f32>(CHANNELS, SAMPLE_RATE as f64, 0)?;
    let mut osc: Box<dyn Osc> = Box::new(TriangleOsc::new(OscSettings::default()));
    let mut stream = pa.open_non_blocking_stream(defaults, move |OutputCallbackArgs { buffer, frames, .. }| {
        for (i, sample) in osc.by_ref().enumerate().take(frames) {
            buffer[i] = sample;
        }
       PAResult::Continue
    })?;
    stream.start()?;
    loop {}
}
