
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Format {
    INES,
    NES2
}

#[derive(Debug)]
pub struct Info {
    pub format: Format,
    pub prg_rom_banks: usize,    // 16kB units
    pub chr_rom_banks: usize,    // 8kB units (0 means board uses CHR RAM)
    pub mapper: usize,           // Mapper Number
    pub four_screen_mode: bool,  // Four screen mode
    pub trainer: bool,           // Trainer present
    pub battback_sram: bool,     // Battery backed SRAM at $6000-$7000
    pub mirror_v: bool,          // Vertical mirroring if true, horizontal if false
    pub vs_unisystem: bool,      // VS Unisystem
    pub playchoice10: bool,      // PlayChoice
    pub tv_system_pal: bool,     // NTSC if false, PAL if true
    pub tv_system_ext: usize,    // Unoffical TV supper, 0 - NTSC, 1 - PAL, 2 - Dual Compat
    
    // below are NES 2.0 only
    pub submapper: usize,        // Submapper number
    pub mapper_planes: usize,    // Mapper planes
    pub batt_prg_ram: usize,     // Amount of battery backed PRG RAM
    pub prg_ram: usize,          // Amount of non-battery backed PRG RAM
    pub batt_chr_ram: usize,     // Amount of battery backed CHR RAM
    pub chr_ram: usize,          // Amount of non-battery backed CHR RAM
}

pub enum ParseError {
    InvalidSize(usize),
    InvalidSig,
    InvalidFormat
}

impl fmt::Debug for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseError::InvalidSig =>     write!(f, "Invalid signature at start of file. Expected `NES`. Not an NES ROM"),
            ParseError::InvalidSize(s) => write!(f, "Not enough data to parse header (Size: {})", s),
            ParseError::InvalidFormat =>  write!(f, "The detected header is not valid")
        }
    }
}

/// Parse NES ROM header
pub fn parse_header(rom_header: &[u8]) -> Result<Info, ParseError> {
    if rom_header.len() < 16 {
        return Err(ParseError::InvalidSize(rom_header.len()))
    }

    if !verify_signature(&rom_header[0..4]) {
        return Err(ParseError::InvalidSig)
    }

    let format = match get_format(rom_header) {
        Ok(f) => f,
        Err(e) => return Err(e),
    };

    let info = get_rom_info(rom_header, format);

    Ok(info)
}

/// Pull rom info from header
fn get_rom_info(rom_header: &[u8], format: Format) -> Info {
    let mut info = Info {
        format: format.clone(),
        prg_rom_banks: 0,
        chr_rom_banks: 0,
        mapper: 0,
        four_screen_mode: false,
        trainer: false,
        battback_sram: false,
        mirror_v: false,
        vs_unisystem: false,
        playchoice10: false,
        tv_system_pal: false,
        tv_system_ext: 0,
        submapper: 0,
        mapper_planes: 0,
        batt_prg_ram: 0,
        prg_ram: 0,
        batt_chr_ram: 0,
        chr_ram: 0,
    };

    get_info_common(rom_header, &mut info);

    match format {
        Format::INES => get_info_ines(rom_header, &mut info),
        Format::NES2 => get_info_nes2(rom_header, &mut info),
    }

    info
}

fn get_info_common(rom_header: &[u8], info: &mut Info) {
    // get program and character bank info
    let prg_rom_banks = rom_header[4] as usize;
    let chr_rom_banks = rom_header[5] as usize;

    let flag6 = rom_header[6];
    let flag7 = rom_header[7];

    // vertical mirroring flag
    let mirror_v = (flag6 & 0x01u8) != 0;
    // battery backed SRAM flag
    let battback_sram = (flag6 & 0x02u8) != 0;
    // trainer
    let trainer = (flag6 & 0x04u8) != 0;
    // four screen mode
    let four_screen_mode = (flag6 & 0x08u8) != 0;

    let vs_unisystem = (flag7 & 0x01) != 0;
    let playchoice10 = (flag7 & 0x02) != 0;

    // mapper number
    let mapper_lo = flag6 >> 4;
    let mapper_hi = flag7 & 0xF0;
    let mapper = mapper_hi | mapper_lo;

    info.prg_rom_banks = prg_rom_banks;
    info.chr_rom_banks = chr_rom_banks;
    info.mirror_v = mirror_v;
    info.battback_sram = battback_sram;
    info.trainer = trainer;
    info.four_screen_mode = four_screen_mode;
    info.mapper = mapper as usize;
    info.vs_unisystem = vs_unisystem;
    info.playchoice10 = playchoice10;
}

/// Get info from INES formatted ROM
fn get_info_ines(rom_header: &[u8], info: &mut Info) {
    // TV system support
    info.tv_system_pal = (rom_header[9] & 0x01u8) != 0;
    info.tv_system_ext = match rom_header[10] & 0x03u8 {
        0     => 0,
        2     => 1,
        1 | 3 => 2,
        _     => 0
    };
}

/// Get info from INES formatted ROM
fn get_info_nes2(rom_header: &[u8], info: &mut Info) {
    // additional mapper info
    let submapper = (rom_header[8] & 0xF0u8) >> 4;
    let mapper_planes = rom_header[8] & 0x0Fu8;

    info.submapper = submapper as usize;
    info.mapper_planes = mapper_planes as usize;

    // extend PRG and CHR rom size
    let prg_rom_hi_bits = rom_header[9] & 0x0Fu8;
    let chr_rom_hi_bits = (rom_header[9] & 0xF0u8) >> 4;

    info.prg_rom_banks |= (prg_rom_hi_bits as usize) << 8;
    info.chr_rom_banks |= (chr_rom_hi_bits as usize) << 8;

    // PRG RAM size
    info.batt_prg_ram = (rom_header[10] >> 4) as usize;
    info.prg_ram = (rom_header[10] & 0x0Fu8) as usize;

    // CHR RAM size
    info.batt_chr_ram = (rom_header[11] >> 4) as usize;
    info.chr_ram = (rom_header[11] & 0x0Fu8) as usize;

    // TV system
    info.tv_system_ext = if (rom_header[12] & 0x01) != 0 {
        0
    }
    else {
        1
    }
}

/// Get the NES ROM format
fn get_format(rom_header: &[u8]) -> Result<Format, ParseError> {
    let flag7 = rom_header[7];

    if (flag7 & 0x0Cu8) == 0x08u8 {
        return Ok(Format::NES2);
    }
    else {
        // if this is an INES format rom, bytes 8-15 should be $00
        let empty_bytes = &rom_header[12..16];

        if empty_bytes == [0, 0, 0, 0] {
            return Ok(Format::INES);
        }
        else {
            return Err(ParseError::InvalidFormat)
        }
    }
}

/// Verify the signature at the start of the file `NES<EOF>`
fn verify_signature(sig: &[u8]) -> bool {
    sig == [0x4E, 0x45, 0x53, 0x1A]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_format_nes2() {
        let mut rom_header = [0; 16];
        rom_header[7] = 0x08u8;

        let format = get_format(&rom_header[..]).unwrap();
        assert_eq!(format, Format::NES2);
    }

    #[test]
    fn test_get_format_ines() {
        let mut rom_header = [0; 16];
        rom_header[7] = 0x04u8;

        let format = get_format(&rom_header[..]).unwrap();
        assert_eq!(format, Format::INES);
    }
}
