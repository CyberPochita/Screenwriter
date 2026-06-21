use serde::{Serialize, Deserialize};

use quick_xml::se::to_string;
use quick_xml::de::from_str;
use quick_xml::errors::serialize::DeError;
use quick_xml::errors::serialize::SeError;

use crate::models::structure_xml::TitlePage;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WriterError {
    pub message: String,
    pub code: String,
}

impl std::fmt::Display for WriterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]: {}", self.code, self.message)
    }
}

impl std::error::Error for WriterError {}

impl From<DeError> for WriterError {
    fn from(err: DeError) -> Self {
        WriterError {
            message: err.to_string(),
            code: "XML_PROCESSING_ERROR".to_string(),
        }
    }
}

impl From<SeError> for WriterError {
    fn from(err: SeError) -> Self {
        WriterError {
            message: err.to_string(),
            code: "XML_SERIALIZATION_ERROR".to_string(),
        }
    }
}

impl TitlePage {
    /// Сериализует структуру титульной страницы в XML-строку.
    /// Безопасно обрабатывает спецсимволы и возвращает WriterError при сбое.
    pub fn save_to_xml_string(&self) -> Result<String, WriterError> {
        let mut buffer = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        
        // Благодаря impl From теперь работает оператор `?`
        let xml_body = to_string(self)?;
        
        buffer.push_str(&xml_body);
        Ok(buffer)
    }

    /// Восстанавливает структуру TitlePage из XML-строки файла .writer.
    /// Возвращает валидированную структуру или WriterError для фронтенда.
    pub fn load_from_xml_string(xml_data: &str) -> Result<Self, WriterError> {
        let page: TitlePage = from_str(xml_data)?;
        Ok(page)
    }
}
