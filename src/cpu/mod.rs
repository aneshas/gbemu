/// Game Boy CPU

use std::process::exit;
use mem;

pub struct CPU {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
    sp: u16,
    pc: u16,
    flag: u8,
}

impl CPU {
    fn af(&self) -> u16 {
        combine_h_l(self.a, self.f)
    }

    fn af_set(&mut self, v: u16) {
        let (high, low) = to_h_l(v);
        self.a = high;
        self.f = low;
    }

    fn bc(&self) -> u16 {
        combine_h_l(self.b, self.c)
    }

    fn bc_set(&mut self, v: u16) {
        let (high, low) = to_h_l(v);
        self.b = high;
        self.c = low;
    }

    fn de(&self) -> u16 {
        combine_h_l(self.d, self.e)
    }

    fn de_set(&mut self, v: u16) {
        let (high, low) = to_h_l(v);
        self.d = high;
        self.e = low;
    }

    fn hl(&self) -> u16 {
        combine_h_l(self.h, self.l)
    }

    fn hl_set(&mut self, v: u16) {
        let (high, low) = to_h_l(v);
        self.h = high;
        self.l = low;
    }

    fn execute_instruction(&mut self, i: Instruction) {
        match i {
            Instruction::NOP => {
                println!("NOP instruction executed");
                self.pc += 1;
            }

            Instruction::JP(ls, ms) => {
                let addr = combine_h_l(ms, ls);
                self.pc = addr;
                println!("Jumping to {:x}", addr);
            }
        }
    }
}

enum Instruction {
    NOP,

    // LS byte first
    JP(u8, u8),
}

fn init() -> CPU {
    CPU {
        a: 0x00,
        b: 0x00,
        c: 0x00,
        d: 0x00,
        e: 0x00,
        f: 0x00,
        h: 0x00,
        l: 0x00,
        pc: 0x100,
        sp: 0xfffe,
        flag: 0x00,
    }
}

pub fn run(m: &mut Box<mem::Mem>) {
    let mut cpu = init();

    loop {
        let instruction = decode_op(m.rom[cpu.pc as usize], cpu.pc, m);
        cpu.execute_instruction(instruction);
    }
}

fn decode_op(op: u8, pc: u16, m: &mut Box<mem::Mem>) -> Instruction {
    match op {
        // NOP
        0x00 => Instruction::NOP,

        // JP nn
        0xC3 => {
            let instruction = Instruction::JP(m.rom[(pc + 1) as usize], m.rom[(pc + 2) as usize]);
            instruction
        }

        _ => {
            println!("Instruction not implemented: {:x}", op);
            exit(1);
        }
    }
}

fn to_h_l(b: u16) -> (u8, u8) {
    let low = (b & 0xff) as u8;
    let high = (b >> 8) as u8;

    (high, low)
}

fn combine_h_l(h: u8, l: u8) -> u16 {
    (h as u16) << 8 | (l as u16)
}
