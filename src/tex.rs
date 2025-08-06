#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use binary_parser::*;
use std::collections::*;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Ord, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct TexDb {
	pub textures: BTreeMap<u32, String>,
}

impl TexDb {
	pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
		let mut reader = BinaryParser::from_file(path)?;
		Self::from_parser(&mut reader)
	}

	fn from_parser(reader: &mut BinaryParser) -> Result<Self> {
		let texture_count = reader.read_u32()?;
		let mut textures = BTreeMap::new();
		reader.read_pointer(|reader| {
			for _ in 0..texture_count {
				let id = reader.read_u32()?;
				let name = reader.read_null_string_pointer()?;

				textures.insert(id, name);
			}

			Ok(())
		})?;

		Ok(Self { textures })
	}

	pub fn write_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
		let parser = self.write_parser()?;
		let mut file = File::create(path)?;
		file.write(&parser.to_buf_const().unwrap())?;
		Ok(())
	}

	fn write_parser(&self) -> Result<BinaryParser<'_>> {
		let mut writer = BinaryParser::new();

		writer.write_u32(self.textures.len() as u32)?;
		writer.write_pointer(move |writer| {
			for (id, name) in self.textures.iter() {
				writer.write_u32(*id)?;
				writer.write_null_string_pointer(&name)?;
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
