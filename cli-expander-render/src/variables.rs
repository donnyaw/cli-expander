use std::collections::HashMap;

pub trait VariableResolver: Send {
    fn resolve(
        &self,
        var_type: &str,
        params: &Option<serde_norway::Value>,
    ) -> anyhow::Result<String>;
}

pub struct DateVariable;

impl VariableResolver for DateVariable {
    fn resolve(
        &self,
        var_type: &str,
        params: &Option<serde_norway::Value>,
    ) -> anyhow::Result<String> {
        if var_type != "date" {
            anyhow::bail!("DateVariable cannot resolve type: {}", var_type);
        }

        let format = params
            .as_ref()
            .and_then(|p| p.get("format"))
            .and_then(|v| v.as_str())
            .unwrap_or("%Y-%m-%d");

        let offset_secs = params
            .as_ref()
            .and_then(|p| p.get("offset"))
            .and_then(|v| v.as_i64())
            .unwrap_or(0);

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;

        let adjusted = now + offset_secs;
        let secs = adjusted as u64;

        let datetime = chrono::DateTime::from_timestamp(secs as i64, 0).unwrap_or_default();

        Ok(datetime.format(format).to_string())
    }
}

pub struct ClipboardVariable;

impl VariableResolver for ClipboardVariable {
    fn resolve(
        &self,
        var_type: &str,
        _params: &Option<serde_norway::Value>,
    ) -> anyhow::Result<String> {
        if var_type != "clipboard" {
            anyhow::bail!("ClipboardVariable cannot resolve type: {}", var_type);
        }
        let content = arboard::Clipboard::new()
            .map_err(|e| anyhow::anyhow!("Failed to open clipboard: {}", e))?
            .get_text()
            .map_err(|e| anyhow::anyhow!("Failed to get clipboard text: {}", e))?;
        Ok(content)
    }
}

pub struct ShellVariable;

impl VariableResolver for ShellVariable {
    fn resolve(
        &self,
        var_type: &str,
        params: &Option<serde_norway::Value>,
    ) -> anyhow::Result<String> {
        if var_type != "shell" {
            anyhow::bail!("ShellVariable cannot resolve type: {}", var_type);
        }

        let cmd = params
            .as_ref()
            .and_then(|p| p.get("cmd"))
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing 'cmd' parameter for shell variable"))?;

        let output = std::process::Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .output()
            .map_err(|e| anyhow::anyhow!("Shell command failed: {}", e))?;

        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
        Ok(stdout)
    }
}

pub struct VariableEngine {
    resolvers: HashMap<String, Box<dyn VariableResolver>>,
}

impl Default for VariableEngine {
    fn default() -> Self {
        let mut engine = Self {
            resolvers: HashMap::new(),
        };
        engine.register("date", Box::new(DateVariable));
        engine.register("clipboard", Box::new(ClipboardVariable));
        engine.register("shell", Box::new(ShellVariable));
        engine
    }
}

impl VariableEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, var_type: &str, resolver: Box<dyn VariableResolver>) {
        self.resolvers.insert(var_type.to_string(), resolver);
    }

    pub fn resolve(
        &self,
        var_type: &str,
        params: &Option<serde_norway::Value>,
    ) -> anyhow::Result<String> {
        match self.resolvers.get(var_type) {
            Some(resolver) => resolver.resolve(var_type, params),
            None => anyhow::bail!("Unknown variable type: {}", var_type),
        }
    }

    pub fn resolve_all(
        &self,
        vars: &[cli_expander_config::VariableDef],
    ) -> anyhow::Result<HashMap<String, String>> {
        let mut results = HashMap::new();
        for var in vars {
            let value = self.resolve(&var.var_type, &var.params)?;
            results.insert(var.name.clone(), value);
        }
        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date_resolve_default_format() {
        let resolver = DateVariable;
        let result = resolver.resolve("date", &None).unwrap();
        // Should match YYYY-MM-DD format
        assert_eq!(result.len(), 10);
        assert_eq!(result.chars().filter(|&c| c == '-').count(), 2);
    }

    #[test]
    fn test_date_resolve_custom_format() {
        let resolver = DateVariable;
        let params = Some(serde_norway::from_str("format: '%H:%M'").unwrap());
        let result = resolver.resolve("date", &params).unwrap();
        assert_eq!(result.len(), 5);
        assert_eq!(result.chars().filter(|&c| c == ':').count(), 1);
    }

    #[test]
    fn test_date_wrong_type() {
        let resolver = DateVariable;
        assert!(resolver.resolve("shell", &None).is_err());
    }

    #[test]
    fn test_variable_engine_default() {
        let engine = VariableEngine::default();
        let result = engine.resolve("date", &None).unwrap();
        assert_eq!(result.len(), 10);
    }

    #[test]
    fn test_variable_engine_unknown_type() {
        let engine = VariableEngine::default();
        assert!(engine.resolve("unknown", &None).is_err());
    }

    #[test]
    fn test_variable_engine_resolve_all() {
        use cli_expander_config::VariableDef;

        let engine = VariableEngine::default();
        let vars = vec![VariableDef {
            name: "mydate".to_string(),
            var_type: "date".to_string(),
            params: Some(serde_norway::from_str("format: '%Y'").unwrap()),
        }];

        let result = engine.resolve_all(&vars).unwrap();
        assert!(result.contains_key("mydate"));
        assert_eq!(result["mydate"].len(), 4); // YYYY
    }
}
