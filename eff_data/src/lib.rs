//! # eff_data
//!
//! eff_data is a high-level library built off [eff_lib](https://crates.io/crates/eff_lib) for reading and writing EFF files from Super Smash Bros. Ultimate.
use std::{
    fs,
    io::{self, Read, Seek, Write},
    path::Path,
};

use binrw::BinResult;
use eff_lib::{EffFile, EffectGroupElement, EffectHandle, EffectHandleFlags, EffectModelEntry};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// The data associated with an [`EffFile`].
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct EffData {
    /// Collection of effect handles.
    pub effect_handles: Vec<EffectHandleData>,

    /// Collection of effect model entries.
    pub effect_model_entries: Vec<EffectModelEntryData>,

    /// Data buffer for the contained file resource.
    #[cfg_attr(feature = "serde", serde(skip))]
    pub resource_data: Option<Vec<u8>>,
}

impl EffData {
    /// Reads the data from the given file path.
    pub fn from_file<P: AsRef<Path>>(path: P) -> BinResult<Self> {
        Ok(EffFile::from_file(path)?.into())
    }

    /// Reads the data from the given reader.
    pub fn read<R: Read + Seek>(reader: &mut R) -> BinResult<Self> {
        Ok(EffFile::read(reader)?.into())
    }

    /// Writes the data to the given writer.
    pub fn write<W: Write + Seek>(&self, writer: &mut W) -> BinResult<()> {
        EffFile::from(self).write(writer)
    }

    /// Writes the data to the given file path.
    pub fn write_to_file<P: AsRef<Path>>(&self, path: P) -> BinResult<()> {
        EffFile::from(self).write_to_file(path)
    }

    /// Writes the data from the resource data buffer to the given file path.
    pub fn write_resource_to_file<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        if let Some(resource_data) = &self.resource_data {
            fs::write(path, resource_data)?;
        }

        Ok(())
    }
}

/// The data associated with an [`EffectHandle`].
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct EffectHandleData {
    /// Name of the effect handle.
    pub name: String,

    /// Flags representing the attributes of an effect.
    pub flags: EffectHandleDataFlags,

    /// Positive index to the emitter set.
    pub emitter_set_handle: i32,

    /// Name of the effect model.
    pub effect_model_name: String,

    /// Collection of effect group elements.
    pub effect_group: Vec<EffectGroupElementData>,
}

/// Flags for an [`EffectHandleData`] representing the attributes of an effect.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct EffectHandleDataFlags {
    pub unk_01: bool,
    pub unk_02: bool,
    pub unk_03: bool,
    pub unk_04: bool,
    pub unk_05: bool,
    pub unk_06: bool,
    pub unk_07: bool,
    // pub unk_08: bool,
    pub unk_09: bool,
    pub unk_10: bool,
    // pub unk_11: bool,
    // pub unk_12: bool,
    pub unk_13: bool,
    pub unk_14: bool,
    pub unk_15: bool,
    pub unk_16: bool,
    pub unk_17: bool,
    // pub unk_18: bool,
    pub hit_effect: bool,
    pub unk_20: bool,
    pub unk_21: bool,
    // pub unk_22: bool,
    pub unk_23: bool,
    pub update_always: bool,
    pub unk_25: bool,
    pub unk_26: bool,
    // pub unk_27: bool,
    // pub unk_28: bool,
    pub unk_29: bool,
    pub unk_30: bool,
    pub unk_31: bool,
    pub unk_32: bool,
}

/// The data associated with an [`EffectGroupElement`].
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct EffectGroupElementData {
    /// Frame to request the emitter set on.
    pub emitter_set_start_frame: i16,

    /// Positive index to the emitter set.
    pub emitter_set_handle: i16,

    /// Joint name to parent the emitter set to.
    pub parent_joint_name: String,
}

/// The data associated with an [`EffectModelEntry`].
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct EffectModelEntryData {
    /// Name of the effect model.
    pub name: String,

    // TODO: Determine the purpose of this field.
    pub unk: i8,
}

impl From<EffFile> for EffData {
    fn from(value: EffFile) -> Self {
        Self::from(&value)
    }
}

