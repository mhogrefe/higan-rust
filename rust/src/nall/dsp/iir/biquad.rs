use std::f64::consts::PI;

#[derive(Clone, Copy, Debug)]
pub enum FilterType {
    LowPass,
    HighPass,
    BandPass,
    Notch,
    Peak,
    LowShelf,
    HighShelf,
}

impl Default for FilterType {
    fn default() -> FilterType {
        FilterType::LowPass
    }
}

#[derive(Clone, Debug, Default)]
pub struct Biquad {
    f_type: FilterType,
    cutoff_frequency: f64,
    sampling_frequency: f64,
    quality: f64, //frequency response quality
    gain: f64,    //peak gain
    a0: f64,
    a1: f64,
    a2: f64,
    b1: f64,
    b2: f64,
    z1: f64,
    z2: f64,
}

impl Biquad {
    // compute Q values for Nth-order butterworth filtering
    pub fn butterworth(order: u32, phase: u32) -> f64 {
        let order = f64::from(order);
        -0.5 / (PI * (f64::from(phase) + order + 0.5) / order).cos()
    }

    pub fn reset(
        &mut self,
        f_type: FilterType,
        cutoff_frequency: f64,
        sampling_frequency: f64,
        quality: f64,
        gain: f64,
    ) {
        self.f_type = f_type;
        self.cutoff_frequency = cutoff_frequency;
        self.sampling_frequency = sampling_frequency;
        self.quality = quality;
        self.gain = gain;

        self.z1 = 0.0;
        self.z2 = 0.0;

        let v = 10.0f64.powf(gain.abs() / 20.0);
        let k = (PI * cutoff_frequency / sampling_frequency).tan();
        let q = quality;
        match f_type {
            FilterType::LowPass => {
                let n = 1.0 / (1.0 + k / q + k * k);
                self.a0 = k * k * n;
                self.a1 = 2.0 * self.a0;
                self.a2 = self.a0;
                self.b1 = 2.0 * (k * k - 1.0) * n;
                self.b2 = (1.0 - k / q + k * k) * n;
            }
            FilterType::HighPass => {
                let n = 1.0 / (1.0 + k / q + k * k);
                self.a0 = 1.0 * n;
                self.a1 = -2.0 * self.a0;
                self.a2 = self.a0;
                self.b1 = 2.0 * (k * k - 1.0) * n;
                self.b2 = (1.0 - k / q + k * k) * n;
            }
            FilterType::BandPass => {
                let n = 1.0 / (1.0 + k / q + k * k);
                self.a0 = k / q * n;
                self.a1 = 0.0;
                self.a2 = -self.a0;
                self.b1 = 2.0 * (k * k - 1.0) * n;
                self.b2 = (1.0 - k / q + k * k) * n;
            }
            FilterType::Notch => {
                let n = 1.0 / (1.0 + k / q + k * k);
                self.a0 = (1.0 + k * k) * n;
                self.a1 = 2.0 * (k * k - 1.0) * n;
                self.a2 = self.a0;
                self.b1 = self.a1;
                self.b2 = (1.0 - k / q + k * k) * n;
            }
            FilterType::Peak => {
                if gain >= 0.0 {
                    let n = 1.0 / (1.0 + 1.0 / q * k + k * k);
                    self.a0 = (1.0 + v / q * k + k * k) * n;
                    self.a1 = 2.0 * (k * k - 1.0) * n;
                    self.a2 = (1.0 - v / q * k + k * k) * n;
                    self.b1 = self.a1;
                    self.b2 = (1.0 - 1.0 / q * k + k * k) * n;
                } else {
                    let n = 1.0 / (1.0 + v / q * k + k * k);
                    self.a0 = (1.0 + 1.0 / q * k + k * k) * n;
                    self.a1 = 2.0 * (k * k - 1.0) * n;
                    self.a2 = (1.0 - 1.0 / q * k + k * k) * n;
                    self.b1 = self.a1;
                    self.b2 = (1.0 - v / q * k + k * k) * n;
                }
            }
            FilterType::LowShelf => {
                if gain >= 0.0 {
                    let n = 1.0 / (1.0 + k / q + k * k);
                    self.a0 = (1.0 + v.sqrt() / q * k + v * k * k) * n;
                    self.a1 = 2.0 * (v * k * k - 1.0) * n;
                    self.a2 = (1.0 - v.sqrt() / q * k + v * k * k) * n;
                    self.b1 = 2.0 * (k * k - 1.0) * n;
                    self.b2 = (1.0 - k / q + k * k) * n;
                } else {
                    let n = 1.0 / (1.0 + v.sqrt() / q * k + v * k * k);
                    self.a0 = (1.0 + k / q + k * k) * n;
                    self.a1 = 2.0 * (k * k - 1.0) * n;
                    self.a2 = (1.0 - k / q + k * k) * n;
                    self.b1 = 2.0 * (v * k * k - 1.0) * n;
                    self.b2 = (1.0 - v.sqrt() / q * k + v * k * k) * n;
                }
            }
            FilterType::HighShelf => {
                if gain >= 0.0 {
                    let n = 1.0 / (1.0 + k / q + k * k);
                    self.a0 = (v + v.sqrt() / q * k + k * k) * n;
                    self.a1 = 2.0 * (k * k - v) * n;
                    self.a2 = (v - v.sqrt() / q * k + k * k) * n;
                    self.b1 = 2.0 * (k * k - 1.0) * n;
                    self.b2 = (1.0 - k / q + k * k) * n;
                } else {
                    let n = 1.0 / (v + v.sqrt() / q * k + k * k);
                    self.a0 = (1.0 + k / q + k * k) * n;
                    self.a1 = 2.0 * (k * k - 1.0) * n;
                    self.a2 = (1.0 - k / q + k * k) * n;
                    self.b1 = 2.0 * (k * k - v) * n;
                    self.b2 = (v - v.sqrt() / q * k + k * k) * n;
                }
            }
        }
    }

    pub fn process(&mut self, i: f64) -> f64 {
        let out = i * self.a0 + self.z1;
        self.z1 = i * self.a1 + self.z2 - self.b1 * out;
        self.z2 = i * self.a2 - self.b2 * out;
        out
    }
}
