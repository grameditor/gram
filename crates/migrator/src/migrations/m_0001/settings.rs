use anyhow::Result;
use serde_json::Value;

pub fn rename_status_bar_show(value: &mut Value) -> Result<()> {
    if let Some(value) = value.as_object_mut()
        && let Some(status_bar) = value.get_mut("status_bar").and_then(|v| v.as_object_mut())
        && let Some(value) = status_bar.remove("experimental.show")
    {
        status_bar.insert("show".to_string(), value);
    }
    Ok(())
}
