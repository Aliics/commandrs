use std::any::{type_name, TypeId};
use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;
use std::string::ParseError;

use crate::error::ProgramError;
use crate::flag::{Flag, FlagValue};

#[derive(PartialEq, Debug)]
pub struct Program<'a> {
    description: String,
    flags: Vec<Flag<'a>>,
    flag_defaults: Vec<FlagValue<'a>>,
    flag_values: Vec<FlagValue<'a>>,
}

impl<'a> Default for Program<'a> {
    fn default() -> Program<'a> {
        Program {
            description: String::default(),
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
        self.description = desc.to_string();
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

    pub fn parse_from_arr(mut self, arr: &[&str]) -> Result<Program<'a>, ProgramError<'a>> {
        let given_flag_args: HashMap<&str, Option<&str>> = arr
            .into_iter()
            .enumerate()
            .filter(|(_, &a)| a.starts_with("--") || a.starts_with("-"))
            .map(|(i, &a)| {
                (
                    a.strip_prefix("--").or(a.strip_prefix("-")).unwrap_or(a),
                    arr.get(i + 1).map(|&b| b),
                )
            })
            .collect();

        let flag_value_mutations: Vec<Result<FlagValue, ProgramError>> = self
            .flags
            .iter()
            .map(
                |&Flag {
                     name,
                     type_id,
                     is_required,
                 }| match given_flag_args.get(name) {
                    Some(Some(given_arg)) => Ok(FlagValue {
                        name,
                        str_value: given_arg.to_string(),
                    }),
                    Some(None) if type_id == TypeId::of::<bool>() => Ok(FlagValue {
                        name,
                        str_value: "true".to_string(),
                    }),
                    Some(None) => Err(ProgramError::RequiredArgWasNotGiven { name }),
                    None if is_required => Err(ProgramError::RequiredArgWasNotGiven { name }),
                    None => {
                        let FlagValue {
                            str_value: default_value,
                            ..
                        } = self
                            .flag_defaults
                            .iter()
                            .find(|fv| fv.name == name)
                            .unwrap();
                        
                        Ok(FlagValue {
                            name,
                            str_value: default_value.to_string(),
                        })
                    }
                },
            )
            .collect();

        if let Some(Err(err)) = flag_value_mutations.iter().find(|r| r.is_err()) {
            return Err(*err);
        }

        self.flag_values = flag_value_mutations
            .into_iter()
            .filter_map(|r| r.ok())
            .collect();

        Ok(self)
    }

    pub fn get<T>(self, name: &str) -> Result<T, ProgramError>
    where
        T: Display + FromStr<Err = ParseError> + 'static,
    {
        match self.flag_values.iter().find(|fv| fv.name == name) {
            Some(flag_value) => flag_value.str_value.parse::<T>().map_err(|_| {
                let type_name = type_name::<T>();
                ProgramError::FailedToParseFlagValue { name, type_name }
            }),
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
            description: "A very cool test program".to_string(),
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
            description: String::default(),
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
            description: String::default(),
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
        let expected = ProgramError::FlagAlreadyExistsWithName { name: "oh-noes" };

        let err = Program::new()
            .with_required_flag::<bool>("oh-noes")
            .unwrap()
            .with_required_flag::<&str>("oh-noes")
            .unwrap_err();

        assert_eq!(expected, err);
    }

    #[test]
    fn should_add_values_for_given_args_when_parsed() {
        let name_value = Program::new()
            .with_required_flag::<&str>("name")
            .unwrap()
            .parse_from_arr(&["--name", "Ollie"])
            .unwrap()
            .get::<String>("name")
            .unwrap();

        assert_eq!("Ollie", name_value);
    }

    #[test]
    fn should_use_default_values_for_optional_args_when_parsed() {
        let name_value = Program::new()
            .with_optional_flag::<&str>("name", "Mr. Ollie")
            .unwrap()
            .parse_from_arr(&["--something", "else"])
            .unwrap()
            .get::<String>("name")
            .unwrap();

        assert_eq!("Mr. Ollie", name_value);
    }
}
