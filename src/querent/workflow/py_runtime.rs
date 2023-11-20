use crate::{
	config::Config,
	cross::{CLRepr, CLReprPython},
	querent::errors::QuerentError,
	tokio_runtime,
};
use log::{error, trace};
use once_cell::sync::OnceCell;
use pyo3::{
	prelude::*,
	types::{PyFunction, PyTuple},
};
use std::{fmt::Formatter, future::Future, pin::Pin};
use tokio::sync::oneshot;

#[derive(Debug)]
pub struct PyAsyncFun {
	fun: Py<PyFunction>,
	args: Vec<CLRepr>,
	callback: PyAsyncCallback,
	config: Option<Config>,
}

pub enum PyAsyncCallback {
	Channel(oneshot::Sender<Result<CLRepr, QuerentError>>),
}

impl std::fmt::Debug for PyAsyncCallback {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			PyAsyncCallback::Channel(_) => write!(f, "Channel<hidden>"),
		}
	}
}

impl PyAsyncFun {
	pub fn split(self) -> (Py<PyFunction>, Vec<CLRepr>, PyAsyncCallback, Option<Config>) {
		(self.fun, self.args, self.callback, self.config)
	}
}

enum PyAsyncFunResult {
	Poll(Pin<Box<dyn Future<Output = PyResult<PyObject>> + Send>>),
}

pub struct PyRuntime {
	sender: tokio::sync::mpsc::Sender<PyAsyncFun>,
}

impl PyRuntime {
	pub async fn call_async(
		&self,
		fun: Py<PyFunction>,
		args: Vec<CLRepr>,
		config: Option<Config>,
	) -> Result<CLRepr, QuerentError> {
		let (rx, tx) = oneshot::channel();

		self.sender
			.send(PyAsyncFun { fun, args, callback: PyAsyncCallback::Channel(rx), config })
			.await
			.map_err(|err| {
				QuerentError::internal(format!("Unable to schedule python function call: {}", err))
			})?;

		tx.await?
	}

	fn process_coroutines(task: PyAsyncFun) -> Result<(), QuerentError> {
		let (fun, args, callback, config) = task.split();

		let task_result = Python::with_gil(move |py| -> PyResult<PyAsyncFunResult> {
			let mut args_tuple = Vec::with_capacity(args.len());

			for arg in args {
				args_tuple.push(arg.into_py(py)?);
			}

			if let Some(config) = config {
				args_tuple.push(config.to_object(py));
			}

			let args = PyTuple::new(py, args_tuple);
			let call_res = fun.call1(py, args)?;
			let fut = pyo3_asyncio::tokio::into_future(call_res.as_ref(py))?;
			Ok(PyAsyncFunResult::Poll(Box::pin(fut)))
		});
		let task_result = match task_result {
			Ok(r) => r,
			Err(err) => {
				match callback {
					PyAsyncCallback::Channel(chan) => {
						let send_res = chan
							.send(Err(QuerentError::internal(format!("Python error: {}", err))));
						if send_res.is_err() {
							return Err(QuerentError::internal(
								"Unable to send result back to consumer".to_string(),
							))
						}
					},
				};

				return Ok(())
			},
		};

		match task_result {
			PyAsyncFunResult::Poll(fut) => {
				tokio::spawn(async move {
					let fut_res = fut.await;

					let res = Python::with_gil(move |py| -> Result<CLRepr, PyErr> {
						let res = match fut_res {
							Ok(r) => CLRepr::from_python_ref(r.as_ref(py)),
							Err(err) => Err(err),
						};

						res
					});

					match callback {
						PyAsyncCallback::Channel(chan) => {
							let _ = match res {
								Ok(r) => chan.send(Ok(r)),
								Err(err) => chan.send(Err(QuerentError::internal(format!(
									"Python error: {}",
									err
								)))),
							};
						},
					}
				});
			},
		};

		Ok(())
	}

	pub fn new() -> Self {
		let (sender, mut receiver) = tokio::sync::mpsc::channel::<PyAsyncFun>(1024);

		trace!("New Python runtime");

		std::thread::spawn(|| {
			trace!("Initializing executor in a separate thread");

			std::thread::spawn(|| {
				pyo3_asyncio::tokio::get_runtime()
					.block_on(pyo3_asyncio::tokio::re_exports::pending::<()>())
			});

			let res = Python::with_gil(|py| -> Result<(), PyErr> {
				pyo3_asyncio::tokio::run(py, async move {
					loop {
						if let Some(task) = receiver.recv().await {
							trace!("New task");

							if let Err(err) = Self::process_coroutines(task) {
								error!("Error while processing python task: {:?}", err)
							};
						}
					}
				})
			});
			match res {
				Ok(_) => trace!("Python runtime loop was closed without error"),
				Err(err) => error!("Critical error while processing python call: {}", err),
			}
		});

		Self { sender }
	}
}

static PY_RUNTIME: OnceCell<PyRuntime> = OnceCell::new();

pub fn py_runtime() -> Result<&'static PyRuntime, QuerentError> {
	if let Some(runtime) = PY_RUNTIME.get() {
		Ok(runtime)
	} else {
		let runtime = PyRuntime::new();
		PY_RUNTIME
			.set(runtime)
			.map(|_| PY_RUNTIME.get().unwrap())
			.map_err(|_| QuerentError::internal("Unable to set PyRuntime".to_string()))
	}
}

pub fn call_async(
	fun: Py<PyFunction>,
	args: Vec<CLRepr>,
	config: Option<Config>,
) -> Result<impl Future<Output = Result<CLRepr, QuerentError>>, QuerentError> {
	let runtime = py_runtime()?;
	Ok(runtime.call_async(fun, args, config))
}

pub fn py_runtime_init() -> Result<(), QuerentError> {
	if PY_RUNTIME.get().is_some() {
		return Ok(())
	}

	let runtime = tokio_runtime()?;

	pyo3::prepare_freethreaded_python();

	pyo3_asyncio::tokio::init_with_runtime(runtime)
		.map_err(|_| QuerentError::internal("Unable to initialize Python runtime".to_string()))?;
	if PY_RUNTIME.set(PyRuntime::new()).is_err() {
		Err(QuerentError::internal("Unable to set PyRuntime".to_string()))
	} else {
		Ok(())
	}
}
