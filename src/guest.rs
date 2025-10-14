use std::path::PathBuf;

use crate::{
    memory::{Memory, MemoryError, MemoryPermissions, Region},
    registers::Registers,
};
use goblin::elf::section_header::{self, SHF_EXECINSTR, SHF_WRITE};
use thiserror::Error;
use tracing::{debug, error};

#[derive(Error, Debug)]
pub enum GuestError {
    #[error("Failed to parse ELF file. {0}")]
    ParseFailure(#[from] goblin::error::Error),
    #[error("{0}")]
    MemoryError(#[from] MemoryError),
    #[error("Architecture not supported.")]
    NoArch,
}

pub struct GuestCPU {
    memory: Box<Memory>,
    registers: Registers,
}

impl GuestCPU {
    pub fn from_elf(path: PathBuf) -> Result<Self, Box<dyn Error>> {
        debug!("parsing ELF file from: {path:?}");

        let bytes = std::fs::read(path)?;

        // from the file bytes let goblin parse the ELF file.
        let elf = goblin::elf::Elf::parse(&bytes).map_err(|e| GuestError::ParseFailure(e))?;

        if elf.header.e_machine != goblin::elf::header::EM_RISCV {
            return Err(Box::new(GuestError::NoArch));
        }

        let mut region = Region::new(, );

        for header in &elf.section_headers {
            // ignore sections that don't get loaded into memory, but in the future we might want
            // to get even more sections for metadata reasons, idk.
            if header.sh_flags & section_header::SHF_ALLOC as u64 == 0 {
                continue;
            }

            let ty = match (
                header.sh_flags & SHF_EXECINSTR as u64 != 0,
                header.sh_flags & SHF_WRITE as u64 != 0,
            ) {
                (true, _) => MemoryPermissions::EXECUTE,
                (false, true) => MemoryPermissions::WRITE,
                (false, false) => MemoryPermissions::READ,
            };

            if let Some(range) = header.file_range() {
                // get the section data from the file.
                let data = bytes
                    .get(range)
                    .ok_or(MemoryError::SectionOutsideBounds)?
                    .to_vec();

                region.add_section()
            }
        }

        unimplemented!()
    }
}
