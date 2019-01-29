use crate::ast::direction::Direction;
use crate::ast::program::Program;
use crate::ast::statement::{
    BlockStatement, CommandStmt, DirectionStmt, Expression, IfStmt, LocalStmt, MakeStmt,
    ProcedureStmt, RepeatStmt, ShowExpr, Statement, Symbol,
};
use crate::lexer::{location::Location, simple_lexer::SimpleLexer, token::Token, Lexer};
use crate::parser::{Parser, ParserResult};

struct ProgramParser;

impl ProgramParser {
    fn new() -> Self {
        Self {}
    }
}

impl Parser for ProgramParser {
    fn parse(&mut self, code: &str) -> ParserResult {
        let mut parser = Self::new();
        let mut lexer = SimpleLexer::new(code);

        let program = parser.parse_program(&mut lexer);
        Ok(program)
    }
}

impl ProgramParser {
    fn parse_program(&mut self, lexer: &mut impl Lexer) -> Program {
        let mut program = Program::default();

        while let Some(stmt) = Self::parse_statement(lexer) {
            match stmt {
                Statement::Nop => continue,
                _ => program.statements.push(stmt),
            }
        }

        program
    }

    fn parse_statement(lexer: &mut impl Lexer) -> Option<Statement> {
        let tok_loc = Self::peek_current_token(lexer);
        if tok_loc.is_none() {
            return None;
        }

        let (token, location) = tok_loc.unwrap();

        match token {
            Token::EOF => return None,
            Token::NEWLINE => {
                Self::skip_token(lexer);
                return Some(Statement::Nop);
            }
            Token::VALUE(val) => match val.as_str() {
                "REPEAT" => {
                    unimplemented!();
                }
                "IF" => {
                    unimplemented!();
                }
                "TO" => {
                    unimplemented!();
                }
                _ => Self::parse_basic_statement(val.clone(), lexer),
            },
            _ => panic!(),
        }
    }

    fn parse_basic_statement(val: String, lexer: &mut impl Lexer) -> Option<Statement> {
        let val = val.as_str();

        let stmt = match val {
            "MAKE" => Self::parse_make(lexer),
            "FORWARD" | "BACKWARD" | "RIGHT" | "LEFT" => Self::parse_direction(val, lexer),
            _ => {
                unimplemented!();
            }
        };

        Some(stmt)
    }

    fn parse_make(lexer: &mut impl Lexer) -> Statement {
        Self::skip_token(lexer); // skipping the `MAKE` token

        let name = Self::expect_ident(lexer);
        let symbol = Symbol { name };

        Self::expect_token(lexer, Token::ASSIGN);

        let expr = Self::parse_expr(lexer);

        let stmt = MakeStmt {
            symbol,
            expr: Box::new(expr),
        };
        Statement::Make(stmt)
    }

    fn parse_direction(direction: &str, lexer: &mut impl Lexer) -> Statement {
        // skipping the direction token
        // we already have the value under `direction`
        Self::skip_token(lexer);

        let distance_expr = Self::parse_expr(lexer);

        Self::expect_newline(lexer);

        let stmt = DirectionStmt {
            distance_expr,
            direction: Direction::from(direction),
        };

        Statement::Direction(stmt)
    }

    fn parse_command(&mut self, val: &str, lexer: &mut impl Lexer) -> CommandStmt {
        match val {
            "PENUP" => CommandStmt::PenUp,
            "PENDOWN" => CommandStmt::PenDown,
            "SHOWTURTLE" => CommandStmt::ShowTurtle,
            "HIDETURTLE" => CommandStmt::HideTurtle,
            _ => panic!(),
        }
    }

    fn parse_expr(lexer: &mut impl Lexer) -> Expression {
        let left_expr = Self::parse_mul_expr(lexer);

        let (tok, loc) = Self::peek_current_token(lexer).unwrap();

        match tok {
            Token::ADD => {
                Self::skip_token(lexer); // we skip the `+` token
                let right_expr = Self::parse_expr(lexer);
                Expression::Add(Box::new(left_expr), Box::new(right_expr))
            }
            Token::MUL => {
                Self::skip_token(lexer); // we skip the `*` token
                let right_expr = Self::parse_expr(lexer);
                Expression::Mul(Box::new(left_expr), Box::new(right_expr))
            }
            // Token::LPAREN => {
            //     Self::skip_token(lexer); // we skip the `(` token
            //     let expr = Self::parse_expr(lexer);
            //     Self::expect_token(lexer, Token::RPAREN); // we expect `)` token
            //
            //     expr
            // }
            _ => left_expr,
        }
    }

    fn parse_mul_expr(lexer: &mut impl Lexer) -> Expression {
        let num = Self::expect_number(lexer);

        Expression::Int(num)
    }

    fn expect_number(lexer: &mut impl Lexer) -> usize {
        let pair = Self::pop_current_token(lexer);

        let (tok, loc) = pair.unwrap();

        match tok {
            Token::EOF | Token::NEWLINE => panic!("unexpected..."),
            Token::VALUE(v) => v.parse::<usize>().unwrap(),
            _ => panic!(),
        }
    }

    fn expect_newline(lexer: &mut impl Lexer) {
        let tok_loc = Self::pop_current_token(lexer);

        if tok_loc.is_some() {
            let (tok, loc) = tok_loc.unwrap();

            match tok {
                Token::EOF | Token::NEWLINE => return,
                _ => panic!("invalid input"),
            }
        }
    }

