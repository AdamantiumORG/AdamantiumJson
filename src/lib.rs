use serde_json::Value;
use std::{fs, io, path::Path};

pub fn parse(text: &str) -> Result<Value, serde_json::Error> {
    serde_json::from_str(text)
}

pub fn pretty(text: &str) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(&parse(text)?)
}

pub fn compact(text: &str) -> Result<String, serde_json::Error> {
    serde_json::to_string(&parse(text)?)
}

pub fn type_of(text: &str) -> Result<&'static str, serde_json::Error> {
    Ok(match parse(text)? {
        Value::Null => "null",
        Value::Bool(_) => "boolean",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    })
}

pub fn read(path: impl AsRef<Path>) -> io::Result<String> {
    let text = fs::read_to_string(path)?;
    pretty(&text).map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))
}

pub fn write(path: impl AsRef<Path>, text: &str) -> io::Result<()> {
    let normalized =
        pretty(text).map_err(|error| io::Error::new(io::ErrorKind::InvalidInput, error))?;
    fs::write(path, format!("{normalized}\n"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handles_every_json_value_and_rejects_malformed_input() {
        for (source, kind) in [
            ("null", "null"),
            ("true", "boolean"),
            ("12.5", "number"),
            (r#""text""#, "string"),
            ("[1,false,null]", "array"),
            (r#"{"value":1}"#, "object"),
        ] {
            assert_eq!(type_of(source).unwrap(), kind);
            assert!(compact(source).is_ok());
        }
        assert!(parse("{").is_err());
    }
}
