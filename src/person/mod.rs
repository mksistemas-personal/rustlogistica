pub mod person_data;
pub mod person_controller;
pub mod person_routes;

use crate::base::document::{Document, DocumentError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PersonError {
    #[error("person.name.empty")]
    EmptyName,
    #[error("person.document.invalid.{0}")]
    DocumentInvalid(#[from] DocumentError),
}

pub struct Person {
    pub name: String,
    pub document: Document
}


