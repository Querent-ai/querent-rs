/// Module containing configuration-related functionality.
///
/// This module provides structures and builders for configuring the system.
/// It includes the main configuration structure `Config`, as well as a
/// builder for constructing configurations more conveniently.
///
/// # Examples
///
/// Creating a basic configuration:
///
/// ```rust
/// use my_module::config::Config;
///
/// let config = Config::default();
/// ```
///
/// Using a configuration builder for more flexibility:
///
/// ```rust
/// use my_module::config_builder::ConfigBuilder;
///
/// let builder = ConfigBuilder::new()
///     .version(1.0)
///     .querent_id("user123")
///     .querent_name("John Doe")
///     .build();
/// ```
pub mod config;

/// Re-export of the `Config` struct from the `config` module.
pub use config::Config;

/// Module containing a builder for the configuration structures.
///
/// The `ConfigBuilder` allows for a more flexible and fluent way to
/// construct configurations by chaining method calls.
///
/// # Examples
///
/// Using the `ConfigBuilder` to create a configuration:
///
/// ```rust
/// use my_module::config_builder::ConfigBuilder;
///
/// let builder = ConfigBuilder::new()
///     .version(1.0)
///     .querent_id("user123")
///     .querent_name("John Doe");
///
/// let config = builder.build();
/// ```
pub mod config_builder;

/// Re-export of the `ConfigBuilder` struct from the `config_builder` module.
pub use config_builder::*;
