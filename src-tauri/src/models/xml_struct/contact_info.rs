use serde::{Deserialize, Serialize};
use super::agent_info::AgentInfo;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContactInfo {
    #[serde(default = "default_contact_margin")]
    pub left_margin: f32,
    
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>, 
    
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent: Option<AgentInfo>,
}
fn default_contact_margin() -> f32 { 8.25 }