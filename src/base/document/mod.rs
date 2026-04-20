use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DocumentError {
    #[error("person.cpf.invalid")]
    InvalidCpf,
    #[error("Cperson.cnpj.invalid")]
    InvalidCnpj,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum DocumentType {
    Cnpj,
    Cpf,
}

impl DocumentType {
    pub fn validate(&self, identifier: &str) -> Result<(), DocumentError> {
        match self {
            DocumentType::Cnpj => {
                if validate_cnpj(identifier) {
                    Ok(())
                } else {
                    Err(DocumentError::InvalidCnpj)
                }
            }
            DocumentType::Cpf => {
                if validate_cpf(identifier) {
                    Ok(())
                } else {
                    Err(DocumentError::InvalidCpf)
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Document {
    #[serde(rename = "type")]
    pub doc_type: DocumentType,
    #[serde(rename = "identifier")]
    pub identifier: String,
    #[serde(rename = "description")]
    pub description: Option<String>,
}

impl Document {
    pub fn new(doc_type: DocumentType, identifier: String, description: Option<String>) -> Self {
        Self {
            doc_type,
            identifier,
            description,
        }
    }

    pub fn validate(&self) -> Result<(), DocumentError> {
        self.doc_type.validate(&self.identifier)
    }
}

fn validate_cpf(cpf: &str) -> bool {
    let digits: Vec<u32> = cpf.chars().filter_map(|c| c.to_digit(10)).collect();
    if digits.len() != 11 {
        return false;
    }

    if digits.iter().all(|&x| x == digits[0]) {
        return false;
    }

    let sum1: u32 = digits[0..9]
        .iter()
        .enumerate()
        .map(|(i, &d)| d * (10 - i as u32))
        .sum();
    let rem1 = (sum1 * 10) % 11;
    let digit1 = if rem1 == 10 { 0 } else { rem1 };

    if digit1 != digits[9] {
        return false;
    }

    let sum2: u32 = digits[0..10]
        .iter()
        .enumerate()
        .map(|(i, &d)| d * (11 - i as u32))
        .sum();
    let rem2 = (sum2 * 10) % 11;
    let digit2 = if rem2 == 10 { 0 } else { rem2 };

    digit2 == digits[10]
}

fn validate_cnpj(cnpj: &str) -> bool {
    let digits: Vec<u32> = cnpj.chars().filter_map(|c| c.to_digit(10)).collect();
    if digits.len() != 14 {
        return false;
    }

    if digits.iter().all(|&x| x == digits[0]) {
        return false;
    }

    let weights1 = [5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
    let sum1: u32 = digits[0..12]
        .iter()
        .zip(weights1.iter())
        .map(|(&d, &w)| d * w)
        .sum();
    let rem1 = sum1 % 11;
    let digit1 = if rem1 < 2 { 0 } else { 11 - rem1 };

    if digit1 != digits[12] {
        return false;
    }

    let weights2 = [6, 5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
    let sum2: u32 = digits[0..13]
        .iter()
        .zip(weights2.iter())
        .map(|(&d, &w)| d * w)
        .sum();
    let rem2 = sum2 % 11;
    let digit2 = if rem2 < 2 { 0 } else { 11 - rem2 };

    digit2 == digits[13]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_cpf() {
        assert!(validate_cpf("11144477735"));
        assert!(validate_cpf("52998224725"));
    }

    #[test]
    fn test_invalid_cpf() {
        assert!(!validate_cpf("11111111111"));
        assert!(!validate_cpf("12345678901"));
        assert!(!validate_cpf("123"));
    }

    #[test]
    fn test_valid_cnpj() {
        assert!(validate_cnpj("11222333000181"));
    }

    #[test]
    fn test_invalid_cnpj() {
        assert!(!validate_cnpj("00000000000000"));
        assert!(!validate_cnpj("12345678901234"));
    }

    #[test]
    fn test_document_struct() {
        let doc = Document::new(
            DocumentType::Cpf,
            "11144477735".to_string(),
            Some("CPF do João".to_string()),
        );
        assert!(doc.validate().is_ok());
        assert_eq!(doc.description, Some("CPF do João".to_string()));
    }

    #[test]
    fn test_document_struct_null_description() {
        let doc = Document::new(
            DocumentType::Cpf,
            "11144477735".to_string(),
            None,
        );
        assert!(doc.validate().is_ok());
        assert!(doc.description.is_none());
    }

    #[test]
    fn test_serialization() {
        let doc = Document::new(
            DocumentType::Cpf,
            "11144477735".to_string(),
            Some("Teste de serialização".to_string()),
        );
        let json = serde_json::to_string(&doc).expect("Falha ao serializar");
        println!("JSON Gerado: {}", json);
        
        // O JSON do DocumentType será {"type":"Cpf"}
        // O JSON do Document será {"doc_type":{"type":"Cpf"},"identifier":"...","description":"..."}
        
        assert!(json.contains("\"type\":\"Cpf\""));
        assert!(json.contains("\"identifier\":\"11144477735\""));
        
        let doc_back: Document = serde_json::from_str(&json).expect("Falha ao desserializar");
        assert!(doc_back.validate().is_ok());
    }
}
