use serde::{Serialize, Deserialize};
use crate::models::xml_struct::formatting::Formatting; // Импортируем вашу структуру Formatting
use crate::xml::gen_xml::WriterError;
use quick_xml::se::to_string;
use quick_xml::de::from_str;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PageContent {
    /// Метаданные полей конкретно для этой страницы (Word-style)
    #[serde(default)]
    pub formatting: Formatting,

    /// Текст страницы
    #[serde(rename = "$value", default)]
    pub text: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename = "document")]
pub struct ScenarioDocument {
    // Поля верхнего уровня (например, глобальные метаданные, если нужны)
    
    #[serde(rename = "pages")]
    pub pages: Vec<PageContent>,
}

impl ScenarioDocument {
    pub fn save_to_xml_string(&self) -> Result<String, WriterError> {
        let mut buffer = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        let xml_body = to_string(self)?;
        buffer.push_str(&xml_body);
        Ok(buffer)
    }

    pub fn load_from_xml_string(xml_data: &str) -> Result<Self, WriterError> {
        let doc: ScenarioDocument = from_str(xml_data)?;
        Ok(doc)
    }
}
