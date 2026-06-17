use std::collections::HashMap;
use cli_expander_config::FieldConfig;

pub struct FormExtension;

impl FormExtension {
    pub fn render_form(
        layout: &str,
        _fields: &HashMap<String, FieldConfig>,
        values: &HashMap<String, String>,
    ) -> String {
        let mut result = layout.to_string();
        for (key, value) in values {
            let placeholder = format!("[[{}]]", key);
            result = result.replace(&placeholder, value);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_form_simple() {
        let mut fields = HashMap::new();
        fields.insert(
            "name".to_string(),
            FieldConfig {
                field_type: None,
                multiline: None,
                default: None,
                placeholder: Some("Enter name".to_string()),
                values: None,
                trim_string_values: None,
                depends_on: None,
            },
        );

        let mut values = HashMap::new();
        values.insert("name".to_string(), "Alice".to_string());

        let result = FormExtension::render_form("Hello [[name]]!", &fields, &values);
        assert_eq!(result, "Hello Alice!");
    }

    #[test]
    fn test_render_form_multiple_fields() {
        let mut values = HashMap::new();
        values.insert("first".to_string(), "John".to_string());
        values.insert("last".to_string(), "Doe".to_string());

        let result =
            FormExtension::render_form("Name: [[first]] [[last]]", &HashMap::new(), &values);
        assert_eq!(result, "Name: John Doe");
    }

    #[test]
    fn test_render_form_unknown_field() {
        let mut values = HashMap::new();
        values.insert("known".to_string(), "value".to_string());

        let result =
            FormExtension::render_form("[[known]] and [[unknown]]", &HashMap::new(), &values);
        assert_eq!(result, "value and [[unknown]]");
    }
}
