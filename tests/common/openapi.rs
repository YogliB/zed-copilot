use serde_json::{json, Value};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct OpenApiSpec {
    pub spec: Value,
    pub schemas: HashMap<String, Value>,
}

impl OpenApiSpec {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let mut content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read OpenAPI spec: {}", e))?;

        let spec: Value = if content.trim().starts_with('{') {
            serde_json::from_str(&content)
                .map_err(|e| format!("Failed to parse OpenAPI JSON: {}", e))?
        } else {
            content = Self::preprocess_yaml(&content);
            serde_yaml::from_str(&content)
                .map_err(|e| format!("Failed to parse OpenAPI YAML: {}", e))?
        };

        let mut schemas = HashMap::new();
        if let Some(components) = spec.get("components").and_then(|c| c.get("schemas")) {
            if let Some(obj) = components.as_object() {
                for (name, schema) in obj {
                    schemas.insert(name.clone(), schema.clone());
                }
            }
        }

        Ok(Self { spec, schemas })
    }

    fn preprocess_yaml(content: &str) -> String {
        let re_bounds = Regex::new(r"(minimum|maximum):\s*-?[0-9]{18,}").unwrap();

        let max_i64 = "9223372036854775807";
        let min_i64 = "-9223372036854775808";

        re_bounds.replace_all(content, |caps: &regex::Captures| {
            let keyword = &caps[1];
            if keyword == "minimum" {
                format!("minimum: {}", min_i64)
            } else {
                format!("maximum: {}", max_i64)
            }
        }).to_string()
    }

    pub fn get_schema(&self, name: &str) -> Option<&Value> {
        self.schemas.get(name)
    }

    pub fn get_endpoint_schema(&self, path: &str, method: &str) -> Option<Value> {
        let method = method.to_lowercase();

        if let Some(paths) = self.spec.get("paths") {
            if let Some(path_obj) = paths.get(path) {
                if let Some(operation) = path_obj.get(&method) {
                    if let Some(responses) = operation.get("responses") {
                        if let Some(response_200) = responses.get("200") {
                            if let Some(content) = response_200.get("content") {
                                if let Some(json_content) = content.get("application/json") {
                                    if let Some(schema) = json_content.get("schema") {
                                        return Some(schema.clone());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        None
    }

    pub fn resolve_schema_ref(&self, schema: &Value) -> Value {
        if let Some(ref_str) = schema.get("$ref").and_then(|r| r.as_str()) {
            let schema_name = ref_str.split('/').last().unwrap_or("");
            if let Some(resolved) = self.get_schema(schema_name) {
                return resolved.clone();
            }
        }
        schema.clone()
    }

    pub fn validate_against_schema(&self, data: &Value, schema_name: &str) -> Result<(), String> {
        if let Some(schema) = self.get_schema(schema_name) {
            self.validate_json(data, schema)
        } else {
            Err(format!("Schema '{}' not found in OpenAPI spec", schema_name))
        }
    }

    pub fn validate_json(&self, data: &Value, schema: &Value) -> Result<(), String> {
        let resolved_schema = self.resolve_schema_ref(schema);

        if let Some(type_str) = resolved_schema.get("type").and_then(|t| t.as_str()) {
            match type_str {
                "object" => self.validate_object(data, &resolved_schema)?,
                "array" => self.validate_array(data, &resolved_schema)?,
                "string" => self.validate_string(data, &resolved_schema)?,
                "number" | "integer" => self.validate_number(data, &resolved_schema)?,
                "boolean" => {
                    if !data.is_boolean() {
                        return Err(format!("Expected boolean, got {}", data.type_str()));
                    }
                }
                "null" => {
                    if !data.is_null() {
                        return Err(format!("Expected null, got {}", data.type_str()));
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn validate_object(&self, data: &Value, schema: &Value) -> Result<(), String> {
        if !data.is_object() {
            return Err(format!("Expected object, got {}", data.type_str()));
        }

        let required = schema
            .get("required")
            .and_then(|r| r.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>())
            .unwrap_or_default();

        for field in &required {
            if data.get(field).is_none() {
                return Err(format!("Required field '{}' is missing", field));
            }
        }

        if let Some(properties) = schema.get("properties").and_then(|p| p.as_object()) {
            for (prop_name, prop_schema) in properties {
                if let Some(prop_value) = data.get(prop_name) {
                    self.validate_json(prop_value, prop_schema)?;
                }
            }
        }

        Ok(())
    }

    fn validate_array(&self, data: &Value, schema: &Value) -> Result<(), String> {
        if !data.is_array() {
            return Err(format!("Expected array, got {}", data.type_str()));
        }

        if let Some(item_schema) = schema.get("items") {
            for item in data.as_array().unwrap() {
                self.validate_json(item, item_schema)?;
            }
        }

        Ok(())
    }

    fn validate_string(&self, data: &Value, schema: &Value) -> Result<(), String> {
        if !data.is_string() {
            return Err(format!("Expected string, got {}", data.type_str()));
        }

        if let Some(max_len) = schema.get("maxLength").and_then(|m| m.as_u64()) {
            let str_len = data.as_str().unwrap_or("").len();
            if str_len as u64 > max_len {
                return Err(format!(
                    "String length {} exceeds maximum {}",
                    str_len, max_len
                ));
            }
        }

        if let Some(min_len) = schema.get("minLength").and_then(|m| m.as_u64()) {
            let str_len = data.as_str().unwrap_or("").len();
            if (str_len as u64) < min_len {
                return Err(format!(
                    "String length {} is below minimum {}",
                    str_len, min_len
                ));
            }
        }

        if let Some(pattern) = schema.get("pattern").and_then(|p| p.as_str()) {
            if let Ok(re) = regex::Regex::new(pattern) {
                let str_val = data.as_str().unwrap_or("");
                if !re.is_match(str_val) {
                    return Err(format!("String '{}' does not match pattern '{}'", str_val, pattern));
                }
            }
        }

        Ok(())
    }

    fn validate_number(&self, data: &Value, schema: &Value) -> Result<(), String> {
        if !data.is_number() {
            return Err(format!("Expected number, got {}", data.type_str()));
        }

        if let Some(maximum) = schema.get("maximum").and_then(|m| m.as_f64()) {
            if let Some(num) = data.as_f64() {
                if num > maximum {
                    return Err(format!("Number {} exceeds maximum {}", num, maximum));
                }
            }
        }

        if let Some(minimum) = schema.get("minimum").and_then(|m| m.as_f64()) {
            if let Some(num) = data.as_f64() {
                if num < minimum {
                    return Err(format!("Number {} is below minimum {}", num, minimum));
                }
            }
        }

        Ok(())
    }
}

trait JsonTypeStr {
    fn type_str(&self) -> &'static str;
}

impl JsonTypeStr for Value {
    fn type_str(&self) -> &'static str {
        match self {
            Value::Null => "null",
            Value::Bool(_) => "boolean",
            Value::Number(_) => "number",
            Value::String(_) => "string",
            Value::Array(_) => "array",
            Value::Object(_) => "object",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_simple_string() {
        let schema = json!({
            "type": "string"
        });
        let data = json!("hello");

        let api_spec = OpenApiSpec {
            spec: json!({}),
            schemas: HashMap::new(),
        };

        assert!(api_spec.validate_json(&data, &schema).is_ok());
    }

    #[test]
    fn test_validate_simple_object() {
        let schema = json!({
            "type": "object",
            "properties": {
                "name": { "type": "string" },
                "age": { "type": "integer" }
            },
            "required": ["name"]
        });
        let data = json!({
            "name": "John",
            "age": 30
        });

        let api_spec = OpenApiSpec {
            spec: json!({}),
            schemas: HashMap::new(),
        };

        assert!(api_spec.validate_json(&data, &schema).is_ok());
    }

    #[test]
    fn test_validate_missing_required_field() {
        let schema = json!({
            "type": "object",
            "properties": {
                "name": { "type": "string" }
            },
            "required": ["name"]
        });
        let data = json!({});

        let api_spec = OpenApiSpec {
            spec: json!({}),
            schemas: HashMap::new(),
        };

        assert!(api_spec.validate_json(&data, &schema).is_err());
    }

    #[test]
    fn test_validate_array() {
        let schema = json!({
            "type": "array",
            "items": { "type": "string" }
        });
        let data = json!(["a", "b", "c"]);

        let api_spec = OpenApiSpec {
            spec: json!({}),
            schemas: HashMap::new(),
        };

        assert!(api_spec.validate_json(&data, &schema).is_ok());
    }

    #[test]
    fn test_validate_wrong_type() {
        let schema = json!({
            "type": "string"
        });
        let data = json!(123);

        let api_spec = OpenApiSpec {
            spec: json!({}),
            schemas: HashMap::new(),
        };

        assert!(api_spec.validate_json(&data, &schema).is_err());
    }
}
