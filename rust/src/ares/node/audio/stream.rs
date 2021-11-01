use malachite_base::num::float::NiceFloat;
use nall::dsp::iir::biquad::{self, Biquad};
use nall::dsp::iir::one_pole::{self, OnePole};
use nall::dsp::resampler::cubic::Cubic;
use std::cmp::min;

#[derive(Clone, Debug)]
pub enum FilterMode {
    OnePole,
    Biquad,
}

impl Default for FilterMode {
    fn default() -> FilterMode {
        FilterMode::OnePole
    }
}

#[derive(Clone, Debug)]
pub enum FilterType {
    None,
    LowPass,
    HighPass,
    LowShelf,
    HighShelf,
}

impl Default for FilterType {
    fn default() -> FilterType {
        FilterType::None
    }
}

#[derive(Clone, Debug)]
pub enum FilterOrder {
    None,
    First,
    Second,
}

impl Default for FilterOrder {
    fn default() -> FilterOrder {
        FilterOrder::None
    }
}

#[derive(Clone, Debug, Default)]
pub struct Filter {
    mode: FilterMode,
    f_type: FilterType,
    order: FilterOrder,
    one_pole: OnePole,
    biquad: Biquad,
}

#[derive(Clone, Debug, Default)]
pub struct Channel {
    filters: Vec<Filter>,
    pub nyquist: Vec<Biquad>,
    pub resampler: Cubic,
}

#[derive(Clone, Debug)]
pub struct Stream {
    pub name: &'static str,
    pub channels: Vec<Channel>,
    pub frequency: f64,
    pub resampler_frequency: f64,
    pub muted: bool,
}

impl Default for Stream {
    fn default() -> Stream {
        Stream {
            name: "",
            channels: Vec::new(),
            frequency: 48000.0,
            resampler_frequency: 48000.0,
            muted: false,
        }
    }
}

impl Stream {
    pub fn new(name: &'static str) -> Stream {
        Stream {
            name,
            channels: Vec::new(),
            frequency: 48000.0,
            resampler_frequency: 48000.0,
            muted: false,
        }
    }

    pub fn set_channels(&mut self, channels: u32) {
        self.channels.clear();
        for _ in 0..channels {
            self.channels.push(Channel::default());
        }
    }

    pub fn set_frequency(&mut self, frequency: f64) {
        self.frequency = frequency;
        self.set_resampler_frequency(self.resampler_frequency);
    }

    pub fn set_resampler_frequency(&mut self, resampler_frequency: f64) {
        self.resampler_frequency = resampler_frequency;
        for channel in &mut self.channels {
            channel.nyquist.clear();
            channel
                .resampler
                .reset(self.frequency, self.resampler_frequency, 0);
        }

        if self.frequency >= self.resampler_frequency * 2.0 {
            // add a low-pass filter to prevent aliasing during resampling
            let cutoff_frequency = min(
                NiceFloat(25000.0),
                NiceFloat(self.resampler_frequency / 2.0 - 2000.0),
            )
            .0;
            for channel in &mut self.channels {
                let passes = 3;
                for pass in 0..passes {
                    let mut filter = Biquad::default();
                    let q = Biquad::butterworth(passes * 2, pass);
                    filter.reset(
                        biquad::FilterType::LowPass,
                        cutoff_frequency,
                        self.frequency,
                        q,
                        0.0,
                    );
                    channel.nyquist.push(filter);
                }
            }
        }
    }

    pub fn add_high_pass_filter(&mut self, cutoff_frequency: f64, order: u32, passes: u32) {
        for channel in &mut self.channels {
            for pass in 0..passes {
                if order == 1 {
                    let mut filter = Filter::default();
                    filter.mode = FilterMode::OnePole;
                    filter.f_type = FilterType::HighPass;
                    filter.order = FilterOrder::First;
                    filter.one_pole.reset(
                        one_pole::FilterType::HighPass,
                        cutoff_frequency,
                        self.frequency,
                    );
                    channel.filters.push(filter);
                } else if order == 2 {
                    let mut filter = Filter::default();
                    filter.mode = FilterMode::Biquad;
                    filter.f_type = FilterType::HighPass;
                    filter.order = FilterOrder::Second;
                    let q = Biquad::butterworth(passes * 2, pass);
                    filter.biquad.reset(
                        biquad::FilterType::HighPass,
                        cutoff_frequency,
                        self.frequency,
                        q,
                        0.0,
                    );
                    channel.filters.push(filter);
                }
            }
        }
    }

    pub fn pending(&self) -> bool {
        !self.channels.is_empty() && self.channels[0].resampler.pending()
    }

    // Returns whether to send samples to audio output
    pub fn write(&mut self, samples: &[f64]) -> bool {
        for c in 0..self.channels.len() {
            let mut sample = samples[c] + 1e-25; //constant offset used to suppress denormals
            for filter in &mut self.channels[c].filters {
                match filter.mode {
                    FilterMode::OnePole => sample = filter.one_pole.process(sample),
                    FilterMode::Biquad => sample = filter.biquad.process(sample),
                }
            }
            for filter in &mut self.channels[c].nyquist {
                sample = filter.process(sample);
            }
            self.channels[c].resampler.write(sample);
        }
        //if there are samples pending, then alert the frontend to possibly process them.
        //this will generally happen when every audio stream has pending samples to be mixed.
        self.pending()
    }
}
