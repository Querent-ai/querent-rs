use pyo3::{
	prelude::*,
	types::{IntoPyDict, PyDict, PyTuple},
};

pub struct Process<'a> {
	py: Python<'a>,
	py_process: &'a PyAny,
	py_manager: &'a PyAny,

	pool: Vec<&'a PyAny>,
}

impl<'a> Process<'a> {
	pub fn new(py: Python<'a>) -> PyResult<Self> {
		let py_multiprocessing = py.import("multiprocessing")?;
		let py_process = py_multiprocessing.getattr("Process")?; // Adjusted
		let py_manager = py_multiprocessing.getattr("Manager")?.call0()?; // Adjusted

		Ok(Self { py, py_process, py_manager, pool: vec![] })
	}

	pub fn is_running(&self) -> bool {
		!self.pool.is_empty()
	}

	pub fn spawn(
		&mut self,
		f: impl IntoPy<Py<PyAny>>,
		args: impl IntoPy<Py<PyTuple>>,
		kwargs: Option<&PyDict>,
	) -> PyResult<()> {
		let f = f.into_py(self.py);
		let f_args = args.into_py(self.py).into_py(self.py);
		let f_kwargs = kwargs.or_else(|| Some(PyDict::new(self.py))).into_py(self.py);

		let kwargs = [("target", f), ("args", f_args), ("kwargs", f_kwargs)].into_py_dict(self.py);

		let p = self.py_process.call((), Some(kwargs))?;
		p.call_method0("start")?;
		self.pool.push(p);
		Ok(())
	}

	pub fn spawn_mut(
		&mut self,
		f: impl IntoPy<Py<PyAny>>,
		args: impl IntoPy<Py<PyTuple>>,
		kwargs: Option<&PyDict>,
	) -> PyResult<(Py<PyAny>, Py<PyAny>)> {
		let f = f.into_py(self.py);
		let f_args: Py<PyTuple> = args.into_py(self.py);
		let f_kwargs: Option<&PyDict> = kwargs.or_else(|| Some(PyDict::new(self.py)));

		let f_args = self.py_manager.call_method1("list", (f_args,))?;
		let f_kwargs = self.py_manager.call_method1("dict", (f_kwargs,))?;

		let kwargs =
			[("target", f), ("args", (f_args, f_kwargs).into_py(self.py))].into_py_dict(self.py);

		let p = self.py_process.call((), Some(kwargs))?;
		p.call_method0("start")?;
		self.pool.push(p);
		Ok((f_args.into_py(self.py), f_kwargs.into_py(self.py)))
	}

	pub fn join(&mut self) -> PyResult<()> {
		for p in &self.pool {
			p.call_method0("join")?;
		}
		self.pool.clear();
		Ok(())
	}
}

impl<'a> Drop for Process<'a> {
	fn drop(&mut self) {
		Python::with_gil(|py| -> PyResult<()> {
			self.join()?; // Ensure subprocesses are joined
			self.py_manager.call_method0("shutdown")?; // Correctly shutdown the manager
			py.import("threading")?.getattr("_shutdown")?.call0()?; // Correctly shutdown threading
			Ok(())
		})
		.map_err(|e| {
			Python::with_gil(|py| {
				e.print_and_set_sys_last_vars(py);
			})
		})
		.unwrap()
	}
}
