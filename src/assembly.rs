use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

#[derive(Debug)]
pub enum Register {
    Eax,
    Ebx,
    Ecx,
    Edx,
}

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Interrupt {
    Syscall = 0x80,
}

pub const SYSCALL_WRITE: u32 = 4;
pub const SYSCALL_EXIT: u32 = 1;

pub const FILE_DESCRIPTOR_STDOUT: u32 = 1;

pub const EXIT_CODE_SUCCESS: u32 = 0;

#[derive(Debug, Clone)]
pub struct Value(Rc<RefCell<u32>>);

impl Value {
    pub fn new(value: u32) -> Self {
        Value(Rc::new(RefCell::new(value)))
    }

    pub fn borrow(&self) -> Ref<u32> {
        self.0.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<u32> {
        self.0.borrow_mut()
    }
}

#[derive(Debug)]
pub enum Instruction {
    Mov(Register, Value),
    Int(Interrupt),
}

pub type Instructions = Vec<Instruction>;

pub fn assemble(assembly: &Instructions) -> Vec<u8> {
    assembly
        .iter()
        .flat_map(|i| match i {
            Instruction::Mov(register, data) => {
                let mut code: Vec<u8> = Vec::with_capacity(5);

                let instruction: u8 = match register {
                    Register::Eax => 0xb8,
                    Register::Ebx => 0xbb,
                    Register::Ecx => 0xb9,
                    Register::Edx => 0xba,
                };

                code.push(instruction);
                let data = data.borrow().to_le_bytes();
                code.extend_from_slice(&data);

                code
            }
            Instruction::Int(interrupt) => vec![0xcd, *interrupt as u8],
        })
        .collect()
}
