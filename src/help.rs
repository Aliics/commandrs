use crate::Program;

impl Program<'_> {
    pub(crate) fn generate_help_text(&self) -> String {
        // We need to figure out the longest of each part of the flag.
        // It's just for formatting, though.
        let (longest_name, longest_ref_or_def, flag_data) = self
            .flags
            .iter()
            .map(|f| {
                let req_or_def = if f.is_required {
                    "(required)".to_string()
                } else {
                    let default_value = self.unwrap_default_flag_value(f.name);
                    format!("(default: {})", default_value)
                };

                (f.name, req_or_def, f.desc)
            })
            .fold(
                (0, 0, vec![]),
                |(longest_name, longest_req_or_def, acc), x| {
                    (
                        longest_name.max(x.0.len()),
                        longest_req_or_def.max(x.1.len()),
                        [acc, vec![x]].concat(),
                    )
                },
            );

        format!(
            "\n{}\n\n{}\n",
            self.desc,
            flag_data
                .iter()
                .fold(String::new(), |acc, (name, req_or_def, desc)| format!(
                    "{}\n\t--{} {}: {}",
                    acc,
                    pad_str(name.to_string(), longest_name),
                    pad_str(req_or_def.to_string(), longest_ref_or_def),
                    desc
                ))
                .strip_prefix("\n")
                .unwrap_or("(no args)")
        )
    }
}

fn pad_str(str: String, n: usize) -> String {
    (0..n).map(|i| str.chars().nth(i).unwrap_or(' ')).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_help_text_happy_path() {
        let program = Program::new()
            .with_description("A bunny observing tool!")
            .with_required_flag::<&str>("rabbit-name", "Name of the rabbit to observe")
            .unwrap()
            .with_required_flag::<&str>("stat", "Rabbit statistic to evaluate")
            .unwrap()
            .with_optional_flag::<bool>("closing-pats", true, "Pat the rabbit when finished?")
            .unwrap();

        assert_eq!(
            r#"
A bunny observing tool!

	--rabbit-name  (required)     : Name of the rabbit to observe
	--stat         (required)     : Rabbit statistic to evaluate
	--closing-pats (default: true): Pat the rabbit when finished?
"#,
            program.generate_help_text()
        );
    }

    #[test]
    fn generate_help_text_empty_program() {
        let program = Program::new().with_description("A boring tool that does nothing");

        assert_eq!(
            r#"
A boring tool that does nothing

(no args)
"#,
            program.generate_help_text()
        );
    }
}
