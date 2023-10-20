use pyo3::prelude::*;

pub trait Model {
	fn set_variables(&mut self, var: &Vec<f64>);
	fn get_results(&self) -> Vec<f64>;
	fn compute(&mut self);
}

pub fn solve<T: Model>(model: &mut T) {
	println!("Magic solver that mutates the model into a resolved state");
}

#[pyfunction]
#[pyo3(name = "solve")]
pub fn solve_wrapper(model: &mut UserModel) {
	solve(model);
}

#[pyclass]
pub struct UserModel {
	model: Py<PyAny>,
}

#[pymodule]
fn trait_exposure(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
	m.add_class::<UserModel>()?;
	m.add_function(wrap_pyfunction!(solve_wrapper, m)?)?;
	Ok(())
}

#[pymethods]
impl UserModel {
	#[new]
	pub fn new(model: Py<PyAny>) -> Self {
		UserModel { model }
	}

	pub fn set_variables(&mut self, var: Vec<f64>) {
		println!("Set variables from Python calling Rust");
		Model::set_variables(self, &var)
	}

	pub fn get_results(&mut self) -> Vec<f64> {
		println!("Get results from Python calling Rust");
		Model::get_results(self)
	}

	pub fn compute(&mut self) {
		Model::compute(self)
	}
}

impl Model for UserModel {
	fn set_variables(&mut self, var: &Vec<f64>) {
		println!("Rust calling Python to set the variables");
		Python::with_gil(|py| {
			let values: Vec<f64> = var.clone();
			let list: PyObject = values.into_py(py);
			let py_model = self.model.as_ref(py);
			py_model.call_method("set_variables", (list,), None).unwrap();
		})
	}

	fn get_results(&self) -> Vec<f64> {
		println!("Get results from Rust calling Python");
		Python::with_gil(|py| {
			let py_result: &PyAny =
				self.model.as_ref(py).call_method("get_results", (), None).unwrap();

			if py_result.get_type().name().unwrap() != "list" {
				panic!(
					"Expected a list for the get_results() method signature, got {}",
					py_result.get_type().name().unwrap()
				);
			}
			py_result.extract()
		})
		.unwrap()
	}

	fn compute(&mut self) {
		println!("Rust calling Python to perform the computation");
		Python::with_gil(|py| {
			self.model.as_ref(py).call_method("compute", (), None).unwrap();
		})
	}
}
