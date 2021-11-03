use std::collections::VecDeque;

#[derive(Clone, Debug, Default)]
pub struct Cubic {
    input_frequency: f64,
    output_frequency: f64,
    ratio: f64,
    fraction: f64,
    history: [f64; 4],
    samples: VecDeque<f64>,
}

impl Cubic {
    pub fn reset(&mut self, input_frequency: f64, output_frequency: f64, queue_size: u32) {
        self.input_frequency = input_frequency;
        self.output_frequency = if output_frequency != 0.0 {
            output_frequency
        } else {
            input_frequency
        };
        self.ratio = input_frequency / output_frequency;
        self.fraction = 0.0;
        for sample in &mut self.history {
            *sample = 0.0;
        }
        //default to 20ms max queue size
        self.samples.resize(
            if queue_size != 0 {
                queue_size as usize
            } else {
                (output_frequency * 0.02) as usize
            },
            0.0,
        );
    }

    #[allow(clippy::many_single_char_names)]
    pub fn write(&mut self, sample: f64) {
        let mu = &mut self.fraction;
        let s = &mut self.history;

        s[0] = s[1];
        s[1] = s[2];
        s[2] = s[3];
        s[3] = sample;

        while *mu <= 1.0 {
            let a = s[3] - s[2] - s[0] + s[1];
            let b = s[0] - s[1] - a;
            let c = s[2] - s[0];
            let d = s[1];

            self.samples
                .push_back(a * *mu * *mu * *mu + b * *mu * *mu + c * *mu + d);
            *mu += self.ratio;
        }
        *mu -= 1.0;
    }

    pub fn pending(&self) -> bool {
        !self.samples.is_empty()
    }
}
