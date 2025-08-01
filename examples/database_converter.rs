use clap::Parser;
use diva_db::*;

#[derive(Parser)]
struct Args {
	file: String,
}

fn main() {
	let args = Args::parse();

	let path = std::path::Path::new(&args.file);
	if !path.exists() || !path.is_file() {
		return;
	}

	let filename = path.file_name().unwrap().to_str().unwrap();
	if filename.contains("spr_db") {
		if filename.ends_with(".bin") {
			let spr_db = SprDb::from_file(path).unwrap();
			let spr_db = serde_json::to_string_pretty(&spr_db).unwrap();
			std::fs::write(&path.with_extension("json"), &spr_db).unwrap();
		} else if filename.ends_with(".json") {
			let spr_db = std::fs::read_to_string(path).unwrap();
			let spr_db: SprDb = serde_json::from_str(&spr_db).unwrap();
			spr_db.write_file(&path.with_extension("bin")).unwrap();
		}
	} else if filename.contains("aet_db") {
		if filename.ends_with(".bin") {
			let db = AetDb::from_file(path).unwrap();
			let db = serde_json::to_string_pretty(&db).unwrap();
			std::fs::write(&path.with_extension("json"), &db).unwrap();
		} else if filename.ends_with(".json") {
			let db = std::fs::read_to_string(path).unwrap();
			let db: AetDb = serde_json::from_str(&db).unwrap();
			db.write_file(&path.with_extension("bin")).unwrap();
		}
	} else if filename.contains("obj_db") {
		if filename.ends_with(".bin") {
			let db = ObjDb::from_file(path).unwrap();
			let db = serde_json::to_string_pretty(&db).unwrap();
			std::fs::write(&path.with_extension("json"), &db).unwrap();
		} else if filename.ends_with(".json") {
			let db = std::fs::read_to_string(path).unwrap();
			let db: ObjDb = serde_json::from_str(&db).unwrap();
			db.write_file(&path.with_extension("bin")).unwrap();
		}
	} else if filename.contains("tex_db") {
		if filename.ends_with(".bin") {
			let db = TexDb::from_file(path).unwrap();
			let db = serde_json::to_string_pretty(&db).unwrap();
			std::fs::write(&path.with_extension("json"), &db).unwrap();
		} else if filename.ends_with(".json") {
			let db = std::fs::read_to_string(path).unwrap();
			let db: TexDb = serde_json::from_str(&db).unwrap();
			db.write_file(&path.with_extension("bin")).unwrap();
		}
	}
}
