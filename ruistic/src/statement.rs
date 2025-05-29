// Copyright (c) 2025 NorthernL1ghts
// This file is part of Ruistic, a custom programming language interpreter.
// See LICENSE file for license information.

use crate::expression::Expr;
use crate::token::*;
#[derive(Debug)]
pub enum Stmt {
    Expr(Expr),
    Print(Expr),
    Var(Token, Option<Expr>),
    Block(Vec<Stmt>),
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    While {
        condition: Expr,
        body: Box<Stmt>,
    },
}
