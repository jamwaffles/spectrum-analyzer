/*
MIT License

Copyright (c) 2023 Philipp Schuster

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/
use audio_visualizer::dynamic::live_input::{list_input_devs, AudioDevAndCfg};
use audio_visualizer::dynamic::window_top_btm::{open_window_connect_audio, TransformFn};
use cpal::traits::{DeviceTrait, HostTrait};
use cpal::{BufferSize, SampleRate, StreamConfig, SupportedBufferSize};
use rand::distributions::uniform::SampleBorrow;
use spectrum_analyzer::scaling::divide_by_N;
use spectrum_analyzer::windows::hann_window;
use spectrum_analyzer::{samples_fft_to_spectrum, FrequencyLimit, FrequencyValue};
use std::cell::RefCell;
use std::cmp::max;

/// Example that creates a live visualization of the frequency spectrum of realtime audio data
/// **Execute this with `--release`, otherwise it is very laggy!**.
fn main() {
    // // Contains the data for the spectrum to be visualized. It contains ordered pairs of
    // // `(frequency, frequency_value)`. During each iteration, the frequency value gets
    // // combined with `max(old_value * smoothing_factor, new_value)`.
    // let visualize_spectrum: RefCell<Vec<(f64, f64)>> = RefCell::new(vec![(0.0, 0.0); 1024]);

    // // Closure that captures `visualize_spectrum`.
    // let to_spectrum_fn = move |audio: &[f32], sampling_rate| {
    //     let skip_elements = audio.len() - 2048;
    //     // spectrum analysis only of the latest 46ms
    //     let relevant_samples = &audio[skip_elements..skip_elements + 2048];

    //     // do FFT
    //     let hann_window = hann_window(relevant_samples);
    //     let latest_spectrum = samples_fft_to_spectrum(
    //         &hann_window,
    //         sampling_rate as u32,
    //         FrequencyLimit::All,
    //         Some(&divide_by_N),
    //     )
    //     .unwrap();

    //     // now smoothen the spectrum; old values are decreased a bit and replaced,
    //     // if the new value is higher
    //     latest_spectrum
    //         .data()
    //         .iter()
    //         .zip(visualize_spectrum.borrow_mut().iter_mut())
    //         .for_each(|((fr_new, fr_val_new), (fr_old, fr_val_old))| {
    //             // actually only required in very first iteration
    //             *fr_old = fr_new.val() as f64;
    //             let old_val = *fr_val_old * 0.84;
    //             let max = max(
    //                 *fr_val_new * 5000.0_f32.into(),
    //                 FrequencyValue::from(old_val as f32),
    //             );
    //             *fr_val_old = max.val() as f64;
    //         });

    //     visualize_spectrum.borrow().clone()
    // };

    let host = cpal::default_host();

    // let device = host
    //     .default_input_device()
    //     .expect("No default input device");

    // for dev in host.input_devices().expect("No input devices") {
    //     dbg!(dev.name());
    // }

    let device = host
        .input_devices()
        .unwrap()
        .find(|d| d.name().unwrap() == "hw:CARD=Mini,DEV=0")
        .expect("No mic");

    // SupportedStreamConfig { channels: 2, sample_rate: SampleRate(44100), buffer_size: Range { min: 5, max: 4294967294 }, sample_format: F32 }
    for config in device.supported_input_configs().unwrap() {
        // dbg!(config.max_sample_rate());
        // dbg!(config.channels());
        // dbg!(config.buffer_size());

        let SupportedBufferSize::Range { min, max } = config.buffer_size() else {
            panic!("Idk");
        };

        let c = StreamConfig {
            channels: config.channels(),
            sample_rate: config.max_sample_rate(),
            buffer_size: BufferSize::Fixed(*max),
        };

        dbg!(device
            .build_input_stream(
                &c,
                |data: &[f32], _info| {
                    println!("Cb");
                },
                |err| {
                    eprintln!("got stream error: {:#?}", err);
                },
                None,
            )
            .is_ok());
    }

    let config = device.default_input_config().expect("No default config");

    let c = config.config();

    let c = StreamConfig {
        channels: 1,
        sample_rate: SampleRate(48_000),
        buffer_size: cpal::BufferSize::Fixed(64),
    };

    // open_window_connect_audio(
    //     "Live Spectrum View",
    //     None,
    //     None,
    //     // 0.0..22050.0_f64.log(100.0),
    //     Some(0.0..22050.0),
    //     Some(0.0..500.0),
    //     "x-axis",
    //     "y-axis",
    //     AudioDevAndCfg::new(Some(device), Some(c)),
    //     TransformFn::Complex(&to_spectrum_fn),
    // );
}
