#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use binary_parser::*;
use std::collections::*;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Ord, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct AetDb {
	pub sets: BTreeMap<u32, AetSet>,
}

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Ord, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct AetSet {
	pub name: String,
	pub filename: String,
	pub sprite_set_id: u32,
	pub scenes: BTreeMap<u32, AetScene>,
}

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Ord, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct AetScene {
	pub name: String,
	pub index: u16,
}

impl AetDb {
	pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
		let mut reader = BinaryParser::from_file(path)?;
		Self::from_parser(&mut reader)
	}

	fn from_parser(reader: &mut BinaryParser) -> Result<Self> {
		let set_count = reader.read_u32()?;
		let mut set_indicies = BTreeMap::new();
		let mut sets = BTreeMap::new();
		reader.read_pointer(|reader| {
			for _ in 0..set_count {
				let id = reader.read_u32()?;
				let name = reader.read_null_string_pointer()?;
				let filename = reader.read_null_string_pointer()?;
				let index = reader.read_u32()?;
				let sprite_set_id = reader.read_u32()?;

				sets.insert(
					id,
					AetSet {
						name,
						filename,
						sprite_set_id,
						scenes: BTreeMap::new(),
					},
				);

				set_indicies.insert(index, id);
			}

			Ok(())
		})?;

		let scene_count = reader.read_u32()?;
		reader.read_pointer(|reader| {
			for _ in 0..scene_count {
				let id = reader.read_u32()?;
				let name = reader.read_null_string_pointer()?;
				let index = reader.read_u16()?;
				let set_index = reader.read_u16()?;

				let Some(set_id) = set_indicies.get(&(set_index as u32 & 0xFFF)) else {
					continue;
				};
				let Some(set) = sets.get_mut(set_id) else {
					continue;
				};

				set.scenes.insert(id, AetScene { name, index });
			}

			Ok(())
		})?;

		Ok(Self { sets })
	}

	pub fn write_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
		let parser = self.write_parser()?;
		let mut file = File::create(path)?;
		file.write(&parser.to_buf_const().unwrap())?;
		Ok(())
	}

	fn write_parser(&self) -> Result<BinaryParser<'_>> {
		let mut writer = BinaryParser::new();

		writer.write_u32(self.sets.len() as u32)?;

		writer.write_pointer(move |writer| {
			for (i, (id, set)) in self.sets.iter().enumerate() {
				writer.write_u32(*id)?;
				writer.write_null_string_pointer(&set.name)?;
				writer.write_null_string_pointer(&set.filename)?;
				writer.write_u32(i as u32)?;
				writer.write_u32(set.sprite_set_id)?;
			}
			writer.align_write(16)?;

			Ok(())
		})?;

		writer.write_u32(
			self.sets
				.iter()
				.map(|(_, set)| set.scenes.len() as u32)
				.sum(),
		)?;

		writer.write_pointer(move |writer| {
			for (i, (_, set)) in self.sets.iter().enumerate() {
				for (id, scene) in set.scenes.iter() {
					writer.write_u32(*id)?;
					writer.write_null_string_pointer(&scene.name)?;
					writer.write_u16(scene.index)?;
					writer.write_u16(i as u16)?;
				}
			}
			writer.align_write(16)?;

			Ok(())
		})?;

		writer.align_write(16)?;
		let mut writer = writer.finish_writes()?;
		writer.align_write(16)?;
		Ok(writer)
	}
}
