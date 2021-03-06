use crate::ast::expression::ExpressionType;
use crate::ast::semantic::SymbolId;

#[derive(Debug, Clone, PartialEq)]
pub struct Procedure {
    pub id: SymbolId,
    pub name: String,
    pub params_types: Vec<ExpressionType>,
    pub return_type: ExpressionType,
}

impl Procedure {
    pub fn new(name: &str, id: SymbolId) -> Self {
        Self {
            id,
            name: name.to_owned(),
            params_types: Vec::new(),
            return_type: ExpressionType::Unit,
        }
    }
}
