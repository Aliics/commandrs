use std::any::{type_name, TypeId};
use std::fmt::Display;
use std::str::FromStr;

use crate::error::ProgramError;
use crate::flag::{Flag, FlagValue};

#[derive(PartialEq, Debug)]
pub struct Program<'a> {
    pub(crate) description: &'a str,
    pub(crate) flags: Vec<Flag<'a>>,
    pub(crate) flag_defaults: Vec<FlagValue<'a>>,
    pub(crate) flag_values: Vec<FlagValue<'a>>,
}

impl<'a> Default for Program<'a> {
    fn default() -> Program<'a> {
        Program {
            description: "",
            flags: vec![],
            flag_defaults: vec![],
            flag_values: vec![],
        }
    }
}

impl<'a> Program<'a> {
    /// This is just an alias for `Program::default`.
    pub fn new() -> Program<'a> {
        Program::default()
    }

    pub fn with_description(mut self, desc: &'a str) -> Program {
        self.description = desc;
        self
    }

    pub fn with_optional_flag<T>(
        mut self,
        name: &'a str,
        default: T,
    ) -> Result<Program, ProgramError>
    where
        T: Display + 'static,
    {
        self = self.add_flag::<T>(name, false)?;
        self.flag_defaults.push(FlagValue {
            name,
            str_value: default.to_string(),
        });
        Ok(self)
    }

    pub fn with_required_flag<T: 'static>(self, name: &'a str) -> Result<Program, ProgramError> {
        self.add_flag::<T>(name, true)
    }

    pub fn get<T>(&self, name: &'a str) -> Result<T, ProgramError>
    where
        T: Display + FromStr + 'static,
    {
        match self.flag_values.iter().find(|fv| fv.name == name) {
            Some(flag_value) => flag_value.str_value.parse::<T>().map_err(|_| {
                let type_name = type_name::<T>();
                ProgramError::FailedToParseFlagValue { name, type_name }
            }),
            None => Err(ProgramError::NoSuchFlagExistsWithName { name }),
        }
    }

    pub fn get_string(&self, name: &'a str) -> Result<String, ProgramError> {
        match self.flag_values.iter().find(|fv| fv.name == name) {
            Some(flag_value) => Ok(flag_value.str_value.to_string()),
            None => Err(ProgramError::NoSuchFlagExistsWithName { name }),
        }
    }

    fn add_flag<T: 'static>(
        mut self,
        name: &'a str,
        is_required: bool,
    ) -> Result<Program, ProgramError> {
        let already_has_flag_with_name = self.flags.iter().any(|f| f.name == name);
        if already_has_flag_with_name {
            // Flag names cannot be duplicate, if they are then there would be no way to parse the
            // arguments on the command line and understand which flag we want.
            return Err(ProgramError::FlagAlreadyExistsWithName { name });
        }

        let type_id = TypeId::of::<T>();
        self.flags.push(Flag {
            name,
            type_id,
            is_required,
        });
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_add_description_when_using_with_description() {
        let expected = Program {
            description: "A very cool test program",
            flags: vec![],
            flag_defaults: vec![],
            flag_values: vec![],
        };

        let builder = Program::default().with_description("A very cool test program");

        assert_eq!(expected, builder);
    }

    #[test]
    fn should_add_optional_flags_when_calling_with_optional_flag_multiple_times() {
        let expected = Program {
            description: "",
            flags: vec![
                Flag {
                    name: "flag0",
                    type_id: TypeId::of::<bool>(),
                    is_required: false,
                },
                Flag {
                    name: "flag1",
                    type_id: TypeId::of::<&str>(),
                    is_required: false,
                },
            ],
            flag_defaults: vec![
                FlagValue {
                    name: "flag0",
                    str_value: "false".to_string(),
                },
                FlagValue {
                    name: "flag1",
                    str_value: "lol".to_string(),
                },
            ],
            flag_values: vec![],
        };

        let program = Program::new()
            .with_optional_flag("flag0", false)
            .unwrap()
            .with_optional_flag("flag1", "lol")
            .unwrap();

        assert_eq!(expected, program);
    }

    #[test]
    fn should_add_required_flags_when_calling_with_required_flag_multiple_times() {
        let expected = Program {
            description: "",
            flags: vec![
                Flag {
                    name: "flag0",
                    type_id: TypeId::of::<bool>(),
                    is_required: true,
                },
                Flag {
                    name: "flag1",
                    type_id: TypeId::of::<&str>(),
                    is_required: true,
                },
            ],
            flag_defaults: vec![],
            flag_values: vec![],
        };

        let program = Program::new()
            .with_required_flag::<bool>("flag0")
            .unwrap()
            .with_required_flag::<&str>("flag1")
            .unwrap();

        assert_eq!(expected, program);
    }

    #[test]
    fn should_not_be_able_to_add_flags_with_the_same_name() {
        let err = Program::new()
            .with_required_flag::<bool>("oh-noes")
            .unwrap()
            .with_required_flag::<&str>("oh-noes")
            .unwrap_err();

        assert_eq!(
            ProgramError::FlagAlreadyExistsWithName { name: "oh-noes" },
            err
        );
    }
}
