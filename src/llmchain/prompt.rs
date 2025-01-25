use std::collections::HashMap;

#[derive(Debug)]
pub struct PromptTemplate {
    template: String,
}

impl PromptTemplate {
    pub fn new(template: &str) -> Self {
        Self {
            template: template.to_string(),
        }
    }

    /// Format template with a single input (backward compatible)
    pub fn format(&self, input: &str) -> String {
        self.template.replace("{}", input)
    }

    /// Format template with multiple named variables
    pub fn format_with_vars(&self, vars: &HashMap<&str, &str>) -> String {
        let mut result = self.template.clone();
        for (key, value) in vars {
            result = result.replace(&format!("{{{}}}", key), value);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_variable() {
        let template = PromptTemplate::new("Hello, {name}!");
        let mut vars = HashMap::new();
        vars.insert("name", "Alice");
        assert_eq!(template.format_with_vars(&vars), "Hello, Alice!");
    }

    #[test]
    fn test_multiple_variables() {
        let template = PromptTemplate::new("{greeting}, {name}! How is your {day}?");
        let mut vars = HashMap::new();
        vars.insert("greeting", "Good morning");
        vars.insert("name", "Bob");
        vars.insert("day", "Monday");
        assert_eq!(
            template.format_with_vars(&vars),
            "Good morning, Bob! How is your Monday?"
        );
    }

    #[test]
    fn test_mixed_formatting() {
        let template = PromptTemplate::new("{} {greeting}, {name}!");
        let mut vars = HashMap::new();
        vars.insert("greeting", "Hello");
        vars.insert("name", "Charlie");
        assert_eq!(
            template.format_with_vars(&vars).replace("{}", "1"),
            "1 Hello, Charlie!"
        );
    }
}
