use std::f64::consts::PI;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FilterType {
    LowPass,
    HighPass,
}

impl Default for FilterType {
    fn default() -> FilterType {
        FilterType::LowPass
    }
}

#[derive(Clone, Debug, Default)]
pub struct OnePole {
    f_type: FilterType,
    cutoff_frequency: f64,
    sampling_frequency: f64,
    a0: f64,
    b1: f64,
    z1: f64,
}

impl OnePole {
    pub fn reset(&mut self, f_type: FilterType, cutoff_frequency: f64, sampling_frequency: f64) {
        self.f_type = f_type;
        self.cutoff_frequency = cutoff_frequency;
        self.sampling_frequency = sampling_frequency;

        self.z1 = 0.0;
        let x = (2.0 * PI * cutoff_frequency / sampling_frequency).cos();
        if f_type == FilterType::LowPass {
            self.b1 = 2.0 - x - ((2.0 - x) * (2.0 - x) - 1.0).sqrt();
            self.a0 = 1.0 - self.b1;
        } else {
            self.b1 = -2.0 - x + ((-2.0 - x) * (-2.0 - x) - 1.0).sqrt();
            self.a0 = 1.0 + self.b1;
        }
    }

    pub fn process(&mut self, i: f64) -> f64 {
        self.z1 = i * self.a0 + self.z1 * self.b1;
        self.z1
    }
}
