use ares::gb::system::{Model, System};
use ares::platform::Platform;

impl<P: Platform> System<P> {
    pub fn controls_poll(&mut self) {
        if self.model == Model::SuperGameBoy {
            return;
        }

        self.platform.input(&mut self.controls.up);
        self.platform.input(&mut self.controls.down);
        self.platform.input(&mut self.controls.left);
        self.platform.input(&mut self.controls.right);
        self.platform.input(&mut self.controls.b);
        self.platform.input(&mut self.controls.a);
        self.platform.input(&mut self.controls.select);
        self.platform.input(&mut self.controls.start);

        if !(self.controls.up.as_button_mut().value && self.controls.down.as_button_mut().value) {
            self.controls.y_hold = false;
            self.controls.up_latch = self.controls.up.as_button_ref().value;
            self.controls.down_latch = self.controls.down.as_button_ref().value;
        } else if !self.controls.y_hold {
            self.controls.y_hold = true;
            let t = self.controls.up_latch;
            self.controls.up_latch = self.controls.down_latch;
            self.controls.down_latch = t;
        }

        if !(self.controls.left.as_button_mut().value && self.controls.right.as_button_mut().value)
        {
            self.controls.x_hold = false;
            self.controls.left_latch = self.controls.left.as_button_ref().value;
            self.controls.right_latch = self.controls.right.as_button_ref().value;
        } else if !self.controls.x_hold {
            self.controls.x_hold = true;
            let t = self.controls.left_latch;
            self.controls.left_latch = self.controls.right_latch;
            self.controls.right_latch = t;
        }
    }
}
