use std::any::TypeId;
use std::fmt::Display;

use crate::error::ProgramError;
use crate::flag::{Flag, FlagValue};

#[derive(PartialEq, Debug)]
pub struct Program {
    pub description: String,
    pub flags: Vec<Flag>,
    pub flag_defaults: Vec<FlagValue>,
}

impl Default for Program {
    fn default() -> Self {
        Program {
            description: String::default(),
            flags: vec![],
            flag_defaults: vec![],
        }
    }
}

impl Program {
    pub fn with_description(mut self, desc: &str) -> Program {
        self.description = desc.to_string();
        self
    }

    pub fn with_optional_flag<T>(mut self, name: &str, default: T) -> Result<Program, ProgramError>
    where
        T: Display + 'static,
    {
        self.add_flag::<T>(name.to_string(), false)?;
        self.flag_defaults.push(FlagValue {
            name: name.to_string(),
            str_value: default.to_string(),
        });
        Ok(self)
    }

    pub fn with_required_flag<T: 'static>(mut self, name: &str) -> Result<Program, ProgramError> {
        self.add_flag::<T>(name.to_string(), true)?;
        Ok(self)
    }

    fn add_flag<T: 'static>(
        &mut self,
        name: String,
        is_required: bool,
    ) -> Result<(), ProgramError> {
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
            type_id,
            is_required,
        });
        Ok(())
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
                    name: "flag0".to_string(),
                    type_id: TypeId::of::<bool>(),
                    is_required: false,
                },
                Flag {
                    name: "flag1".to_string(),
                    type_id: TypeId::of::<&str>(),
                    is_required: false,
                },
            ],
            flag_defaults: vec![
                FlagValue {
                    name: "flag0".to_string(),
                    str_value: "false".to_string(),
                },
                FlagValue {
                    name: "flag1".to_string(),
                    str_value: "lol".to_string(),
                },
            ],
        };

        let program = Program::default()
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
                    name: "flag0".to_string(),
                    type_id: TypeId::of::<bool>(),
                    is_required: true,
                },
                Flag {
                    name: "flag1".to_string(),
                    type_id: TypeId::of::<&str>(),
                    is_required: true,
                },
            ],
            flag_defaults: vec![],
        };

        let program = Program::default()
            .with_required_flag::<bool>("flag0")
            .unwrap()
            .with_required_flag::<&str>("flag1")
            .unwrap();

        assert_eq!(expected, program);
    }

    #[test]
    fn should_not_be_able_to_add_flags_with_the_same_name() {
        let expected = ProgramError::FlagAlreadyExistsWithName {
            name: "oh noes".to_string(),
        };

        let err = Program::default()
            .with_required_flag::<bool>("oh noes")
            .unwrap()
            .with_required_flag::<&str>("oh noes")
            .unwrap_err();

        assert_eq!(expected, err);
    }
}
