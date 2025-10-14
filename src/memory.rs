use std::ops::Range;

use bitflags::bitflags;
use thiserror::Error;
use tracing::error;

#[derive(Error, Debug)]
pub enum MemoryError {
    #[error("Section outside of bounds.")]
    SectionOutsideBounds,
    #[error("Invalid permissions to perform action.")]
    ProtectionFault,
}

bitflags! {
    /// RWX permissions for memory.
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct MemoryPermissions: u32 {
        const NONE    = 0b000;
        const READ    = 0b001;
        const WRITE   = 0b010;
        const EXECUTE = 0b100;
        const ALL     = 0b111;
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Section {
    range: Range<usize>,
    permissions: MemoryPermissions,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Region {
    base: u64,
    data: Vec<u8>,
    sections: Vec<Section>,
}

impl Region {
    pub fn new(base: u64, size: usize) -> Self {
        Self {
            base,
            data: vec![0; size],
            sections: Vec::new(),
        }
    }

    /// Creates a region but the entire region is allocated, think: malloc or something.
    pub fn new_filled(base: u64, size: usize, perms: MemoryPermissions) -> Self {
        let mut r = Self::new(base, size);

        r.sections.push(Section {
            range: 0..size,
            permissions: perms,
        });

        r
    }

    fn add_section(
        &mut self,
        vaddr: u64,
        perms: MemoryPermissions,
        bytes: &[u8],
    ) -> Result<(), MemoryError> {
        let start = vaddr
            .checked_sub(self.base)
            .ok_or(MemoryError::SectionOutsideBounds)? as usize;

        let end = start + bytes.len();

        if end > self.data.len() {
            error!("tried to add section but end is outside memory region.");
            return Err(MemoryError::SectionOutsideBounds);
        }

        // copy bytes into memory.
        self.data[start..end].copy_from_slice(bytes);

        // add the section to the region.
        self.sections.push(Section {
            range: start..end,
            permissions: perms,
        });

        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Memory {
    pub regions: Vec<Region>,
}