    fn expect_ident(lexer: &mut impl Lexer) -> String {
        let (token, loc) = Self::pop_current_token(lexer).unwrap();

        if let Token::VALUE(v) = token {
            return v;
        } else {
            panic!("Expected an identifier");
        }
    }

    fn expect_token(lexer: &mut impl Lexer, expected: Token) {
        let (actual, loc) = Self::pop_current_token(lexer).unwrap();

        assert_eq!(actual, expected);
    }

    fn peek_current_token(lexer: &impl Lexer) -> Option<&(Token, Location)> {
        lexer.peek_current_token()
    }

    fn skip_token(lexer: &mut impl Lexer) {
        Self::pop_current_token(lexer);
    }

    fn pop_current_token(lexer: &mut impl Lexer) -> Option<(Token, Location)> {
        lexer.pop_current_token()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forward() {
        let actual = ProgramParser.parse("FORWARD 20").unwrap();

        let expected = Program {
            statements: vec![Statement::Direction(DirectionStmt {
                direction: Direction::Forward,
                distance_expr: Expression::Int(20),
            })],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn backward() {
        let actual = ProgramParser.parse("BACKWARD 20").unwrap();

        let expected = Program {
            statements: vec![Statement::Direction(DirectionStmt {
                direction: Direction::Backward,
                distance_expr: Expression::Int(20),
            })],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn left() {
        let actual = ProgramParser.parse("LEFT 20").unwrap();

        let expected = Program {
            statements: vec![Statement::Direction(DirectionStmt {
                direction: Direction::Left,
                distance_expr: Expression::Int(20),
            })],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn right() {
        let actual = ProgramParser.parse("RIGHT 20").unwrap();

        let expected = Program {
            statements: vec![Statement::Direction(DirectionStmt {
                direction: Direction::Right,
                distance_expr: Expression::Int(20),
            })],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn forward_and_then_backward_no_empty_lines() {
        let actual = ProgramParser.parse("FORWARD 10\nRIGHT 20").unwrap();

        let expected = Program {
            statements: vec![
                Statement::Direction(DirectionStmt {
                    direction: Direction::Forward,
                    distance_expr: Expression::Int(10),
                }),
                Statement::Direction(DirectionStmt {
                    direction: Direction::Right,
                    distance_expr: Expression::Int(20),
                }),
            ],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn forward_and_then_backward_with_empty_lines() {
        let actual = ProgramParser
            .parse("\n\nFORWARD 10\n\nRIGHT 20\n\n")
            .unwrap();

        let expected = Program {
            statements: vec![
                Statement::Direction(DirectionStmt {
                    direction: Direction::Forward,
                    distance_expr: Expression::Int(10),
                }),
                Statement::Direction(DirectionStmt {
                    direction: Direction::Right,
                    distance_expr: Expression::Int(20),
                }),
            ],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    #[ignore]
    fn forward_only_integer_expr_surrounded_by_parentheses() {
        let actual = ProgramParser.parse("FORWARD (10)").unwrap();

        let expected = Program {
            statements: vec![Statement::Direction(DirectionStmt {
                direction: Direction::Forward,
                distance_expr: Expression::Int(10),
            })],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn forward_only_add_integers_expr_with_spaces() {
        let actual = ProgramParser.parse("FORWARD 1 + 2").unwrap();

        let expected = Program {
            statements: vec![Statement::Direction(DirectionStmt {
                direction: Direction::Forward,
                distance_expr: Expression::Add(
                    Box::new(Expression::Int(1)),
                    Box::new(Expression::Int(2)),
                ),
            })],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn forward_only_add_integers_expr_without_spaces() {
        let actual = ProgramParser.parse("FORWARD 1 + 2").unwrap();

        let expected = Program {
            statements: vec![Statement::Direction(DirectionStmt {
                direction: Direction::Forward,
                distance_expr: Expression::Add(
                    Box::new(Expression::Int(1)),
                    Box::new(Expression::Int(2)),
                ),
            })],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn forward_only_mul_integers_expr_without_spaces() {
        let actual = ProgramParser.parse("FORWARD 1 * 2").unwrap();

        let expected = Program {
            statements: vec![Statement::Direction(DirectionStmt {
                direction: Direction::Forward,
                distance_expr: Expression::Mul(
                    Box::new(Expression::Int(1)),
                    Box::new(Expression::Int(2)),
                ),
            })],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn make_variable_assign_an_integer() {
        let actual = ProgramParser.parse("MAKE MyVar = 2").unwrap();

        let expected = Program {
            statements: vec![Statement::Make(MakeStmt {
                symbol: Symbol {
                    name: "MyVar".to_string(),
                },
                expr: Box::new(Expression::Int(2)),
            })],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn make_variable_assign_an_expr() {
        let actual = ProgramParser.parse("MAKE MyVar = 1 + 2").unwrap();

        let expected_expr =
            Expression::Add(Box::new(Expression::Int(1)), Box::new(Expression::Int(2)));

        let expected = Program {
            statements: vec![Statement::Make(MakeStmt {
                symbol: Symbol {
                    name: "MyVar".to_string(),
                },
                expr: Box::new(expected_expr),
            })],
        };

        assert_eq!(actual, expected);
    }
}
