use std::default::Default;
use std::fmt;

use crate::ast::statement::BlockStatement;

#[derive(Debug, Clone, PartialEq)]
pub struct ProcParam {
    pub param_name: String,
    pub param_type: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProcedureStmt {
    pub id: Option<u64>,
    pub name: String,
    pub params: Vec<ProcParam>,
    pub return_type: String,
    pub block: BlockStatement,
}

impl ProcedureStmt {
    pub fn new(name: String) -> Self {
        Self {
            name,
            id: None,
            params: Default::default(),
            return_type: "".to_string(),
            block: BlockStatement::new(),
        }
    }
}
