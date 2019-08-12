use crate::analyze::{Analyze, DetectionResult};
use crate::generation::parsed_to_render::*;
use crate::instrument::{Basis, Oscillator};
use crate::ring_buffer::RingBuffer;
use crate::settings::{default_settings, Settings};
use crate::write::write_output_buffer;
use crate::control::MicState;
use num_rational::Rational64;
use portaudio as pa;
use socool_ast::PointOp;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::sync::{Arc, Mutex};

struct RealTimeState {
    count: Rational64,
    inc: Rational64,
    current_op: PointOp,
}

impl RealTimeState {
    fn inc(&mut self) {
        self.count += self.inc * Rational64::new(1, 1)
    }
}

fn process_result(result: &mut DetectionResult) -> Basis {
    result.normalize();

    Basis {
        f: 2.0 * result.frequency as f64,
        l: 1.0,
        g: result.gain as f64,
        p: 0.0,
        a: 44100.0,
        d: 44100.0,
    }
}

fn process_pointop(result: &mut DetectionResult, home: f32) -> (f32, f32) {
    result.normalize();

    (result.frequency / home, result.gain / home)
}

fn file_start() -> &'static [u8] {
    "{ f: 340, l: 0.05, g: 1, p: 0}\n
    \n
    main = {\n
        Seq [\n"
        .as_bytes()
}

fn prepare_file() -> File {
    let out_path = "test.socool";

    let mut output_file = File::create(out_path).unwrap();

    let input_file = file_start();
    let buffered_input = BufReader::new(input_file);

    for (i, line) in buffered_input.lines().enumerate() {
        if i < 4 {
            write!(output_file, "{}\n", line.unwrap()).unwrap();
        }
    }

    output_file
}

pub fn duplex_setup(
    state: Arc<Mutex<MicState>>,
    parsed_composition: Vec<Vec<PointOp>>,
) -> Result<pa::Stream<pa::NonBlocking, pa::Duplex<f32, f32>>, pa::Error> {
    let pa = pa::PortAudio::new()?;
    let settings = default_settings();
    let duplex_stream_settings = get_duplex_settings(&pa, &settings)?;

    let mut input_buffer: RingBuffer<f32> = RingBuffer::<f32>::new(settings.yin_buffer_size);

    let mut count = 0;
    let home = 120.0;
    let mut file = prepare_file();

    let mut nf_iterators = vec![];

    for seq in parsed_composition {
        let mut iterator = seq.clone().into_iter().cycle();
        let state = RealTimeState {
            count: Rational64::new(0, 1),
            inc: Rational64::new(settings.buffer_size as i64, settings.sample_rate as i64),
            current_op: iterator.next().unwrap(),
        };

        nf_iterators.push((Oscillator::init(&default_settings()), iterator, state))
    }

    let mut i = 0;
    let duplex_stream = pa.open_non_blocking_stream(
        duplex_stream_settings,
        move |pa::DuplexStreamCallbackArgs {
                  in_buffer,
                  mut out_buffer,
                  ..
              }| {
            if count < 20 {
                count += 1;
                if count == 20 {
                    println!("{}", "* * * * * ready * * * * *");
                }
                pa::Continue
            } else {
                input_buffer.push_vec(in_buffer.to_vec());
                let mut result: DetectionResult = input_buffer
                    .to_vec()
                    .analyze(settings.sample_rate as f32, settings.probability_threshold);
                i += 1;

                let origin = process_result(&mut result);

                let (tm, gain) = process_pointop(&mut result, home);

                {
                    let shared = state.lock().unwrap();
                    println!(
                        "freq {}, gain {}, state {:?}",
                        result.frequency, result.gain, shared
                    );
                }

                let op = format!("Tm {:?} | Gain {:?},\n", tm * 2.0, gain * 1000.0);
                write!(file, "\t{}", op).expect("Couldn't write to file");

                let mut result = vec![];

                for (ref mut oscillator, ref mut iterator, ref mut state) in nf_iterators.iter_mut()
                {
                    if state.count >= state.current_op.l {
                        state.count = Rational64::new(0, 1);
                        state.current_op = iterator.next().unwrap()
                    }

                    let mut current_point_op = state.current_op.clone();

                    current_point_op.l =
                        Rational64::new(settings.buffer_size as i64, settings.sample_rate as i64);
                    let stereo_waveform = render_mic(&current_point_op, origin, oscillator);
                    result.push(stereo_waveform);
                    state.inc();
                }

                let stereo_waveform = sum_all_waveforms(result);
                write_output_buffer(&mut out_buffer, stereo_waveform);

                pa::Continue
            }
        },
    )?;

    Ok(duplex_stream)
}

fn get_duplex_settings(
    ref pa: &pa::PortAudio,
    ref settings: &Settings,
) -> Result<pa::stream::DuplexSettings<f32, f32>, pa::Error> {
    let def_input = pa.default_input_device()?;
    let input_info = pa.device_info(def_input)?;
    //    println!("Default input device info: {:#?}", &input_info);

    let latency = input_info.default_low_input_latency;
    let input_params = pa::StreamParameters::<f32>::new(
        def_input,
        settings.channels,
        settings.interleaved,
        latency,
    );

    let def_output = pa.default_output_device()?;
    let output_info = pa.device_info(def_output)?;
    //    println!("Default output device info: {:#?}", &output_info);

    let latency = output_info.default_low_output_latency;
    let output_params =
        pa::StreamParameters::new(def_output, settings.channels, settings.interleaved, latency);

    let duplex_settings = pa::DuplexStreamSettings::new(
        input_params,
        output_params,
        settings.sample_rate as f64,
        settings.buffer_size as u32,
    );

    Ok(duplex_settings)
}
