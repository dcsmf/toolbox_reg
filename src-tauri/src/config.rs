use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ToolsDto<'a> {
    pub tag: &'a str,
    pub name: &'a str,
    pub location: &'a str,
}
