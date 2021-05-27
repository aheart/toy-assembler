mod assembly;
mod elf;
mod writer;

use crate::assembly::{
    assemble, Instructions, Interrupt, Register, Value, EXIT_CODE_SUCCESS, FILE_DESCRIPTOR_STDOUT,
    SYSCALL_EXIT, SYSCALL_WRITE,
};
use elf::build_elf;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    use assembly::Instruction::*;

    let data = "Hello World\n";
    // We don't know the address in virtual memory yet
    let data_pointer = Value::new(0);

    let instructions: Instructions = vec![
        Mov(Register::Edx, Value::new(data.len() as u32)),
        Mov(Register::Ecx, data_pointer.clone()),
        Mov(Register::Ebx, Value::new(FILE_DESCRIPTOR_STDOUT)),
        Mov(Register::Eax, Value::new(SYSCALL_WRITE)),
        Int(Interrupt::Syscall),
        Mov(Register::Ebx, Value::new(EXIT_CODE_SUCCESS)),
        Mov(Register::Eax, Value::new(SYSCALL_EXIT)),
        Int(Interrupt::Syscall),
    ];

    // First pass with invalid pointers
    let machine_code = assemble(&instructions);

    let elf = build_elf(machine_code.len(), data.len());

    // Update the virtual address of our data segment
    *data_pointer.borrow_mut() = elf.data_segment_virtual_address() as u32;

    // Second pass with valid pointers
    let machine_code = assemble(&instructions);

    writer::write_binary(&elf, &machine_code, data)
}
