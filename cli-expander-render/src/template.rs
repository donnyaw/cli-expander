use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Template {
    source: String,
}

impl Template {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_string(),
        }
    }

    pub fn render(&self, vars: &HashMap<String, String>) -> String {
        let mut result = self.source.clone();
        for (key, value) in vars {
            let placeholder = format!("{{{{{}}}}}", key);
            result = result.replace(&placeholder, value);
        }
        result
    }

    pub fn cursor_position(&self) -> Option<usize> {
        self.source.find("$|$")
    }

    pub fn has_cursor_marker(&self) -> bool {
        self.source.contains("$|$")
    }

    pub fn extract_variable_names(&self) -> Vec<String> {
        let mut names = Vec::new();
        let mut remaining = self.source.as_str();
        while let Some(start) = remaining.find("{{") {
            remaining = &remaining[start + 2..];
            if let Some(end) = remaining.find("}}") {
                let name = remaining[..end].trim().to_string();
                if !name.is_empty() {
                    names.push(name);
                }
                remaining = &remaining[end + 2..];
            }
        }
        names
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_render() {
        let t = Template::new("Hello {{name}}!");
        let mut vars = HashMap::new();
        vars.insert("name".to_string(), "World".to_string());
        assert_eq!(t.render(&vars), "Hello World!");
    }

    #[test]
    fn test_multiple_vars() {
        let t = Template::new("{{a}} and {{b}}");
        let mut vars = HashMap::new();
        vars.insert("a".to_string(), "1".to_string());
        vars.insert("b".to_string(), "2".to_string());
        assert_eq!(t.render(&vars), "1 and 2");
    }

    #[test]
    fn test_no_vars() {
        let t = Template::new("plain text");
        let vars = HashMap::new();
        assert_eq!(t.render(&vars), "plain text");
    }

    #[test]
    fn test_cursor_position() {
        let t = Template::new("before $|$ after");
        assert_eq!(t.cursor_position(), Some(7));
    }

    #[test]
    fn test_no_cursor() {
        let t = Template::new("no cursor here");
        assert_eq!(t.cursor_position(), None);
    }

    #[test]
    fn test_extract_variable_names() {
        let t = Template::new("{{greeting}} {{name}}, {{form.field}}");
        let names = t.extract_variable_names();
        assert_eq!(names, vec!["greeting", "name", "form.field"]);
    }

    #[test]
    fn test_dot_notation_variable() {
        let t = Template::new("{{form1.name}}");
        let names = t.extract_variable_names();
        assert_eq!(names, vec!["form1.name"]);
    }

    #[test]
    fn test_has_cursor_marker() {
        let t = Template::new("text $|$ text");
        assert!(t.has_cursor_marker());
    }
}
