# commandrs

Create robust command line tools in Rust with *commandrs*.

Here is a small list of features that commandrs offers:

- Required and optional flags
- Automatic help texts
- Type CLI arguments
- Flag and CLI descriptions

# Example

Here is an example of how to use commandrs's main struct [Program](./src/program.rs) to create some sort of config:

```rust
use commandrs::error::ProgramError;
use commandrs::Program;

struct Config {
    port: u16,
    use_tls: bool,
}

impl Config {
    pub fn new_from_args() -> Result<Config, ProgramError> {
        let program = Program::new()
            .with_description("An HTTP server")
            .with_required_flag::<u16>("port", "Port number")?
            .with_optional_flag::<bool>("use-tls", false, "TLS PLS?")?
            .parse_from_str_arr(&["--port", "8080"])?;

        Ok(Config {
            port: program.get::<u16>("port")?,
            use_tls: program.get::<bool>("use-tls")?
        })
    }
}

Config::new_from_args().expect("Invalid program args");
```
