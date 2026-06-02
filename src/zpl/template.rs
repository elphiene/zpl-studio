// ZPL template handling

use std::collections::HashMap;

/// A variable in a ZPL template
#[derive(Debug, Clone)]
pub struct TemplateVariable {
    pub name: String,
    pub display_name: String,
    pub value: String,
}

/// A ZPL template with variables
#[derive(Debug, Clone)]
pub struct ZplTemplate {
    pub raw_zpl: String,
    pub variables: Vec<TemplateVariable>,
}

impl ZplTemplate {
    /// Create a new template from raw ZPL
    pub fn from_zpl(zpl: String) -> Self {
        let variables = extract_variables(&zpl);
        Self {
            raw_zpl: zpl,
            variables,
        }
    }

    /// Render the template with variable values
    pub fn render(&self, values: &HashMap<String, String>) -> Result<String, String> {
        let mut result = self.raw_zpl.clone();

        // Replace all variables
        for var in &self.variables {
            let value = values.get(&var.name).ok_or_else(|| {
                format!("Missing value for variable: {}", var.display_name)
            })?;

            // Replace {{VAR}} and {{VAR:Label}}
            // Use regex to handle both forms at once
            let escaped_name = regex::escape(&var.name);
            let pattern = format!(r"\{{\{{{}(?::[^}}]+)?\}}\}}", escaped_name);
            let re = regex::Regex::new(&pattern).unwrap();

            result = re.replace_all(&result, value.as_str()).to_string();
        }

        Ok(result)
    }

    /// Get a map of variable values
    pub fn get_values_map(&self) -> HashMap<String, String> {
        self.variables
            .iter()
            .map(|v| (v.name.clone(), v.value.clone()))
            .collect()
    }
}

/// Extract variables from ZPL code
/// Finds patterns like {{NAME}} or {{NAME:Display Label}}
/// Supports any characters in variable names (including spaces, lowercase, etc.)
fn extract_variables(zpl: &str) -> Vec<TemplateVariable> {
    let re = regex::Regex::new(r"\{\{([^:}]+?)(?::([^}]+))?\}\}").unwrap();
    let mut vars = Vec::new();
    let mut seen = std::collections::HashSet::new();

    for cap in re.captures_iter(zpl) {
        let name = cap.get(1).unwrap().as_str().trim().to_string();

        // Skip if we've already seen this variable
        if seen.contains(&name) {
            continue;
        }
        seen.insert(name.clone());

        let display_name = cap
            .get(2)
            .map(|m| m.as_str().trim().to_string())
            .unwrap_or_else(|| name.clone());

        vars.push(TemplateVariable {
            name,
            display_name,
            value: String::new(),
        });
    }

    vars
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_variables() {
        let zpl = "^XA\n^FO20,60^FD{{NAME}}^FS\n^FO20,120^FD{{PHONE:Phone Number}}^FS\n^XZ";
        let vars = extract_variables(zpl);

        assert_eq!(vars.len(), 2);
        assert_eq!(vars[0].name, "NAME");
        assert_eq!(vars[0].display_name, "NAME");
        assert_eq!(vars[1].name, "PHONE");
        assert_eq!(vars[1].display_name, "Phone Number");
    }

    #[test]
    fn test_extract_variables_with_spaces_and_lowercase() {
        let zpl = "^XA\n^FO50,30^FD{{Detail}}^FS\n^FO50,130^FD{{Cherrys Labs Tracking ID}}^FS\n^XZ";
        let vars = extract_variables(zpl);

        assert_eq!(vars.len(), 2);
        assert_eq!(vars[0].name, "Detail");
        assert_eq!(vars[0].display_name, "Detail");
        assert_eq!(vars[1].name, "Cherrys Labs Tracking ID");
        assert_eq!(vars[1].display_name, "Cherrys Labs Tracking ID");
    }

    #[test]
    fn test_render_template() {
        let zpl = "^XA\n^FO20,60^FD{{NAME}}^FS\n^FO20,120^FD{{PHONE:Phone Number}}^FS\n^XZ";
        let template = ZplTemplate::from_zpl(zpl.to_string());

        let mut values = HashMap::new();
        values.insert("NAME".to_string(), "John Doe".to_string());
        values.insert("PHONE".to_string(), "555-1234".to_string());

        let result = template.render(&values).unwrap();
        assert!(result.contains("John Doe"));
        assert!(result.contains("555-1234"));
        assert!(!result.contains("{{"));
    }
}
