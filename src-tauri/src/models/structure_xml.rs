use serde::{Serialize, Deserialize};
use super::xml_struct::authorship::Authorship;
use super::xml_struct::contact_info::ContactInfo;
use super::xml_struct::formatting::Formatting;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename = "title_page")]
pub struct TitlePage {
    #[serde(default)]
    pub formatting: Formatting,

    pub title: String,
    pub author: String,
    pub authorship: Authorship,
    pub contact: ContactInfo,
}