///
/// nescore/lib.rs
///
/// @author Natesh Narain <nnaraindev@gmail.com>
///

#[macro_use] mod bit;
#[macro_use] mod common;
mod cpu;
mod ppu;
mod apu;
mod mapper;
mod joy;

pub mod cart;
pub use cart::Cartridge;
pub use ppu::{DISPLAY_WIDTH, DISPLAY_HEIGHT};
pub use joy::{Controller, Button};

use cpu::Cpu;
use cpu::bus::CpuIoBus;
use ppu::Ppu;
use ppu::bus::PpuIoBus;
use apu::Apu;
use joy::Joy;
use mapper::Mapper;
use common::Clockable;

use std::rc::Rc;
use std::cell::RefCell;

/// Size of the display frame buffer: display size * RGB (3 bytes)
const FRAME_BUFFER_SIZE: usize = DISPLAY_WIDTH * DISPLAY_HEIGHT * 3;

/// Representation of the NES system
#[derive(Default)]
pub struct Nes {
    cpu: Rc<RefCell<Cpu<CpuIoBus>>>,    // NES Central Processing Uni
    ppu: Rc<RefCell<Ppu<PpuIoBus>>>,    // NES Picture Processing Unit
    apu: Rc<RefCell<Apu>>,              // NES Audio Processing Unit
    joy: Rc<RefCell<Joy>>,              // NES Joystick
                                        // TODO: APU
    mapper: Option<Mapper> // Catridge Mapper
}

impl Nes {
    /// Directly set the CPU entry point
    /// ```
    /// # use nescore::Nes;
    /// let nes = Nes::default().entry(0xC000);
    /// ```
    pub fn entry(self, entry_addr: u16) -> Self {
        self.cpu.borrow_mut().set_pc(entry_addr);
        self
    }

    /// Builder function to allow inserting the cartridge
    pub fn with_cart(mut self, cart: Cartridge) -> Self {
        self.insert(cart);
        self
    }

    /// Builder function to set debug mode
    /// ```
    /// # use nescore::Nes;
    /// let nes = Nes::default().debug_mode(true);
    /// ```
    pub fn debug_mode(self, debug: bool) -> Self {
        self.cpu.borrow_mut().set_debug(debug);
        self
    }

    /// Run the emulator for a single frame
    /// ```
    /// # use nescore::Nes;
    /// let mut nes = Nes::default();
    /// let framebuffer = nes.emulate_frame();
    /// ```
    pub fn emulate_frame(&mut self) -> [u8; FRAME_BUFFER_SIZE] {
        let mut framebuffer = [0x00u8; FRAME_BUFFER_SIZE];
        let mut idx = 0usize;

        if self.mapper.is_some() {
            // TODO: Need some kind of clock sequencer
            let mut count = 0;
            for _ in 0..ppu::CYCLES_PER_FRAME {
                // Clock the PPU and clock the CPU every 3 cycles
                let pixel = self.clock_components(count % 3 == 0, count % 6 == 0);
                if let Some((r, g, b)) = pixel {
                    // Insert RGB data into the frame buffer
                    framebuffer[idx] = r;
                    framebuffer[idx + 1] = g;
                    framebuffer[idx + 2] = b;
                    idx += 3;
                }

                count += 1;
            }
        }

        framebuffer
    }

    /// Apply a button input into the emulator
    /// ```
    /// # use nescore::{Nes, Button};
    /// let mut nes = Nes::default();
    /// nes.input(Button::A, true);
    /// ```
    pub fn input(&mut self, btn: Button, pressed: bool) {
        self.joy.borrow_mut().input(btn, pressed);
    }

    /// Run until the CPU's PC is at address **addr**
    pub fn run_until(&mut self, addr: u16) {
        // TODO: Time limit
        // TODO: Consistent clocking of components
        if self.mapper.is_some() {
            while self.cpu.borrow().get_pc() != addr {
                self.clock_components(true, false);
            }
        }
    }

    /// Clock the NES components
    fn clock_components(&mut self, clock_cpu: bool, clock_apu: bool) -> Option<ppu::Pixel> {
        let pixel = self.ppu.borrow_mut().tick();

        if clock_cpu {
            self.cpu.borrow_mut().tick();
        }

        if clock_apu {
            self.apu.borrow_mut().tick();
        }

        pixel
    }

    /// Check if the CPU is in an infinite loop state
    pub fn is_holding(&self) -> bool {
        self.cpu.borrow().is_holding()
    }

    /// Load a cartridge
    pub fn insert(&mut self, cart: Cartridge) {
        // Consume provided cartridge and get the mapper
        let mapper = mapper::from_cartridge(cart);

        // Complete initialization of components
        let cpu_bus = CpuIoBus::new(self.ppu.clone(), self.apu.clone(), self.joy.clone(), mapper.clone());
        self.cpu.borrow_mut().load_bus(cpu_bus);

        let ppu_bus = PpuIoBus::new(self.cpu.clone(), mapper.clone());
        self.ppu.borrow_mut().load_bus(ppu_bus);

        self.mapper = Some(mapper);
    }

    //------------------------------------------------------------------------------------------------------------------
    // Inspect the state of the NES system
    //------------------------------------------------------------------------------------------------------------------

    /// Get the CPU's program counter
    pub fn get_program_counter(&self) -> u16 {
        self.cpu.borrow().get_pc()
    }

    /// Read the byte, at the specified address, from CPU's internal RAM
    pub fn read_cpu_ram(&self, addr: u16) -> u8 {
        self.cpu.borrow().read_ram(addr)
    }

    /// Read directly from VRAM
    pub fn read_ppu_memory(&self, addr: u16) -> u8 {
        self.ppu.borrow().read_vram(addr)
    }

    /// Read a tile from the current nametable
    pub fn read_tile(&self, nametable: u16, x: usize, y: usize) -> u8 {
        self.ppu.borrow().read_tile(nametable, x, y)
    }
}

#[cfg(test)]
mod tests {

}
