use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::error::ProgramError::FlagAlreadyExistsWithName;

#[derive(Debug, PartialEq)]
pub enum ProgramError {
    FlagAlreadyExistsWithName { name: String },
}

impl Display for ProgramError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FlagAlreadyExistsWithName { name } => {
                write!(f, "Flag already exists with name {0}", name)
            }
        }
    }
}

impl Error for ProgramError {}
