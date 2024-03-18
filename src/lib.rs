//! `commandrs` is typically used by constructing a `Program` using the various flag registering
//! methods such as `Program::with_description` or `Program::with_optional_flag`.
//!
//! Once your `Program` looks the way you like it, calling `parse`, or an alternative, should parse
//! the command line arguments and allow you to extract the values with `get`, or an alternative.
//!
//! Example usage of how you might want to use commandrs to construct a "config" struct.
//! ```
//! use commandrs::error::ProgramError;
//! use commandrs::Program;
//!
//! struct Config {
//!     port: u16,
//!     use_tls: bool,
//! }
//!
//! impl Config {
//!     pub fn new_from_args() -> Result<Config, ProgramError> {
//!         let program = Program::new()
//!             .with_description("An HTTP server")
//!             .with_required_flag::<u16>("port", "Port number")?
//!             .with_optional_flag::<bool>("use-tls", false, "TLS PLS?")?
//!             .parse_from_str_arr(&["--port", "8080"])?;
//!
//!         Ok(Config {
//!             port: program.get::<u16>("port")?,
//!             use_tls: program.get::<bool>("use-tls")?
//!         })
//!     }
//! }
//!
//! Config::new_from_args().expect("Invalid program args");
//! ```

pub mod error;
pub mod flag;
mod help;
pub mod parser;
pub mod program;

pub use program::Program;
