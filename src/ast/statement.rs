use crate::ast::direction::Direction;
use crate::ir::instruction::Instruction;
use crate::lexer::location::Location;

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Int(usize),
    Add(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ShowExpr {
    pub varname: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OutputExpr {}

#[derive(Debug, Clone, PartialEq)]
pub struct ProcedureStmt {
    pub loction: Option<Location>,
    pub name: String,
    pub block: BlockStatement,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SymbolKind {
    Int,
    String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Symbol {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MakeStmt {
    pub symbol: Symbol,
    pub expr: Box<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LocalStmt {
    pub symbol: SymbolKind,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RepeatStmt {
    pub count_expr: Expression,
    pub block: BlockStatement,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectionStmt {
    pub direction: Direction,
    pub distance_expr: Expression,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfStmt {
    pub cond_expr: Box<Expression>,
    pub true_block: BlockStatement,
    pub false_block: Option<BlockStatement>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CommandStmt {
    PenUp,
    PenDown,
    ShowTurtle,
    HideTurtle,
    PenErase,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockStatement {
    pub stmts: Vec<Statement>,
}

impl Default for BlockStatement {
    fn default() -> Self {
        Self {
            stmts: Default::default(),
        }
    }
}

impl BlockStatement {
    pub fn add_statement(&mut self, stmt: Statement) {
        self.stmts.push(stmt);
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Command(CommandStmt),
    Direction(DirectionStmt),
    Repeat(RepeatStmt),
    If(IfStmt),
    Make(MakeStmt),
    Procedure(ProcedureStmt),
    Nop,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn int_expr_sanity() {
        let expr1 = Expression::Int(10);
        let expr2 = Expression::Int(10);
        let expr3 = Expression::Int(20);

        assert_eq!(expr1, expr2);
        assert_ne!(expr1, expr3);
    }
}