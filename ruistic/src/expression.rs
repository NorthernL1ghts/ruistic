// Copyright (c) 2025 NorthernL1ghts
// This file is part of Ruistic, a custom programming language interpreter.
// See LICENSE file for license information.

use crate::token::{Token, Value};

#[derive(Debug)]
pub enum Expr {
    Literal(Value),
    Unary { operator: Token, right: Box<Expr> },
    Binary { left: Box<Expr>, operator: Token, right: Box<Expr> },
    Grouping(Box<Expr>),
    Variable(Token),
    Assign { name: Token, value: Box<Expr> },
}
