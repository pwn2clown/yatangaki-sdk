#[derive(Debug)]
pub enum ParsingError {
    InvalidPathAndValue,
}

impl std::fmt::Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParsingError::InvalidPathAndValue => write!(f, "invalid path and value format"),
        }
    }
}

impl std::error::Error for ParsingError {}

//  some/path/until/the:value
#[derive(Debug, PartialEq, Eq)]
pub struct PathAndValue {
    pub path: Vec<String>,
    pub value: String,
}

impl PathAndValue {
    pub fn parse(content: &str) -> Result<Self, ParsingError> {
        let (path, value) = content
            .rsplit_once(":")
            .ok_or(ParsingError::InvalidPathAndValue)?;

        Ok(Self {
            path: path.split('/').map(|e| e.into()).collect(),
            value: value.into(),
        })
    }
}

pub struct Template {
    pub fields: Vec<PathAndValue>,
}

impl Template {
    pub fn parse(content: &str) -> Result<Self, ParsingError> {
        let mut fields = vec![];

        for line in content.lines() {
            fields.push(PathAndValue::parse(line)?);
        }

        Ok(Self { fields })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_path_and_value() {
        assert!(PathAndValue::parse("some/path:value").is_ok());
        assert_eq!(
            PathAndValue::parse("some/path:value").unwrap(),
            PathAndValue {
                path: vec!["some".into(), "path".into()],
                value: "value".into()
            }
        );
    }

    #[test]
    fn parse_template_values() {
        assert!(Template::parse("a/b:c\ne/r/t:y").is_ok());
    }
}
