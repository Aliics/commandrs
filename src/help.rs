use crate::Program;

impl Program<'_> {
    pub(crate) fn generate_help_text(&self) -> String {
        format!(
            "\n{}\n\n{}\n",
            self.desc,
            self.flags
                .iter()
                .map(|f| {
                    let additional_info = if f.is_required {
                        "(required)".to_string()
                    } else {
                        let default_value = self.unwrap_default_flag_value(f.name);
                        format!("(default: {})", default_value)
                    };
                    
                    format!("\t--{} {}: {}", f.name, additional_info, f.desc)
                })
                .reduce(|acc, s| format!("{}\n{}", acc, s))
                .unwrap_or("(no args)".to_string())
        )
    }
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

	--rabbit-name (required): Name of the rabbit to observe
	--stat (required): Rabbit statistic to evaluate
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
