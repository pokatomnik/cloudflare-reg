use serde_json::Value;

pub(crate) trait ValueExt<'a> {
    fn required_str(&'a self, key: &str) -> anyhow::Result<&'a str>;

    fn str(&'a self, key: &str) -> Option<&'a str>;

    fn value_by_path(&'a self, path: &[&str]) -> Option<&'a Value>;
}

impl<'a> ValueExt<'a> for &'a Value {
    fn required_str(&'a self, key: &str) -> anyhow::Result<&'a str> {
        let value = self
            .get(key)
            .ok_or_else(|| anyhow::Error::msg(format!("Missing \"{key}\"")))?;
        value
            .as_str()
            .ok_or_else(|| anyhow::Error::msg(format!("\"{key}\" value is not of String type")))
    }

    fn str(&'a self, key: &str) -> Option<&'a str> {
        let value = self.get(key)?;
        value.as_str()
    }

    fn value_by_path(&'a self, path: &[&str]) -> Option<&'a Value> {
        let mut result: &Value = self;

        for item in path {
            result = result.get(item)?;
        }

        Some(result)
    }
}
