use serde::{Serialize, Deserialize};
use serde_json;
use logistica::Document;
use logistica::DocumentType;

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u8,
    is_active: bool,
    hobbies: Vec<String>,
}

fn main() {
    // --- Exemplo com Person ---
    println!("--- Exemplo com Person ---");
    let person = Person {
        name: "João Silva".to_string(),
        age: 30,
        is_active: true,
        hobbies: vec!["Codar".to_string(), "Ler".to_string(), "Caminhar".to_string()],
    };

    let json_string = serde_json::to_string(&person).expect("Falha ao serializar Person");
    println!("JSON Person (compacto): {}", json_string);

    // --- Exemplo com Document (CPF) ---
    println!("\n--- Exemplo com Document (CPF) ---");
    let doc_cpf = Document::new(
        DocumentType::Cpf("11144477735".to_string()),
        Some("CPF do João".to_string()),
    );

    let json_cpf = serde_json::to_string_pretty(&doc_cpf).expect("Falha ao serializar Document CPF");
    println!("JSON Document CPF:\n{}", json_cpf);

    let doc_cpf_back: Document = serde_json::from_str(&json_cpf).expect("Falha ao desserializar Document CPF");
    println!("Objeto CPF restaurado: {:?} (Válido: {})", doc_cpf_back, doc_cpf_back.validate().is_ok());

    // --- Exemplo com Document (CNPJ) ---
    println!("\n--- Exemplo com Document (CNPJ) ---");
    let doc_cnpj = Document::new(
        DocumentType::Cnpj("11222333000181".to_string()),
        None,
    );

    let json_cnpj = serde_json::to_string_pretty(&doc_cnpj).expect("Falha ao serializar Document CNPJ");
    println!("JSON Document CNPJ:\n{}", json_cnpj);

    let doc_cnpj_back: Document = serde_json::from_str(&json_cnpj).expect("Falha ao desserializar Document CNPJ");
    println!("Objeto CNPJ restaurado: {:?} (Válido: {})", doc_cnpj_back, doc_cnpj_back.validate().is_ok());

    // --- Exemplo com Document (Nulo) ---
    println!("\n--- Exemplo com Document (Descrição Nula) ---");
    let doc_null = Document::new(
        DocumentType::Cpf("52998224725".to_string()),
        None,
    );

    let json_null = serde_json::to_string_pretty(&doc_null).expect("Falha ao serializar Document Nulo");
    println!("JSON Document Nulo:\n{}", json_null);

    let doc_null_back: Document = serde_json::from_str(&json_null).expect("Falha ao desserializar Document Nulo");
    println!("Objeto Nulo restaurado: {:?} (Válido: {})", doc_null_back, doc_null_back.validate().is_ok());
}
