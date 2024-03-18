use std::any::{type_name, TypeId};
use std::fmt::Display;
use std::str::FromStr;

use crate::error::ProgramError;
use crate::flag::{Flag, FlagValue};

#[derive(PartialEq, Debug)]
pub struct Program<'a> {
    pub(crate) desc: &'a str,
    pub(crate) flags: Vec<Flag<'a>>,
    pub(crate) flag_defaults: Vec<FlagValue<'a>>,
    pub(crate) flag_values: Vec<FlagValue<'a>>,
}

impl<'a> Default for Program<'a> {
    fn default() -> Program<'a> {
        Program {
            desc: "",
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

    /// Add a description to the `Program`. This will be displayed by the help text.
    pub fn with_description(mut self, desc: &'a str) -> Program {
        self.desc = desc;
        self
    }

    /// Add an optional flag to the `Program`. These do not have to be provided, but require a
    /// default value in the case of no value being provided.
    ///
    /// The name must be unique.
    pub fn with_optional_flag<T>(
        mut self,
        name: &'a str,
        default: T,
        desc: &'a str,
    ) -> Result<Program<'a>, ProgramError>
    where
        T: Display + 'static,
    {
        self = self.add_flag::<T>(name, desc, false)?;
        self.flag_defaults.push(FlagValue {
            name,
            str_value: default.to_string(),
        });
        Ok(self)
    }

    /// Add a required flag to the `Program`. These must be provided when parsing the command line
    /// arguments.
    ///
    /// The name must be unique.
    pub fn with_required_flag<T: 'static>(
        self,
        name: &'a str,
        desc: &'a str,
    ) -> Result<Program<'a>, ProgramError> {
        self.add_flag::<T>(name, desc, true)
    }

    /// Extract the parsed value by its unique name. This can fail if the argument passed cannot be
    /// parsed as a type of `T` or not registered. 
    pub fn get<T>(&self, name: &'a str) -> Result<T, ProgramError>
    where
        T: Display + FromStr + 'static,
    {
        match self.flag_values.iter().find(|fv| fv.name == name) {
            Some(flag_value) => flag_value.str_value.parse::<T>().map_err(|_| {
                let type_name = type_name::<T>().to_string();
                ProgramError::FailedToParseFlagValue {
                    name: name.to_string(),
                    type_name,
                }
            }),
            None => Err(ProgramError::NoSuchFlagExistsWithName {
                name: name.to_string(),
            }),
        }
    }

    /// A wrapper for `Program::get`, but this does not need to be converted as command line 
    /// arguments are already Strings.
    pub fn get_string(&self, name: &'a str) -> Result<String, ProgramError> {
        match self.flag_values.iter().find(|fv| fv.name == name) {
            Some(flag_value) => Ok(flag_value.str_value.to_string()),
            None => Err(ProgramError::NoSuchFlagExistsWithName {
                name: name.to_string(),
            }),
        }
    }

    fn add_flag<T: 'static>(
        mut self,
        name: &'a str,
        desc: &'a str,
        is_required: bool,
    ) -> Result<Program<'a>, ProgramError> {
        let already_has_flag_with_name = self.flags.iter().any(|f| f.name == name);
        if already_has_flag_with_name {
            // Flag names cannot be duplicate, if they are then there would be no way to parse the
            // arguments on the command line and understand which flag we want.
            return Err(ProgramError::FlagAlreadyExistsWithName {
                name: name.to_string(),
            });
        }

        let type_id = TypeId::of::<T>();
        self.flags.push(Flag {
            name,
            desc,
            type_id,
            is_required,
        });
        Ok(self)
    }

    /// Attempts to acquire the default value for a flag by name. The reason for the "unwrap" prefix
    /// is to indicate that this will call `unwrap` instead of handling `Option<FlagValue>`
    /// correctly. The assumption is made that the caller will only use this when a default flag can
    /// be used.
    pub(crate) fn unwrap_default_flag_value(&self, name: &str) -> &String {
        &self
            .flag_defaults
            .iter()
            .find(|fv| fv.name == name)
            .unwrap()
            .str_value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_add_description_when_using_with_description() {
        let expected = Program {
            desc: "A very cool test program",
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
            desc: "",
            flags: vec![
                Flag {
                    name: "flag0",
                    desc: "Zero-th flag",
                    type_id: TypeId::of::<bool>(),
                    is_required: false,
                },
                Flag {
                    name: "flag1",
                    desc: "First flag",
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
            .with_optional_flag("flag0", false, "Zero-th flag")
            .unwrap()
            .with_optional_flag("flag1", "lol", "First flag")
            .unwrap();

        assert_eq!(expected, program);
    }

    #[test]
    fn should_add_required_flags_when_calling_with_required_flag_multiple_times() {
        let expected = Program {
            desc: "",
            flags: vec![
                Flag {
                    name: "flag0",
                    desc: "Zero-th flag",
                    type_id: TypeId::of::<bool>(),
                    is_required: true,
                },
                Flag {
                    name: "flag1",
                    desc: "First flag",
                    type_id: TypeId::of::<&str>(),
                    is_required: true,
                },
            ],
            flag_defaults: vec![],
            flag_values: vec![],
        };

        let program = Program::new()
            .with_required_flag::<bool>("flag0", "Zero-th flag")
            .unwrap()
            .with_required_flag::<&str>("flag1", "First flag")
            .unwrap();

        assert_eq!(expected, program);
    }

    #[test]
    fn should_not_be_able_to_add_flags_with_the_same_name() {
        let err = Program::new()
            .with_required_flag::<bool>("oh-noes", "Ruh roh")
            .unwrap()
            .with_required_flag::<&str>("oh-noes", "Ruh roh")
            .unwrap_err();

        assert_eq!(
            ProgramError::FlagAlreadyExistsWithName {
                name: "oh-noes".to_string()
            },
            err
        );
    }
}
