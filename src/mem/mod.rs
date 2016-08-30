use std::process::exit;
use std::clone;
use std::io;
use std::io::prelude::*;
use std::fs::File;

/// Mem represents internal memory type
pub struct Mem {
    /// First 16K section of 32K cartridge ROM
    pub rom: [u8; 0x4000],

    /// Second 16K section of 32K cartridge ROM
    pub switchable_rom: [u8; 0x4000],

    /// 8K video RAM
    pub video_ram: [u8; 0x2000],

    /// 8K switchable RAM bank
    pub switchable_ram: [u8; 0x2000],

    /// 8K internal RAM
    pub internal_ram: [u8; 0x2000],

    /// 8K internal RAM echo
    pub internal_ram_echo: [u8; 0x2000],

    /// 8K memory dedicated to sprite attribs
    pub sprite_attrib_mem: [u8; 0x2000],
}

pub fn init() -> Mem {
    // Initialize memory
    let mut mem = Mem {
        rom: [0; 0x4000],
        switchable_rom: [0; 0x4000],
        video_ram: [0; 0x2000],
        switchable_ram: [0; 0x2000],
        internal_ram: [0; 0x2000],
        internal_ram_echo: [0; 0x2000],
        sprite_attrib_mem: [0; 0x2000],
    };

    // Read ROM file into memory
    // 32K buffer
    let mut temp_buff: [u8; 0x8000] = [0; 0x8000];

    let mut f = match File::open("rom.gb") {
        Result::Ok(handle) => handle,
        Result::Err(_) => {
            println!("Unable to open ROM file");
            exit(1)
        }
    };

    match f.read(&mut temp_buff) {
        Result::Ok(num_bytes) => println!("Successfully read {} bytes from ROM file", num_bytes),
        Result::Err(_) => {
            println!("Unable to read ROM file");
            exit(1)
        }
    };

    mem.rom.clone_from_slice(&temp_buff[..0x4000]);
    mem.switchable_rom.clone_from_slice(&temp_buff[0x4000..]);

    println!("ROM file read successfully");

    mem
}
