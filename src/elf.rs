use std::mem::size_of;

const ELF_MAGIC_NUMBER: [u8; 4] = [0x7f, 'E' as u8, 'L' as u8, 'F' as u8];

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
enum Architecture {
    X86 = 1,
    X64 = 2,
}

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
enum Endianness {
    Little = 1,
    Big = 2,
}

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
enum OsAbi {
    SystemV = 0x00,
    HpUx = 0x01,
    NetBsd = 0x02,
    Linux = 0x03,
    GnuHurd = 0x04,
    Solaris = 0x06,
    Aix = 0x07,
    Irix = 0x08,
    FreeBsd = 0x09,
    Tru64 = 0x0A,
    NovellModesto = 0x0B,
    OpenBsd = 0x0C,
    OpenVms = 0x0D,
    NonStopKernel = 0x0E,
    Aros = 0x0F,
    FenixOs = 0x10,
    CloudAbi = 0x11,
    StratusTechnologiesOpenVos = 0x12,
}

#[derive(Copy, Clone, Debug)]
#[repr(u16)]
enum ObjectFileType {
    EtNone = 0x00,
    EtRel = 0x01,
    EtExec = 0x02,
    EtDyn = 0x03,
    EtCore = 0x04,
    EtLoos = 0xFE00,
    EtHios = 0xFEFF,
    EtLoproc = 0xFF00,
    EtHiproc = 0xFFFF,
}

#[derive(Copy, Clone, Debug)]
#[repr(u16)]
enum InstructionSetArchitecture {
    AmdX64 = 0x3E,
}

#[derive(Debug)]
#[repr(C)]
pub struct Elf {
    // e_ident[EI_MAG0] through e_ident[EI_MAG3]
    magic_number: [u8; 4],
    // e_ident[EI_CLASS]
    architecture: Architecture,
    // e_ident[EI_DATA]
    endianness: Endianness,
    // e_ident[EI_VERSION]
    elf_version: u8,
    // e_ident[EI_OSABI]
    os_abi: OsAbi,
    // e_ident[EI_ABIVERSION]
    abi_version: u8,
    // e_ident[EI_PAD]
    unused: [u8; 7],
    // e_type
    file_type: ObjectFileType,
    // e_machine
    machine: InstructionSetArchitecture,
    // e_version
    version: u32,
    // e_entry
    entry_point: u64,
    // e_phoff
    header_offset: u64,
    // e_shoff
    section_header_offset: u64,
    // e_flags
    flags: u32,
    // e_ehsize
    header_size: u16,
    // e_phentsize
    program_header_table_entry_size: u16,
    // e_phnum
    program_header_table_entries: u16,
    // e_shentsize
    section_header_table_entry_size: u16,
    // e_shnum
    section_header_table_entries: u16,
    // e_shstrndx
    section_name_index: u16,

    pub program_headers: [ProgramHeader; 2],
}

#[derive(Debug)]
#[repr(C)]
pub struct ProgramHeader {
    // p_type
    segment_type: u32,
    // p_flags
    segment_flags: u32,
    // p_offset
    pub segment_offset: u64,
    // p_vaddr
    pub virtual_address: u64,
    // p_paddr
    physical_address: u64,
    // p_filesz
    segment_size_in_file: u64,
    // p_memsz
    segment_size_in_memory: u64,
    // p_align
    alignment: u64,
}

// #[derive(Debug)]
// #[repr(C)]
// struct SectionHeader {
//     // sh_name
//     name_offset: u32,
//     // p_flags
//     header_type: u32,
//     // sh_flags
//     section_flags: u64,
//     // sh_addr
//     virtual_address: u64,
//     // sh_offset
//     offset: u64,
//     // sh_size
//     size: u64,
//     // sh_link
//     link: u32,
//     // sh_info
//     info: u32,
//     // sh_addralign
//     alignment: u64,
//     // sh_entsize
//     entry_size: u64,
// }

pub fn build_elf(code_length: usize, data_length: usize) -> Elf {
    let code_offset = align(size_of::<Elf>(), 8);
    let header_and_code_segment_length = code_offset + code_length;
    let data_offset = align(code_offset + code_length, 4) as u64;
    let virtual_data_offset = 0x6000000 + data_offset;

    Elf {
        magic_number: ELF_MAGIC_NUMBER,
        architecture: Architecture::X64,
        endianness: Endianness::Little,
        elf_version: 1,
        os_abi: OsAbi::SystemV,
        abi_version: 0,
        unused: [0; 7],
        file_type: ObjectFileType::EtExec,
        machine: InstructionSetArchitecture::AmdX64,
        version: 1,
        entry_point: 0x400000 + code_offset as u64,
        header_offset: 0x40,
        section_header_offset: 0,
        flags: 0,
        header_size: 0x40,
        program_header_table_entry_size: 0x38,
        program_header_table_entries: 2,
        section_header_table_entry_size: 0x40,
        section_header_table_entries: 0,
        section_name_index: 0,
        program_headers: [
            ProgramHeader {
                segment_type: 1,
                segment_flags: 5,
                segment_offset: 0,
                virtual_address: 0x400000,
                physical_address: 0x400000,
                segment_size_in_file: header_and_code_segment_length as u64,
                segment_size_in_memory: header_and_code_segment_length as u64,
                alignment: 0x200000,
            },
            ProgramHeader {
                segment_type: 1,
                segment_flags: 6,
                segment_offset: data_offset,
                virtual_address: virtual_data_offset,
                physical_address: virtual_data_offset,
                segment_size_in_file: data_length as u64,
                segment_size_in_memory: data_length as u64,
                alignment: 0x200000,
            },
        ],
    }
}

impl Elf {
    pub fn data_segment_virtual_address(&self) -> u64 {
        self.program_headers[1].virtual_address
    }
}

pub fn align(number: usize, align_to: usize) -> usize {
    let remainder = number % align_to;
    if remainder == 0 {
        number
    } else {
        number + align_to - remainder
    }
}
