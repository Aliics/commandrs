use std::any::TypeId;
use std::collections::HashMap;
use std::env;
use std::string::ToString;

use lazy_static::lazy_static;

use crate::error::ProgramError;
use crate::error::ProgramError::HelpFlagGiven;
use crate::flag::{Flag, FlagValue};
use crate::Program;

const ARG_PREFIX: &str = "--";
const HELP_FLAG: &str = "help";

lazy_static! {
    static ref BOOL_TYPE_ID: TypeId = TypeId::of::<bool>();
}

impl<'a> Program<'a> {
    /// Parse command line arguments and store their values against the flags configured on 
    /// `Program`. These values are stored in their string representation until later fetched.
    pub fn parse(self) -> Result<Program<'a>, ProgramError> {
        self.parse_from_strings(env::args().collect())
    }

    /// Just wraps `Program::parse_from_strings`, but instead accepts a `&[&str]`.
    pub fn parse_from_str_arr(self, arr: &[&str]) -> Result<Program<'a>, ProgramError> {
        self.parse_from_strings(arr.iter().map(|s| s.to_string()).collect())
    }

    /// Parse the given `args` parameters and store their values against the flags configured on 
    /// `Program`. These values are stored in their string representation until later fetched.
    /// 
    /// Generally, this function will not be used, and instead you will want the `Program::parse`
    /// function for most programs.
    pub fn parse_from_strings(mut self, args: Vec<String>) -> Result<Program<'a>, ProgramError> {
        let given_flag_args: HashMap<&str, Option<&String>> = args
            .iter()
            .enumerate()
            .filter(|(_, a)| is_in_arg_format(a))
            .map(|(i, a)| {
                let arg_name = a.strip_prefix(ARG_PREFIX).unwrap_or(a);
                let requires_value = self
                    .flags
                    .iter()
                    .find(|f| f.name == arg_name)
                    .map(|f| f.type_id != *BOOL_TYPE_ID)
                    .unwrap_or(false);

                let arg_value = args
                    .get(i + 1)
                    .map(|b| b)
                    .filter(|s| requires_value || !is_in_arg_format(s));
                (arg_name, arg_value)
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
                     ..
                 }| match given_flag_args.get(name) {
                    Some(Some(given_arg)) => Ok(FlagValue {
                        name,
                        str_value: given_arg.to_string(),
                    }),
                    Some(_) if type_id == *BOOL_TYPE_ID => Ok(FlagValue {
                        name,
                        str_value: true.to_string(),
                    }),
                    Some(None) => Err(ProgramError::RequiredArgWasNotGiven {
                        name: name.to_string(),
                    }),
                    None if is_required => Err(ProgramError::RequiredArgWasNotGiven {
                        name: name.to_string(),
                    }),
                    None => {
                        let flag_value = self.unwrap_default_flag_value(name);
                        Ok(FlagValue {
                            name,
                            str_value: flag_value.to_string(),
                        })
                    }
                },
            )
            .collect();

        if let Some(Err(err)) = flag_value_mutations.iter().find(|r| r.is_err()) {
            return Err(err.clone());
        }

        if given_flag_args.contains_key(HELP_FLAG) {
            println!("{}", self.generate_help_text());

            return Err(HelpFlagGiven);
        }

        self.flag_values = flag_value_mutations
            .into_iter()
            .filter_map(|r| r.ok())
            .collect();

        Ok(self)
    }
}

fn is_in_arg_format(s: &str) -> bool {
    s.starts_with(ARG_PREFIX)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_have_values_for_given_args_when_parsed() {
        let name_value = Program::new()
            .with_required_flag::<&str>("name", "Your name")
            .unwrap()
            .parse_from_str_arr(&["--name", "Ollie"])
            .unwrap()
            .get_string("name")
            .unwrap();

        assert_eq!("Ollie", name_value);
    }

    #[test]
    fn should_have_values_for_given_args_when_parsed_and_convert_them() {
        let name_value = Program::new()
            .with_required_flag::<usize>("cranberries", "Number of cranberries")
            .unwrap()
            .parse_from_str_arr(&["--cranberries", "314159265358979"])
            .unwrap()
            .get::<usize>("cranberries")
            .unwrap();

        assert_eq!(314159265358979, name_value);
    }

    #[test]
    fn should_result_in_an_error_when_required_arg_is_not_given() {
        let err = Program::new()
            .with_required_flag::<&str>("required-flag", "A required flag, wow")
            .unwrap()
            .parse_from_str_arr(&[])
            .unwrap_err();

        assert_eq!(
            ProgramError::RequiredArgWasNotGiven {
                name: "required-flag".to_string()
            },
            err
        );
    }

    #[test]
    fn should_result_in_an_error_when_parsing_fails_for_type() {
        let program = Program::new()
            .with_required_flag::<u8>("age", "Your age")
            .unwrap()
            .parse_from_str_arr(&["--age", "who?"])
            .unwrap();

        let err = program.get::<u8>("age").unwrap_err();

        assert_eq!(
            ProgramError::FailedToParseFlagValue {
                name: "age".to_string(),
                type_name: "u8".to_string()
            },
            err
        );
    }

    #[test]
    fn should_use_default_values_for_optional_args_when_parsed() {
        let name_value = Program::new()
            .with_optional_flag::<&str>("name", "Mr. Ollie", "Your name")
            .unwrap()
            .parse_from_str_arr(&["--something", "else"])
            .unwrap()
            .get_string("name")
            .unwrap();

        assert_eq!("Mr. Ollie", name_value);
    }

    #[test]
    fn should_still_use_boolean_flag_even_when_value_is_not_explicitly_given() {
        let program = Program::new()
            .with_optional_flag::<bool>("is-wonderful", false, "Is it wonderful?")
            .unwrap()
            .with_required_flag::<&str>("name", "Your name")
            .unwrap()
            .parse_from_str_arr(&["--is-wonderful", "--name", "Dr. Ollie"])
            .unwrap();

        let is_wonderful = program.get::<bool>("is-wonderful").unwrap();
        let name = program.get_string("name").unwrap();

        assert!(is_wonderful);
        assert_eq!("Dr. Ollie", name);
    }

    #[test]
    fn should_still_use_boolean_flag_when_value_is_explicitly_given() {
        let program = Program::new()
            .with_required_flag::<bool>("is-great", "Is it great?")
            .unwrap()
            .with_required_flag::<&str>("name", "Your name")
            .unwrap()
            .parse_from_str_arr(&["--is-great", "true", "--name", "Dr. Ollie"])
            .unwrap();

        let is_great = program.get::<bool>("is-great").unwrap();
        let name = program.get_string("name").unwrap();

        assert!(is_great);
        assert_eq!("Dr. Ollie", name);
    }
}
