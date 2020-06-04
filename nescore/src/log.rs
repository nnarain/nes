//
// log.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Jun 04 2020
//

use crate::events::CpuEvent;
use crate::cpu::format;

pub fn console(event: CpuEvent) {
    match event {
        CpuEvent::Instruction(data) => {
            println!("${:04X} | {} | {} | A={:02X}, X={:02X}, Y={:02X}, P={:02X}, SP={:04X}",
                data.addr,
                format::operands(&data.opcode_data[..], data.mode.operand_len()),
                format::disassemble(data.instr, data.mode, &data.opcode_data[..]),
                data.a, data.x, data.y, data.p, data.sp);
        },
    }
}
