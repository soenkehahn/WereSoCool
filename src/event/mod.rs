use instrument::{oscillator::Oscillator, stereo_waveform::StereoWaveform};
use pbr::ProgressBar;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Event {
    pub sounds: Vec<Sound>,
    pub length: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Sound {
    pub frequency: f64,
    pub gain: f64,
    pub pan: f64,
}

impl Sound {
    pub fn init() -> Sound {
        Sound {
            frequency: 0.0,
            gain: 0.0,
            pan: 0.0,
        }
    }
}

impl Event {
    pub fn init(frequency: f64, gain: f64, pan: f64, length: f64) -> Event {
        Event {
            sounds: {
                vec![Sound {
                    frequency,
                    gain,
                    pan,
                }]
            },
            length,
        }
    }
}

pub trait Render<T> {
    fn render(&mut self, oscillator: &mut Oscillator) -> StereoWaveform;
}

impl Render<Event> for Event {
    fn render(&mut self, oscillator: &mut Oscillator) -> StereoWaveform {
//        println!("{:?}", self);
        oscillator.update(self.sounds.clone());
        let n_samples_to_generate = self.length * 44_100.0;
        oscillator.generate(n_samples_to_generate)
    }
}

impl Render<Vec<Event>> for Vec<Event> {
    fn render(&mut self, oscillator: &mut Oscillator) -> StereoWaveform {
        let mut result: StereoWaveform = StereoWaveform::new(0);
        let mut events = self.clone();
        events.push(Event::init(0.0, 0.0, 0.0, 1.0));
                let n_events = events.len();
                let mut pb = ProgressBar::new(n_events as u64);
                pb.format("╢w♬░╟");
                pb.message("Rendering:  ");
                let sub_div = 1 + n_events / 1000;
                let mut i = 1;
            for mut event in events {
                        i += 1;
                        if i % sub_div == 0 {
                            pb.add(sub_div as u64);
                        }
            let stereo_waveform = event.render(oscillator);
            result.append(stereo_waveform);
        }
                let finish_string = "".to_string();
                pb.finish_print(&finish_string);

        result
    }
}

#[cfg(test)]
mod test;
