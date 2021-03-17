


use crate::model::rho_types;
use super::super::bnfc;
use super::super::errors::*;
use super::*;

impl super::Normalizer {

    pub fn normalize_name(&mut self, n : &RhoName,  input: &NameVisitInputs) -> Option<NameVisitOutputs> {
        
        match n.kind {
            bnfc::Name__is_NameWildcard => {

            },
            bnfc::Name__is_NameVar => {

            },
            bnfc::Name__is_NameQuote => {

            },
            _ => {
                self.faulty_errors.push(CompliationError::UnrecognizedKind(n.kind, "bnfc::Name".to_string()));
            }
        };

 
        None
    }
    
}