use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
pub struct Metadata {
    pub debug: Option<bool>,
    pub family: Option<String>,
    pub init_params: Option<String>,
    pub instance: Option<String>,
    pub memory: Option<u64>,
    pub name: Option<String>,
    pub region: Option<String>,
    pub url: Option<String>,
    pub vcpu: Option<u32>,
}
