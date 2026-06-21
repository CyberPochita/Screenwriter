use serde::{Serialize, Deserialize};
use crate::models::structure_xml::TitlePage;
use crate::xml::gen_xml::WriterError;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PageContent {
    #[serde(rename = "$value", default)]
    pub text: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename = "document")]
pub struct ScenarioDocument {
    pub title_page: TitlePage,
    
    #[serde(rename = "pages")]
    pub pages: Vec<PageContent>,
}

impl ScenarioDocument {
    pub fn save_to_xml_string(&self) -> Result<String, WriterError> {
        let mut buffer = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        let xml_body = quick_xml::se::to_string(self)?;
        buffer.push_str(&xml_body);
        Ok(buffer)
    }

    pub fn load_from_xml_string(xml_data: &str) -> Result<Self, WriterError> {
        let doc: ScenarioDocument = quick_xml::de::from_str(xml_data)?;
        Ok(doc)
    }
}
