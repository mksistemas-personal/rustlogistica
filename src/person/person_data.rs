use serde::{Deserialize, Serialize};
use crate::Document;
use crate::person::PersonError;

#[derive(Serialize, Deserialize, Debug)]
pub struct PersonRequest {
    pub name: String,
    pub document: Document
}

impl PersonRequest {
    pub fn new(name: String, document: Document) -> Self {
        PersonRequest { name, document }
    }

    pub fn validate(&self) -> Result<(), PersonError> {
        self.validate_name()
            .and_then(|_| self.validate_document())
    }

    fn validate_name(&self) -> Result<(), PersonError> {
        if self.name.trim().is_empty() {
            Err(PersonError::EmptyName)
        } else {
            Ok(())
        }
    }

    fn validate_document(&self) -> Result<(), PersonError> {
        self.document.validate()
            .map_err(PersonError::DocumentInvalid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::base::document::DocumentType;

    #[test]
    fn test_person_request_valid() {
        let doc = Document::new(
            DocumentType::Cpf("11144477735".to_string()),
            None,
        );
        let req = PersonRequest::new("João".to_string(), doc);
        assert!(req.validate().is_ok());
    }

    #[test]
    fn test_person_request_empty_name() {
        let doc = Document::new(
            DocumentType::Cpf("11144477735".to_string()),
            None,
        );
        let req = PersonRequest::new("".to_string(), doc);
        let result = req.validate();
        assert!(result.is_err());
        match result.err().unwrap() {
            PersonError::EmptyName => (),
            _ => panic!("Esperava EmptyName"),
        }
    }

    #[test]
    fn test_person_request_invalid_document() {
        let doc = Document::new(
            DocumentType::Cpf("123".to_string()),
            None,
        );
        let req = PersonRequest::new("João".to_string(), doc);
        let result = req.validate();
        assert!(result.is_err());
        match result.err().unwrap() {
            PersonError::DocumentInvalid(_) => (),
            _ => panic!("Esperava DocumentInvalid"),
        }
    }
}