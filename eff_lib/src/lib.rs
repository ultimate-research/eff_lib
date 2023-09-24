//! # eff_lib
//!
//! eff_lib is a library for reading and writing EFF files from Super Smash Bros. Ultimate.
use std::{
    fs,
    io::{self, Cursor, Read, Seek, Write},
    mem,
    path::Path,
};

use binrw::{binrw, BinReaderExt, BinResult, BinWrite};
use modular_bitfield::prelude::*;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

mod string;

pub use string::CString;

/// The container type for the EFF file format.
#[binrw]
#[brw(magic = b"EFFN")]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub struct EffFile {
    #[br(temp)]
    #[bw(calc = 0x00020000)]
    _version: u32,

    #[br(temp)]
    #[bw(calc = effect_handles.len() as i16)]
    effect_handle_count: i16,

    #[br(temp)]
    #[bw(calc = effect_model_entries.len() as i16)]
    effect_model_count: i16,

    #[br(temp)]
    #[bw(calc = effect_group_elements.len() as i16)]
    effect_group_element_count: i16,

    #[br(temp)]
    #[bw(calc = self.calculate_resource_alignment_factor() as i16)]
    resource_alignment_factor: i16,

    /// Collection of effect handles.
    #[br(count = effect_handle_count)]
    pub effect_handles: Vec<EffectHandle>,

    /// Collection of effect group elements.
    #[br(count = effect_group_element_count)]
    pub effect_group_elements: Vec<EffectGroupElement>,

    /// Collection of effect model entries.
    #[br(count = effect_model_count)]
    pub effect_model_entries: Vec<EffectModelEntry>,

    /// Collection of effect handle names.
    #[br(count = effect_handle_count)]
    pub effect_handle_names: Vec<CString>,

    /// Collection of effect model names.
    #[br(count = effect_model_count)]
    pub effect_model_names: Vec<CString>,

    /// Collection of parent joint names to emitter sets in effect group elements.
    #[br(count = effect_group_element_count)]
    pub parent_joint_names: Vec<CString>,

    /// Data buffer for the contained file resource.
    #[br(parse_with = binrw::helpers::until_eof)]
    #[brw(if(resource_alignment_factor != -1), align_before = Self::calculate_resource_alignment(resource_alignment_factor))]
    #[cfg_attr(feature = "serde", serde(skip))]
    pub resource_data: Option<Vec<u8>>,
}

impl EffFile {
    const RESOURCE_ALIGNMENT_COEFFICIENT: usize = 0x1000;

    /// Reads the data from the given file path.
    pub fn from_file<P: AsRef<Path>>(path: P) -> BinResult<Self> {
        let mut file = Cursor::new(fs::read(path)?);
        let eff = file.read_le::<Self>()?;

        Ok(eff)
    }

    /// Reads the data from the given reader.
    pub fn read<R: Read + Seek>(reader: &mut R) -> BinResult<Self> {
        let eff = reader.read_le::<Self>()?;

        Ok(eff)
    }

    /// Writes the data to the given writer.
    pub fn write<W: Write + Seek>(&self, writer: &mut W) -> BinResult<()> {
        self.write_le(writer)
    }

    /// Writes the data to the given file path.
    pub fn write_to_file<P: AsRef<Path>>(&self, path: P) -> BinResult<()> {
        let mut cursor = Cursor::new(Vec::new());

        self.write_le(&mut cursor)?;
        fs::write(path, cursor.get_mut())?;

        Ok(())
    }

    /// Writes the data from the resource data buffer to the given file path.
    pub fn write_resource_to_file<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        if let Some(resource_data) = &self.resource_data {
            fs::write(path, resource_data)?;
        }

        Ok(())
    }

    fn calculate_resource_alignment_factor(&self) -> usize {
        if self.resource_data.is_none() {
            return usize::MAX;
        }

        let mut size = 0x10;

        size += self.effect_handles.len() * mem::size_of::<EffectHandle>();
        size += self.effect_group_elements.len() * mem::size_of::<EffectGroupElement>();
        size += self.effect_model_entries.len() * mem::size_of::<EffectModelEntry>();

        for name in self.effect_handle_names.iter() {
            size += name.len() + 1;
        }

        for name in self.effect_model_names.iter() {
            size += name.len() + 1;
        }

        for name in self.parent_joint_names.iter() {
            size += name.len() + 1;
        }

        ((size + Self::RESOURCE_ALIGNMENT_COEFFICIENT) & !0xFFF)
            >> Self::RESOURCE_ALIGNMENT_COEFFICIENT.ilog2()
    }

    fn calculate_resource_alignment(factor: i16) -> usize {
        if factor < mem::align_of::<u8>() as i16 {
            return mem::align_of::<u8>();
        }

        factor as usize * Self::RESOURCE_ALIGNMENT_COEFFICIENT
    }
}

/// The data associated with an effect handle.
#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct EffectHandle {
    /// Flags representing the attributes of an effect.
    pub flags: EffectHandleFlags,

    /// Positive index to the emitter set.
    pub emitter_set_handle: i32,

    /// Positive index to the effect model entry.
    pub effect_model_entry_handle: i32,

    /// Positive index to the first element in the effect group.
    pub effect_group_element_start: i16,

    /// Number of elements in the effect group.
    pub effect_group_element_count: i16,
}

/// Flags for an [`EffectHandle`] representing the attributes of an effect.
#[bitfield(bits = 32)]
#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[br(map = Self::from_bytes)]
#[bw(map = |b: &EffectHandleFlags| b.into_bytes())]
pub struct EffectHandleFlags {
    pub unk_01: bool,
    pub unk_02: bool,
    pub unk_03: bool,
    pub unk_04: bool,
    pub unk_05: bool,
    pub unk_06: bool,
    pub unk_07: bool,
    #[skip]
    __: bool,
    pub unk_09: bool,
    pub unk_10: bool,
    #[skip]
    __: B2,
    pub unk_13: bool,
    pub unk_14: bool,
    pub unk_15: bool,
    pub unk_16: bool,
    pub unk_17: bool,
    #[skip]
    __: bool,
    pub hit_effect: bool,
    pub unk_20: bool,
    pub unk_21: bool,
    #[skip]
    __: bool,
    pub unk_23: bool,
    pub update_always: bool,
    pub unk_25: bool,
    pub unk_26: bool,
    #[skip]
    __: B2,
    pub unk_29: bool,
    pub unk_30: bool,
    pub unk_31: bool,
    pub unk_32: bool,
}

/// The data associated with an effect group element.
#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct EffectGroupElement {
    /// Frame to request the emitter set on.
    pub emitter_set_start_frame: i16,

    /// Positive index to the emitter set.
    pub emitter_set_handle: i16,
}

/// The data associated with an effect model entry.
#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct EffectModelEntry {
    // TODO: Determine the purpose of this field. Only equal to zero or one, and only checked if zero.
    pub unk: i8,
}
