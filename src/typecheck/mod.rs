//! typechecker/mod.rs
//! 
//! The Lambda type checker walks the AST and validates type correctness.
//! It ensures consistent function signatures, valid operations,
//! proper coercions, and semantic correctness before code generation.
//! 
//! Phase order: lexer → parser → typechecker → IR/codegen
//!

use crate::parser::ast::*;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum TypeError {
    UnknownIdentifier(String),
    UnknownType(String),
    TypeMismatch { expected: String, found: String },
    BadCoercion { expected: String, found: String },
    ReturnTypeMismatch { expected: String, found: String },
    ArityMismatch { function: String, expected: usize, found: usize },
    InvalidOperand { op: String, left: String, right: String },
    NotCallable(String),
    MissingReturn(String),
    Other(String),
}

pub type TypeResult<T> = Result<T, TypeError>;

/// The typecheck environment tracks symbols and their types.
/// Each function or block has its own scope.
#[derive(Default)]
pub struct TypeContext {
    pub symbols: HashMap<String, String>, // Var -> type
    pub functions: HashMap<String, (Vec<String>, String)>, // Func -> param types -> ret type
}

impl TypeContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn declare_var(&mut self, name: &str, ty: &str) {
        self.symbols.insert(name.to_string(), ty.to_string());
    }

    pub fn get_var_type(&self, name: &str) -> Option<&String> {
        self.symbols.get(name)
    }

    pub fn declare_fn(&mut self, name: &str, params: Vec<String>, ret: String) {
        self.functions.insert(name.to_string(), (params, ret));
    }

    pub fn get_fn(&self, name: &str) -> Option<&(Vec<String>, String)> {
        self.functions.get(name)
    }

}

///////////////////////////////////////////////////////////////////////////////
// ---------------------- INTRINSICS / BUILT-IN FUNCTIONS ------------------ //
///////////////////////////////////////////////////////////////////////////////

struct Intrinsic<'a> {
    name: &'a str,
    params: Vec<&'a str>,
    ret: &'a str,
}

/// Predeclared environment with Lambda intrinsics
fn predecl_intrinsics() -> TypeContext {
    let mut ctx = TypeContext::new();

    let intrinsics = [
        // IO
        Intrinsic { name: "write", params: vec!["string"], ret: "null" },
        Intrinsic { name: "read", params: vec!["string"], ret: "string" },

        // Type Conversion
        Intrinsic { name: "to_str", params: vec!["any"], ret: "string" },
        Intrinsic { name: "to_int", params: vec!["any"], ret: "integer" },
        Intrinsic { name: "to_float", params: vec!["any"], ret: "float" },
        Intrinsic { name: "to_bool", params: vec!["any"], ret: "bool" },

        // Introspection
        Intrinsic { name: "typeof", params: vec!["any"], ret: "string" },
        Intrinsic { name: "length", params: vec!["string"], ret: "integer" },

        // Diagnostic
        Intrinsic { name: "assert", params: vec!["bool", "string"], ret: "null" },
    ];

    for i in intrinsics {
        ctx.declare_fn(i.name, i.params.iter().map(|s| s.to_string()).collect(), i.ret.into());
    }

    ctx
}

/// Typechecker entrypoint
pub fn typecheck(ast: &[Stmt]) -> TypeResult<()> {
    let mut context = predecl_intrinsics();
    
    for stmt in ast {
        check_stmt(stmt, &mut context)?;
    }

    Ok(())
}

///////////////////////////////////////////////////////////////////////////////
// -------------------------------- HELPERS -------------------------------- //
///////////////////////////////////////////////////////////////////////////////

fn check_stmt(stmt: &Stmt, ctx: &mut TypeContext) -> TypeResult<()> {
    // TODO: match arms for VarDecl, While, For, and other missing blocks
    // TODO: Also clean up some non-existent funcs like clone and type issues
    match stmt {
        Stmt::Fn { name, params, body } => {
            // For now, assume function return type = null (to be inferred later)
            ctx.declare_fn(name, vec!["any".into(); params.len()], "null".into());

            // Enter function scope
            let mut local_ctx = ctx.clone();
            for p in params {
                local_ctx.declare_var(p, "any");
            }

            for s in body {
                check_stmt(s, &mut local_ctx)?;
            }

            Ok(())
        }

        Stmt::If { condition, then_branch, else_branch } => {
            let cond_ty = infer_expr_type(condition, ctx)?;
            if cond_ty != "bool" {
                return Err(TypeError::TypeMismatch { expected: "bool".into(), found: cond_ty });
            }

            for s in then_branch {
                check_stmt(s, ctx)?;
            }
            if let Some(else_b) = else_branch {
                for s in else_b {
                    check_stmt(s, ctx)?;
                }
            }
            Ok(())
        }

        Stmt::Return(Some(expr)) => {
            infer_expr_type(expr, ctx)?; // just validate
            Ok(())
        }

        Stmt::Return(None) => Ok(()),

        Stmt::Expr(expr) => {
            infer_expr_type(expr, ctx)?;
            Ok(())
        }
    }
}

fn infer_expr_type(expr: &Expr, ctx: &TypeContext) -> TypeResult<String> {
    match expr {
        Expr::Literal(lit) => Ok(match lit {
            Literal::Int(_) => "integer".into(),
            Literal::Float(_) => "float".into(),
            Literal::Bool(_) => "bool".into(),
            Literal::String(_) => "string".into(),
            Literal::Null => "null".into(),
        }),

        Expr::Identifier(name) => {
            ctx.get_var_type(name)
                .cloned()
                .ok_or_else(|| TypeError::UnknownIdentifier(name.clone()))
        }

        Expr::Call { callee, args } => {
            if let Some((params, ret)) = ctx.get_fn(callee) {
                if params.len() != args.len() {
                    return Err(TypeError::ArityMismatch {
                        function: callee.clone(),
                        expected: params.len(),
                        found: args.len(),
                    });
                }

                for (arg, expected_ty) in args.iter().zip(params.iter()) {
                    let actual_ty = infer_expr_type(arg, ctx)?;
                    if expected_ty != "any" && expected_ty != &actual_ty {
                        return Err(TypeError::TypeMismatch {
                            expected: expected_ty.clone(),
                            found: actual_ty,
                        });
                    }
                }

                Ok(ret.clone())
            } else {
                Err(TypeError::NotCallable(callee.clone()))
            }
        }

        _ => Err(TypeError::Other("Unsupported expression form".into())),
    }
}