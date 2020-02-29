//
// ppu/bus.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Feb 11 2020
//


use crate::common::{IoAccess, IoAccessRef};
use crate::mapper::Mapper;

const INTERNAL_RAM: usize = 0x1000;

pub struct PpuIoBus {
    cpu: IoAccessRef,
    mapper: Mapper,

    nametable_ram: [u8; INTERNAL_RAM],
    palette_ram: [u8; 256],

    vertical_mirroring: bool,
}

impl PpuIoBus {
    pub fn new(cpu_io: IoAccessRef, mapper: Mapper) -> Self {
        PpuIoBus {
            cpu: cpu_io,
            mapper: mapper,

            nametable_ram: [0x00; INTERNAL_RAM],
            palette_ram: [0x00; 256],

            vertical_mirroring: false,
        }
    }
}

impl IoAccess for PpuIoBus {
    fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x1FFF => self.mapper.borrow().read_chr(addr),
            0x2000..=0x2FFF => {
                self.nametable_ram[(helpers::calc_nametable_addr(addr, self.vertical_mirroring) - 0x2000) as usize]
            },
            0x3000..=0x3EFF => {
                self.nametable_ram[(helpers::calc_nametable_addr(addr - 0x1000, self.vertical_mirroring) - 0x2000) as usize]
            },
            0x3F00..=0x3FFF => self.palette_ram[(addr - 0x3F00) as usize],

            _ => panic!("Invalid read {:04X}", addr),
        }
    }

    fn write_byte(&mut self, addr: u16, value: u8) {
        match addr {
            0x0000..=0x1FFF => self.mapper.borrow_mut().write_chr(addr, value),
            0x2000..=0x2FFF => {
                self.nametable_ram[(helpers::calc_nametable_addr(addr, self.vertical_mirroring) - 0x2000) as usize] = value
            },
            0x3000..=0x3EFF => {
                self.nametable_ram[(helpers::calc_nametable_addr(addr - 0x1000, self.vertical_mirroring) - 0x2000) as usize] = value;
            },
            0x3F00..=0x3FFF => self.palette_ram[(addr - 0x3F00) as usize] = value,

            _ => panic!("Invalid write {:04X}={:02X}", addr, value),
        }
    }

    fn raise_interrupt(&mut self) {
        self.cpu.borrow_mut().raise_interrupt();
    }
}

mod helpers {
    pub fn calc_nametable_addr(addr: u16, vertically_mirrored: bool) -> u16 {
        if vertically_mirrored {
            match addr {
                0x2000..=0x27FF => addr + 0x800,
                _ => addr,
            }
        }
        else {
            match addr {
                0x2000..=0x23FF | 0x2800..=0x2BFF => addr + 0x400,
                _ => addr,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mapper::MapperControl;

    #[test]
    fn horizontal_mirroring() {
        assert_eq!(helpers::calc_nametable_addr(0x2000, false), 0x2400);
        assert_eq!(helpers::calc_nametable_addr(0x2800, false), 0x2C00);
    }

    #[test]
    fn vertical_mirroring() {
        assert_eq!(helpers::calc_nametable_addr(0x2000, true), 0x2800);
        assert_eq!(helpers::calc_nametable_addr(0x2400, true), 0x2C00);
    }

    //------------------------------------------------------------------------------------------------------------------
    // Helpers
    //------------------------------------------------------------------------------------------------------------------

    struct FakeCpu {
        interrupted: bool,
    }

    impl FakeCpu {
        fn new() -> Self {
            FakeCpu {
                interrupted: false,
            }
        }
    }

    impl IoAccess for FakeCpu {
        fn raise_interrupt(&mut self) {
            self.interrupted = true;
        }
    }

    struct FakeMapper {
        prg: [u8; 10],
        chr: [u8; 10],
    }

    impl FakeMapper {
        fn new() -> Self {
            FakeMapper {
                prg: [0; 10],
                chr: [0; 10],
            }
        }
    }

    impl MapperControl for FakeMapper {
        fn read(&self, addr: u16) -> u8 {
            self.prg[addr as usize]
        }

        fn write(&mut self, addr: u16, data: u8) {
            self.prg[addr as usize] = data;
        }

        fn read_chr(&self, addr: u16) -> u8 {
            self.chr[addr as usize]
        }

        fn write_chr(&mut self, addr: u16, value: u8) {
            self.chr[addr as usize] = value;
        }
    }
}
