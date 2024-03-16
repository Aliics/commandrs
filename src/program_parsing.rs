use std::any::TypeId;
use std::collections::HashMap;
use crate::error::ProgramError;
use crate::flag::{Flag, FlagValue};
use crate::Program;

impl <'a> Program<'a> {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    
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