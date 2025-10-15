//! Lambda Parser Module
//!

pub mod ast;

use crate::lexer::{Token, Span};
use ast::*;

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken(String),
    MissingEndBlock,
    MissingReturnValue,
    MissingTypeAnnotation,
    UnexpectedEOF,
    DuplicateParameter(String),
    ExpectedFunctionName,

    TypeMismatch { expected: String, found: String},
    BadTypeCoercion { expected: String, found: String },
    UnknownType(String),
    TypeInferenceConflict(String),
    ReturnTypeMismatch { expected: String, found: String },

    DivideByZero,
}

pub struct Parser {
    tokens: Vec<(Token, Span)>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<(Token, Span)>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            let stmt = self.declaration()?;
            statements.push(stmt);
        }

        Ok(statements)
    }

    //
    // Core Parser Utilities
    //

    fn is_at_end(&self) -> bool {
        matches!(self.peek().0, Token::EOF)
    }

    fn peek(&self) -> &(Token, Span) {
        &self.tokens[self.current]
    }

    fn advance(&mut self) -> &(Token, Span) {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn previous(&self) -> &(Token, Span) {
        &self.tokens[self.current.saturating_sub(1)]
    }

    fn match_token(&mut self, expected: &[Token]) -> bool {
        for tok in expected {
            if self.check(tok) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token_type: &Token) -> bool {
        if self.is_at_end() {
            return false;
        }
        std::mem::discriminant(&self.peek().0) == std::mem::discriminant(token_type)
    }

    //
    // High-level parse rules
    //

    fn declaration(&mut self) -> Result<Stmt, ParseError> {
        match self.peek().0 {
            Token::Fn => self.fn_declaration(),
            _ => self.statement(),
        }
    }

    fn fn_declaration(&mut self) -> Result<Stmt, ParseError> {
       self.advance();

       let name = match &self.advance().0 {
            Token::Identifier(n) => n.clone(),
            _ => return Err(ParseError::ExpectedFunctionName),
        };

        if !self.match_token(&[Token::LParen]) {
            return Err(ParseError::UnexpectedToken("Expected '(' after function name".into()));
        }

        let mut params = Vec::new();

        while let Token::Identifier(ref _p) = self.peek().0 {
            if let Token::Identifier(param) = self.advance().0.clone() {
                if params.contains(&param) {
                    return Err(ParseError::DuplicateParameter(param));
                }
                params.push(param);
            }
        }

        if !self.match_token(&[Token::RParen]) {
            return Err(ParseError::UnexpectedToken("Expected ')' after parameters".into()));
        }

        let mut body = Vec::new();
        while !matches!(self.peek().0, Token::End | Token::EOF) {
            body.push(self.statement()?);
        }

        if !self.match_token(&[Token::End]) {
            return Err(ParseError::MissingEndBlock);
        }

        Ok(Stmt::Fn { name, params, body })
    }

    fn statement(&mut self) -> Result<Stmt, ParseError> {
        match self.peek().0 {
            Token::Return => self.return_statement(),
            Token::If => self.if_statement(),
            _ => self.expression_statement(),
        }
    }

    fn expression_statement(&mut self) -> Result<Stmt, ParseError> {
        let expr = self.expression()?;
        Ok(Stmt::Expr(expr))
    }

    fn return_statement(&mut self) -> Result<Stmt, ParseError> {
        self.advance(); // consume 'return'

        let expr = if !matches!(self.peek().0, Token::End | Token::EOF) {
            Some(self.expression()?)
        } else {
            None
        };

        Ok(Stmt::Return(expr))
    }

    fn if_statement(&mut self) -> Result<Stmt, ParseError> {
        self.advance(); // consume 'if'
        let condition = self.expression()?;
        let mut then_branch = Vec::new();

        while !matches!(self.peek().0, Token::Else | Token::End | Token::EOF) {
            then_branch.push(self.statement()?);
        }

        let else_branch = if self.match_token(&[Token::Else]) {
            let mut stmts = Vec::new();
            while !matches!(self.peek().0, Token::End | Token::EOF) {
                stmts.push(self.statement()?)   
            }
            Some(stmts)
        } else {
            None
        };

        if !self.match_token(&[Token::End]) {
            return Err(ParseError::MissingEndBlock);
        }
 
        Ok(Stmt::If { condition, then_branch, else_branch })
    }

    fn expression(&mut self) -> Result<Expr, ParseError> {
        match &self.peek().0 {
            Token::Integer(n) => {
                let val = *n;
                self.advance();
                Ok(Expr::Literal(Literal::Int(val)))
            }
            Token::Float(f) => {
                let val = *f;
                self.advance();
                Ok(Expr::Literal(Literal::Float(val)))
            }
            Token::Bool(b) => {
                let val = *b;
                self.advance();
                Ok(Expr::Literal(Literal::Bool(val)))
            }
            Token::String(s) => {
                let val = s.clone();
                self.advance();
                Ok(Expr::Literal(Literal::String(val)))
            }
            Token::Null => {
                self.advance();
                Ok(Expr::Literal(Literal::Null))
            }
            Token::Identifier(name) => {
                let id = name.clone();
                self.advance();
                Ok(Expr::Identifier(id))
            }
            Token::Slash => {
                self.advance();
                let rhs = self.expression()?;
                if let Expr::Literal(Literal::Int(0)) = rhs {
                    Err(ParseError::DivideByZero)
                } else {
                    Ok(Expr::Binary {
                        left: Box::new(Expr::Literal(Literal::Int(1))),
                        op: Operator::Slash,
                        right: Box::new(rhs),
                    })
                }
            }
            _ => Err(ParseError::UnexpectedToken(format!(
                "Unexpected token: {:?} at {:?}",
                self.peek().0, self.peek().1
            ))),
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
// --------------------------- !! PARSER TESTS !! -------------------------- //
///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::lex;

    #[test]
    fn parse_simple_fn() {
        let src = "fn greet() end";
        let tokens = lex(src, false);
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        assert_eq!(ast.len(), 1);
        match &ast[0] {
            Stmt::Fn { name, params, body } => {
                assert_eq!(name, "greet");
                assert!(params.is_empty());
                assert!(body.is_empty());
            }
            _ => panic!("Expected function declaration"),
        }
    }

    #[test]
    fn parse_if_statement() {
        let src = "if true return 42 end";
        let tokens = lex(src, false);
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        assert_eq!(ast.len(), 1);
        match &ast[0] {
            Stmt::If { condition, then_branch, else_branch } => {
                assert!(else_branch.is_none());
                assert!(matches!(condition, Expr::Literal(Literal::Bool(true))));
                assert!(matches!(&then_branch[0], Stmt::Return(Some(Expr::Literal(Literal::Int(42))))));
            }
            _ => panic!("Expected if statement"),
        }
    }

    #[test]
    fn parse_return_statement() {
        let src = "return 123";
        let tokens = lex(src, false);
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        assert_eq!(ast.len(), 1);
        match &ast[0] {
            Stmt::Return(Some(Expr::Literal(Literal::Int(123)))) => {}
            _ => panic!("Expected return with integer literal"),
        }
    }
}