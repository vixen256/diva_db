#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use binary_parser::*;
use std::collections::*;
use std::fs::File;
use std::io::SeekFrom;
use std::io::Write;
use std::path::Path;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Ord, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "pyo3", cfg_eval::cfg_eval, pyclass)]
pub struct ObjDb {
	#[cfg_attr(feature = "pyo3", pyo3(get, set))]
	pub sets: BTreeMap<u32, ObjSet>,
}

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Ord, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "pyo3", cfg_eval::cfg_eval, pyclass)]
pub struct ObjSet {
	#[cfg_attr(feature = "pyo3", pyo3(get, set))]
	pub name: String,
	#[cfg_attr(feature = "pyo3", pyo3(get, set))]
	pub object_filename: String,
	#[cfg_attr(feature = "pyo3", pyo3(get, set))]
	pub texture_filename: String,
	#[cfg_attr(feature = "pyo3", pyo3(get, set))]
	pub archive_filename: String,
	#[cfg_attr(feature = "pyo3", pyo3(get, set))]
	pub objects: BTreeMap<u16, String>,
}

impl ObjDb {
	pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
		let mut reader = BinaryParser::from_file(path)?;
		Self::from_parser(&mut reader)
	}

	fn from_parser(reader: &mut BinaryParser) -> Result<Self> {
		let set_count = reader.read_u32()?;
		reader.seek(SeekFrom::Current(4))?;
		let mut sets = BTreeMap::new();
		reader.read_pointer(|reader| {
			for _ in 0..set_count {
				let name = reader.read_null_string_pointer()?;
				let id = reader.read_u32()?;
				let object_filename = reader.read_null_string_pointer()?;
				let texture_filename = reader.read_null_string_pointer()?;
				let archive_filename = reader.read_null_string_pointer()?;
				reader.seek(SeekFrom::Current(16))?;

				sets.insert(
					id,
					ObjSet {
						name,
						object_filename,
						texture_filename,
						archive_filename,
						objects: BTreeMap::new(),
					},
				);
			}

			Ok(())
		})?;

		let object_count = reader.read_u32()?;
		reader.read_pointer(|reader| {
			for _ in 0..object_count {
				let id = reader.read_u16()?;
				let set_id = reader.read_u16()?;
				let name = reader.read_null_string_pointer()?;

				let Some(set) = sets.get_mut(&(set_id as u32)) else {
					continue;
				};
				set.objects.insert(id, name);
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

		let max_id = self
			.sets
			.iter()
			.map(|(_, set)| set.objects.iter().map(|(id, _)| *id).max().unwrap_or(0))
			.max()
			.unwrap_or(0);

		writer.write_u32(self.sets.len() as u32)?;
		writer.write_u32(max_id as u32)?;

		writer.write_pointer(move |writer| {
			for (id, set) in self.sets.iter() {
				writer.write_null_string_pointer(&set.name)?;
				writer.write_u32(*id)?;
				writer.write_null_string_pointer(&set.object_filename)?;
				writer.write_null_string_pointer(&set.texture_filename)?;
				writer.write_null_string_pointer(&set.archive_filename)?;
				writer.write_u32_array(&[0, 0, 0, 0])?;
			}
			writer.align_write(16)?;

			Ok(())
		})?;

		writer.write_u32(
			self.sets
				.iter()
				.map(|(_, set)| set.objects.len() as u32)
				.sum(),
		)?;

		writer.write_pointer(move |writer| {
			writer.align_write(16)?;
			for (set_id, set) in self.sets.iter() {
				for (id, name) in set.objects.iter() {
					writer.write_u16(*id)?;
					writer.write_u16(*set_id as u16)?;
					writer.write_null_string_pointer(&name)?;
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
