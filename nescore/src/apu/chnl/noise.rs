//
// apu/chnl/noise.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Apr 03 2020
//
use crate::common::{Clockable, IoAccess};
use super::{SoundChannel, LengthCounter, LengthCounterUnit};

#[derive(Default)]
pub struct Noise {
    lenctr: LengthCounter,
}

impl_length_counter!(Noise, lenctr);

impl SoundChannel for Noise {

}

impl Clockable for Noise {
    fn tick(&mut self) {

    }
}

impl IoAccess for Noise {
    #[allow(unused)]
    fn read_byte(&self, addr: u16) -> u8 {
        0
    }

    fn write_byte(&mut self, reg: u16, data: u8) {
        match reg {
            0 => {
                self.lenctr.set_halt(bit_is_set!(data, 5));
            },
            1 => {},
            2 => {},
            3 => {
                self.lenctr.load(bit_group!(data, 0x1F, 3) as usize);
            },

            _ => panic!("invalid register for Noise channel"),
        }
    }
}
