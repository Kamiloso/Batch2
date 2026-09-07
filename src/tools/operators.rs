use std::collections::HashMap;
use std::sync::LazyLock;
use std::string::ToString;
use crate::tools::chars;
use crate::error::RuntimeResult;
use crate::types::{ShapeParse, ShapeStringify, NumberShape, BoolShape};

pub fn get_single_fn(op: &str) -> Option<fn(&str) -> Option<String>> {
    match op {
        OP_NOT => Some(not),
        OP_DEFINED => Some(defined),
        _ => None
    }
}

pub fn get_double_fn(op: &str) -> Option<fn(&str, &str) -> Option<String>> {
    match op {
        OP_ADD => Some(add),
        OP_DIV => Some(subtract),
        OP_MUL => Some(multiply),
        OP_DIV => Some(divide),
        OP_OR => Some(or),
        OP_AND => Some(and),
        OP_EQUALS => Some(equals),
        OP_NOT_EQUALS => Some(not_equals),
        _ => None
    }
}

pub const OP_ADD: &str = "+";
fn add(a: &str, b: &str) -> Option<String> {
    let a = NumberShape::parse(a)?;
    let b = NumberShape::parse(b)?;

    Some(NumberShape::stringify(a + b))
}

pub const OP_SUB: &str = "-";
fn subtract(a: &str, b: &str) -> Option<String> {
    let a = NumberShape::parse(a)?;
    let b = NumberShape::parse(b)?;

    Some(NumberShape::stringify(a - b))
}

pub const OP_MUL: &str = "*";
fn multiply(a: &str, b: &str) -> Option<String> {
    let a = NumberShape::parse(a)?;
    let b = NumberShape::parse(b)?;

    Some(NumberShape::stringify(a * b))
}

pub const OP_DIV: &str = "/";
fn divide(a: &str, b: &str) -> Option<String> {
    let a = NumberShape::parse(a)?;
    let b = NumberShape::parse(b)?;

    Some(NumberShape::stringify(a / b))
}

pub const OP_OR: &str = "or";
fn or(a: &str, b: &str) -> Option<String> {
    let a = BoolShape::parse(a)?;
    let b = BoolShape::parse(b)?;
    
    Some(BoolShape::stringify(a || b))
}

pub const OP_AND: &str = "and";
fn and(a: &str, b: &str) -> Option<String> {
    let a = BoolShape::parse(a)?;
    let b = BoolShape::parse(b)?;
    
    Some(BoolShape::stringify(a && b))
}

pub const OP_EQUALS: &str = "==";
fn equals(a: &str, b: &str) -> Option<String> {
    Some(BoolShape::stringify(a == b))
}

pub const OP_NOT_EQUALS: &str = "!=";
fn not_equals(a: &str, b: &str) -> Option<String> {
    Some(BoolShape::stringify(a != b))
}

pub const OP_NOT: &str = "not";
fn not(a: &str) -> Option<String> {
    let a = BoolShape::parse(a)?;
    
    Some(BoolShape::stringify(!a))
}

pub const OP_DEFINED: &str = "defined";
fn defined(a: &str) -> Option<String> {
    Some(BoolShape::stringify(a != ""))
}