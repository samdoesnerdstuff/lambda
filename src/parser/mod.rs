//! Lambda Parser Module
//!

pub mod ast;

use crate::lexer::{Token, Span};
use ast::*;
pub struct Parser {
    tokens: Vec<(Token, Span)>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<(Token, Span)>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            if let Some(stmt) = self.declaration() {
                statements.push(stmt);
            } else {
                println!("Bad token Found");
                self.advance();
            }
        }

        statements
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

    fn declaration(&mut self) -> Option<Stmt> {
        match self.peek().0 {
            Token::Fn => self.fn_declaration(),
            _ => self.statement(),
        }
    }

    fn fn_declaration(&mut self) -> Option<Stmt> {
        self.advance(); // consume 'fn'

        let name = if let Token::Identifier(ref n) = self.advance().0 {
            n.clone()
        } else {
            eprintln!("Expected function name after 'fn'");
            return None;
        };

        self.advance(); // '('
        let mut params = Vec::new();

        while let Token::Identifier(ref p) = self.peek().0 {
            if let Token::Identifier(param) = self.advance().0.clone() {
                params.push(param);
            }
        }

        self.advance(); // ')'

        let mut body = Vec::new();
        while !matches!(self.peek().0, Token::End | Token::EOF) {
            if let Some(stmt) = self.statement() {
                body.push(stmt);
            } else {
                self.advance();
            }
        }

        if self.match_token(&[Token::End]) {
            Some(Stmt::Fn { name, params, body })
        } else {
            eprintln!("Expected 'end' after function body");
            None
        }
    }

    fn statement(&mut self) -> Option<Stmt> {
        match self.peek().0 {
            Token::Return => self.return_statement(),
            Token::If => self.if_statement(),
            _ => self.expression_statement(),
        }
    }

    fn expression_statement(&mut self) -> Option<Stmt> {
        let expr = self.expression()?;
        Some(Stmt::Expr(expr))
    }

    fn return_statement(&mut self) -> Option<Stmt> {
        self.advance(); // consume 'return'

        let expr = if !matches!(self.peek().0, Token::End | Token::EOF) {
            Some(self.expression()?)
        } else {
            None
        };

        Some(Stmt::Return(expr))
    }

    fn if_statement(&mut self) -> Option<Stmt> {
        self.advance(); // consume 'if'
        let condition = self.expression()?;

        let mut then_branch = Vec::new();
        let mut else_branch = None;

        // Parse then-block
        while !matches!(self.peek().0, Token::Else | Token::End | Token::EOF) {
            if let Some(stmt) = self.statement() {
                then_branch.push(stmt);
            } else {
                self.advance();
            }
        }

        // Optional else-block
        if self.match_token(&[Token::Else]) {
            let mut else_stmts = Vec::new();
            while !matches!(self.peek().0, Token::End | Token::EOF) {
                if let Some(stmt) = self.statement() {
                    else_stmts.push(stmt);
                } else {
                    self.advance();
                }
            }
            else_branch = Some(else_stmts);
        }

        if !self.match_token(&[Token::End]) {
            eprintln!("Expected 'end' after if/else block");
        }

        Some(Stmt::If { condition, then_branch, else_branch })
    }

    fn expression(&mut self) -> Option<Expr> {
        match &self.peek().0 {
            Token::Integer(n) => {
                let val = *n;
                self.advance();
                Some(Expr::Literal(Literal::Int(val)))
            }
            Token::Float(f) => {
                let val = *f;
                self.advance();
                Some(Expr::Literal(Literal::Float(val)))
            }
            Token::Bool(b) => {
                let val = *b;
                self.advance();
                Some(Expr::Literal(Literal::Bool(val)))
            }
            Token::String(s) => {
                let val = s.clone();
                self.advance();
                Some(Expr::Literal(Literal::String(val)))
            }
            Token::Null => {
                self.advance();
                Some(Expr::Literal(Literal::Null))
            }
            Token::Identifier(name) => {
                let id = name.clone();
                self.advance();
                Some(Expr::Identifier(id))
            }
            _ => None,
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
        let tokens = lex(src);
        let mut parser = Parser::new(tokens);
        let ast = parser.parse();

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
        let tokens = lex(src);
        let mut parser = Parser::new(tokens);
        let ast = parser.parse();

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
        let tokens = lex(src);
        let mut parser = Parser::new(tokens);
        let ast = parser.parse();

        assert_eq!(ast.len(), 1);
        match &ast[0] {
            Stmt::Return(Some(Expr::Literal(Literal::Int(123)))) => {}
            _ => panic!("Expected return with integer literal"),
        }
    }
}