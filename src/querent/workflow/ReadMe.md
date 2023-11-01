# Python runtime for Rust

1. **PyAsyncFun**:
   - `PyAsyncFun` is a struct that holds information about a Python function call. It contains the Python function (`fun`), a list of arguments (`args`), and a callback mechanism (`callback`) for receiving the result asynchronously.

2. **PyAsyncCallback**:
   - `PyAsyncCallback` is an enum that represents different callback mechanisms. In this code, it uses `oneshot::Sender` for sending the result back to the caller.

3. **PyAsyncFunResult**:
   - This enum represents the possible outcomes of calling a Python function asynchronously. It's either a pollable future or an immediate result. In this code, only the future is used.

4. **PyRuntime**:
   - `PyRuntime` is a struct responsible for managing and executing Python functions asynchronously. It contains a `sender` for sending tasks to be executed.

   - `call_async` is a public method to schedule a Python function call. It creates an `oneshot::channel`, sends the task to the internal sender, and waits for the result to be returned asynchronously.

   - `process_coroutines` is a function that processes Python coroutines. It takes a `PyAsyncFun` as input, splits it, and then handles the execution of the Python function.

   - In this code, Python's Global Interpreter Lock (GIL) is acquired and released using `Python::with_gil`. Arguments are converted to Python types, the Python function is called, and if it returns a coroutine, the result is wrapped in a pollable future.

   - If there's an error in processing the task, it returns an error or uses the `PyAsyncCallback` to send an error result back.

   - If the result is a pollable future, it's spawned as a Tokio task that will later resolve the result and send it using the provided `PyAsyncCallback`.

   - `new` is a constructor for creating a new `PyRuntime`. It initializes the Python asyncio event loop and sets up a separate Tokio task to process Python tasks in an infinite loop.

5. **PY_RUNTIME**:
   - `PY_RUNTIME` is an `OnceCell` that holds the instance of `PyRuntime`. It ensures that only one instance of `PyRuntime` is created.

6. **py_runtime**:
   - `py_runtime` is a public function to get the `PyRuntime` instance. If an instance is already created, it returns it. If not, it creates a new one using `OnceCell`.

Usage:

- You can use the `py_runtime` function to obtain a reference to the `PyRuntime` instance. This instance allows you to call Python functions asynchronously by using the `call_async` method.

- To call a Python function asynchronously, provide a Python function object, a list of arguments, and a callback mechanism. The callback can be a channel that receives the result.

- The `PyRuntime` internally manages the execution of Python tasks in a separate thread and processes coroutines using Tokio. The results are sent back asynchronously using the provided callback mechanism.

- Ensure you have the necessary dependencies and initialization for PyO3, Tokio, and asyncio in your Rust project to use this code effectively.
