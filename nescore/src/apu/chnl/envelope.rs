//
// apu/chnl/envelope.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date May 17 2020
//
use super::Divider;
use crate::common::Clockable;

const DECAY_RELOAD: u8 = 15;

pub trait EnvelopeUnit {
    fn clock_envelope(&mut self);
    // fn envelope_start(&mut self);
}

#[macro_export]
macro_rules! impl_envelope {
    ($t:ident, $f:ident) => {
        impl EnvelopeUnit for $t {
            fn clock_envelope(&mut self) {
                self.$f.tick();
            }
        }
    };
}

#[derive(Default)]
pub struct Envelope {
    divider: Divider,
    decay: u8,

    volume: u8,
    constant: bool,
    loop_flag: bool,

    start_flag: bool,
}

impl Clockable for Envelope {
    fn tick(&mut self) {
        if !self.start_flag {
            if self.divider.tick() {

                if self.decay == 0 {
                    if self.loop_flag {
                        self.decay = DECAY_RELOAD;
                    }
                }
                else {
                    self.decay -= 1;
                }
            }
        }
        else {
            self.start_flag = false;
            self.divider.set_period(self.volume as u32);
            self.decay = DECAY_RELOAD;
        }
    }
}

impl Envelope {
    pub fn set_volume(&mut self, volume: u8) {
        self.volume = volume;
    }

    pub fn set_loop(&mut self, loop_flag: bool) {
        self.loop_flag = loop_flag;
    }

    pub fn set_constant(&mut self, constant: bool) {
        self.constant = constant;
    }

    pub fn start(&mut self) {
        self.start_flag = true;
    }

    pub fn output(&self) -> u8 {
        if self.constant {
            self.volume
        }
        else {
            self.decay
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /**
        Envelope Configuration
        ----------------------

        | Volume   | 15         |
        | Constant | true/false |
        | Loop     | true/false |
    */

    #[test]
    fn decay_reload() {
        let mut envelope = Envelope::default();
        envelope.set_constant(false);
        envelope.set_volume(10);
        envelope.set_loop(true);

        envelope.start();
        // First tick to initialize the envelope unit
        envelope.tick();

        assert_eq!(envelope.output(), 15);

        // Clock decay and check envelope output
        for output in (0..15).rev() {
            // Envelope period is volume + 1
            clock_period(&mut envelope, 10 + 1);
            assert_eq!(envelope.output(), output);
        }

        clock_period(&mut envelope, 10 + 1);

        assert_eq!(envelope.output(), 15);
    }

    #[test]
    fn decay_no_reload() {
        let mut envelope = Envelope::default();
        envelope.set_loop(false);
        envelope.set_volume(10);

        envelope.start();
        // First tick to initialize the envelope unit
        envelope.tick();

        assert_eq!(envelope.output(), 15);

        // Clock decay and check envelope output
        for output in (0..15).rev() {
            // Envelope period is volume + 1
            clock_period(&mut envelope, 10 + 1);
            assert_eq!(envelope.output(), output);
        }

        clock_period(&mut envelope, 10 + 1);

        assert_eq!(envelope.output(), 0);
    }

    #[test]
    fn constant() {
        let mut envelope = Envelope::default();
        envelope.set_volume(15);
        envelope.set_constant(true);

        envelope.start();

        for _ in 0..22 {
            envelope.tick();
        }

        assert_eq!(envelope.output(), 15);
    }

    fn clock_period(envelope: &mut Envelope, period: u32) {
        for _ in 0..period {
            envelope.tick();
        }
    }
}