impl From<&EffFile> for EffData {
    fn from(value: &EffFile) -> Self {
        Self {
            effect_handles: value
                .effect_handles
                .iter()
                .zip(value.effect_handle_names.iter())
                .map(|(handle, name)| EffectHandleData {
                    name: name.to_string().unwrap(),
                    flags: EffectHandleDataFlags {
                        unk_01: handle.flags.unk_01(),
                        unk_02: handle.flags.unk_02(),
                        unk_03: handle.flags.unk_03(),
                        unk_04: handle.flags.unk_04(),
                        unk_05: handle.flags.unk_05(),
                        unk_06: handle.flags.unk_06(),
                        unk_07: handle.flags.unk_07(),
                        // unk_08: handle.flags.unk_08(),
                        unk_09: handle.flags.unk_09(),
                        unk_10: handle.flags.unk_10(),
                        // unk_11: handle.flags.unk_11(),
                        // unk_12: handle.flags.unk_12(),
                        unk_13: handle.flags.unk_13(),
                        unk_14: handle.flags.unk_14(),
                        unk_15: handle.flags.unk_15(),
                        unk_16: handle.flags.unk_16(),
                        unk_17: handle.flags.unk_17(),
                        // unk_18: handle.flags.unk_18(),
                        hit_effect: handle.flags.hit_effect(),
                        unk_20: handle.flags.unk_20(),
                        unk_21: handle.flags.unk_21(),
                        // unk_22: handle.flags.unk_22(),
                        unk_23: handle.flags.unk_23(),
                        update_always: handle.flags.update_always(),
                        unk_25: handle.flags.unk_25(),
                        unk_26: handle.flags.unk_26(),
                        // unk_27: handle.flags.unk_27(),
                        // unk_28: handle.flags.unk_28(),
                        unk_29: handle.flags.unk_29(),
                        unk_30: handle.flags.unk_30(),
                        unk_31: handle.flags.unk_31(),
                        unk_32: handle.flags.unk_32(),
                    },
                    emitter_set_handle: handle.emitter_set_handle,
                    effect_model_name: if handle.effect_model_entry_handle != 0 {
                        value.effect_model_names[handle.effect_model_entry_handle as usize - 1]
                            .to_string()
                            .unwrap()
                    } else {
                        String::new()
                    },
                    effect_group: if handle.effect_group_element_count != 0 {
                        let start = handle.effect_group_element_start as usize - 1;
                        let end = start + handle.effect_group_element_count as usize;

                        value.effect_group_elements[start..end]
                            .iter()
                            .zip(value.parent_joint_names[start..end].iter())
                            .map(|(element, parent_joint_name)| EffectGroupElementData {
                                emitter_set_start_frame: element.emitter_set_start_frame,
                                emitter_set_handle: element.emitter_set_handle,
                                parent_joint_name: parent_joint_name.to_string().unwrap(),
                            })
                            .collect()
                    } else {
                        Vec::new()
                    },
                })
                .collect(),
            effect_model_entries: value
                .effect_model_entries
                .iter()
                .zip(value.effect_model_names.iter())
                .map(|(model, name)| EffectModelEntryData {
                    name: name.to_string().unwrap(),
                    unk: model.unk,
                })
                .collect(),
            resource_data: value.resource_data.clone(),
        }
    }
}

impl From<EffData> for EffFile {
    fn from(value: EffData) -> Self {
        Self::from(&value)
    }
}

impl From<&EffData> for EffFile {
    fn from(value: &EffData) -> Self {
        let mut effect_group_start_index: i16 = 0;

        Self {
            effect_handles: value
                .effect_handles
                .iter()
                .map(|handle| EffectHandle {
                    flags: EffectHandleFlags::new()
                        .with_unk_01(handle.flags.unk_01)
                        .with_unk_02(handle.flags.unk_02)
                        .with_unk_03(handle.flags.unk_03)
                        .with_unk_04(handle.flags.unk_04)
                        .with_unk_05(handle.flags.unk_05)
                        .with_unk_06(handle.flags.unk_06)
                        .with_unk_07(handle.flags.unk_07)
                        // .with_unk_08(handle.flags.unk_08)
                        .with_unk_09(handle.flags.unk_09)
                        .with_unk_10(handle.flags.unk_10)
                        // .with_unk_11(handle.flags.unk_11)
                        // .with_unk_12(handle.flags.unk_12)
                        .with_unk_13(handle.flags.unk_13)
                        .with_unk_14(handle.flags.unk_14)
                        .with_unk_15(handle.flags.unk_15)
                        .with_unk_16(handle.flags.unk_16)
                        .with_unk_17(handle.flags.unk_17)
                        // .with_unk_18(handle.flags.unk_18)
                        .with_hit_effect(handle.flags.hit_effect)
                        .with_unk_20(handle.flags.unk_20)
                        .with_unk_21(handle.flags.unk_21)
                        // .with_unk_22(handle.flags.unk_22)
                        .with_unk_23(handle.flags.unk_23)
                        .with_update_always(handle.flags.update_always)
                        .with_unk_25(handle.flags.unk_25)
                        .with_unk_26(handle.flags.unk_26)
                        // .with_unk_27(handle.flags.unk_27)
                        // .with_unk_28(handle.flags.unk_28)
                        .with_unk_29(handle.flags.unk_29)
                        .with_unk_30(handle.flags.unk_30)
                        .with_unk_31(handle.flags.unk_31)
                        .with_unk_32(handle.flags.unk_32),
                    emitter_set_handle: handle.emitter_set_handle,
                    effect_model_entry_handle: value
                        .effect_model_entries
                        .iter()
                        .position(|model| model.name == handle.effect_model_name)
                        .map_or(0, |i| i + 1) as i32,
                    effect_group_element_start: if !handle.effect_group.is_empty() {
                        let effect_group_element_start = effect_group_start_index + 1;

                        effect_group_start_index += handle.effect_group.len() as i16;

                        effect_group_element_start
                    } else {
                        0
                    },
                    effect_group_element_count: handle.effect_group.len() as i16,
                })
                .collect(),
            effect_group_elements: value
                .effect_handles
                .iter()
                .flat_map(|handle| {
                    handle
                        .effect_group
                        .iter()
                        .map(|element| EffectGroupElement {
                            emitter_set_start_frame: element.emitter_set_start_frame,
                            emitter_set_handle: element.emitter_set_handle,
                        })
                })
                .collect(),
            effect_model_entries: value
                .effect_model_entries
                .iter()
                .map(|model| EffectModelEntry { unk: model.unk })
                .collect(),
            effect_handle_names: value
                .effect_handles
                .iter()
                .map(|handle| handle.name.clone().into())
                .collect(),
            effect_model_names: value
                .effect_model_entries
                .iter()
                .map(|model| model.name.clone().into())
                .collect(),
            parent_joint_names: value
                .effect_handles
                .iter()
                .flat_map(|handle| {
                    handle
                        .effect_group
                        .iter()
                        .map(|element| element.parent_joint_name.clone().into())
                })
                .collect(),
            resource_data: value.resource_data.clone(),
        }
    }
}
