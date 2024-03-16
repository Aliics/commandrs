use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::error::ProgramError::*;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ProgramError<'a> {
    FlagAlreadyExistsWithName { name: &'a str },
    NoSuchFlagExistsWithName { name: &'a str },
    FailedToParseFlagValue { name: &'a str, type_name: &'a str },
    RequiredArgWasNotGiven { name: &'a str },
}

impl Display for ProgramError<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FlagAlreadyExistsWithName { name } => {
                write!(f, "Flag already exists with name {0}", name)
            }
            NoSuchFlagExistsWithName { name } => {
                write!(f, "No such flag exists with name {0}", name)
            }
            FailedToParseFlagValue { name, type_name } => {
                write!(f, "Could not parse {0} as type of {1}", name, type_name)
            }
            RequiredArgWasNotGiven { name } => {
                write!(f, "Required args was not given with name {0}", name)
            }
        }
    }
}

impl Error for ProgramError<'_> {}
