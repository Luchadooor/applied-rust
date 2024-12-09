//! This module contains the configuration options for the application.
//! # Examples:
//! ```
//! use cli_utils::config::Logging;
//! let config = Logging::new();
//! ```
//! 

/// Level of the log:
/// # Example:
///```
/// use cli_utils::config::*;
/// println!("LogLevel: {:#?}", LogLevel::Info);
///```
#[derive(Debug)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

/// Output of the log:
/// # Example:
///```
/// use cli_utils::config::*;
/// println!("LogLevel: {:#?}", LogOutput::File("El Testo".to_string()));
///```
#[derive(Debug)]
pub enum LogOutput {
    Stdout,
    Stderr,
    File(String),
}

/// This struct contains configuration options for the application.
/// # Examples:
/// ```
/// use cli_utils::config::Logging;
/// let config = Logging::new();
/// ```
/// 
/// Creating a new instance of the Logging struct:
/// ```
/// use cli_utils::config::{Logging, LogLevel, LogOutput};
/// let config = Logging{ enabled: true, level: LogLevel::Info, destination: LogOutput::Stdout };
/// ```
pub struct Logging {
    pub enabled: bool,
    pub level: LogLevel,
    pub destination: LogOutput,   
}

impl Logging {
    pub fn new() -> Self {
        Self {
            enabled: false,
            level: LogLevel::Info,
            destination: LogOutput::Stdout,
        }
    }
}