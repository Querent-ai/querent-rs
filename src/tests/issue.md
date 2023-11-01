# Note to self

I can't run cargo test; or I can't build in a Cargo workspace: I'm having linker issues like "Symbol not found" or "Undefined reference to _PyExc_SystemError"!
Currently, #340 causes cargo test to fail with linking errors when the extension-module feature is activated. Linking errors can also happen when building in a cargo workspace where a different crate also uses PyO3 (see #2521). For now, there are three ways we can work around these issues.

Make the extension-module feature optional. Build with maturin develop --features "extension-module"
[dependencies.pyo3]
version = "0.20.0"

[features]
extension-module = ["pyo3/extension-module"]
Make the extension-module feature optional and default. Run tests with cargo test --no-default-features:
[dependencies.pyo3]
version = "0.20.0"

[features]
extension-module = ["pyo3/extension-module"]
default = ["extension-module"]
If you are using a pyproject.toml file to control maturin settings, add the following section:
[tool.maturin]
features = ["pyo3/extension-module"]

# Or for maturin 0.12

# cargo-extra-args = ["--features", "pyo3/extension-module"]
