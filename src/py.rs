use crate::aet::*;
use crate::obj::*;
use crate::spr::*;
use crate::tex::*;
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;

#[pymodule]
fn diva_db(m: &Bound<'_, PyModule>) -> PyResult<()> {
	m.add_class::<AetDb>()?;
	m.add_class::<AetSet>()?;
	m.add_class::<AetScene>()?;
	m.add_class::<ObjDb>()?;
	m.add_class::<ObjSet>()?;
	m.add_class::<SprDb>()?;
	m.add_class::<SprSet>()?;
	m.add_class::<SprEntry>()?;
	m.add_class::<TexDb>()?;
	Ok(())
}

#[pymethods]
impl AetDb {
	#[staticmethod]
	fn read(path: String) -> PyResult<Self> {
		Self::from_file(path).map_err(|e| PyErr::new::<PyTypeError, String>(e.to_string()))
	}

	fn write(&self, path: String) -> PyResult<()> {
		self.write_file(path)
			.map_err(|e| PyErr::new::<PyTypeError, String>(e.to_string()))
	}
}

#[pymethods]
impl AetSet {
	fn __str__(&self) -> String {
		self.name.clone()
	}

	fn __repr__(&self) -> String {
		self.name.clone()
	}
}

#[pymethods]
impl AetScene {
	fn __str__(&self) -> String {
		self.name.clone()
	}

	fn __repr__(&self) -> String {
		self.name.clone()
	}
}

#[pymethods]
impl ObjDb {
	#[staticmethod]
	fn read(path: String) -> PyResult<Self> {
		Self::from_file(path).map_err(|e| PyErr::new::<PyTypeError, String>(e.to_string()))
	}

	fn write(&self, path: String) -> PyResult<()> {
		self.write_file(path)
			.map_err(|e| PyErr::new::<PyTypeError, String>(e.to_string()))
	}
}

#[pymethods]
impl ObjSet {
	fn __str__(&self) -> String {
		self.name.clone()
	}

	fn __repr__(&self) -> String {
		self.name.clone()
	}
}

#[pymethods]
impl SprDb {
	#[staticmethod]
	fn read(path: String) -> PyResult<Self> {
		Self::from_file(path).map_err(|e| PyErr::new::<PyTypeError, String>(e.to_string()))
	}

	fn write(&self, path: String) -> PyResult<()> {
		self.write_file(path)
			.map_err(|e| PyErr::new::<PyTypeError, String>(e.to_string()))
	}
}

#[pymethods]
impl SprSet {
	fn __str__(&self) -> String {
		self.name.clone()
	}

	fn __repr__(&self) -> String {
		self.name.clone()
	}
}

#[pymethods]
impl SprEntry {
	fn __str__(&self) -> String {
		self.name.clone()
	}

	fn __repr__(&self) -> String {
		self.name.clone()
	}
}

#[pymethods]
impl TexDb {
	#[staticmethod]
	fn read(path: String) -> PyResult<Self> {
		Self::from_file(path).map_err(|e| PyErr::new::<PyTypeError, String>(e.to_string()))
	}

	fn write(&self, path: String) -> PyResult<()> {
		self.write_file(path)
			.map_err(|e| PyErr::new::<PyTypeError, String>(e.to_string()))
	}
}
