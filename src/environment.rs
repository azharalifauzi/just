use std::{collections::HashMap, fmt};

use crate::ast::Statement;

#[derive(Debug, Clone)]

pub struct FunctionExpression {
    pub parameters: Vec<String>,
    pub body: Vec<Statement>,
}

impl FunctionExpression {
    pub fn new(parameters: Vec<String>, body: Vec<Statement>) -> Self {
        Self { parameters, body }
    }
}

#[derive(Debug, Clone)]

pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
    Function(Box<FunctionExpression>),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "\"{}\"", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Null => write!(f, "null"),
            Value::Function(_) => write!(f, "[Function]"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Environment {
    values: HashMap<String, Value>,
    pub parent: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            parent: None,
        }
    }

    pub fn with_parent(parent: Environment) -> Self {
        Self {
            values: HashMap::new(),
            parent: Some(Box::new(parent)),
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &String) -> Option<Value> {
        if let Some(v) = self.values.get(name) {
            return Some(v.clone());
        }

        if let Some(parent) = &self.parent {
            return parent.get(name);
        }

        None
    }

    pub fn assign(&mut self, name: String, value: Value) -> Result<(), String> {
        if self.values.contains_key(&name) {
            self.values.insert(name, value);
            return Ok(());
        }

        if let Some(parent) = &mut self.parent {
            return parent.assign(name, value);
        }

        Err(format!(""))
    }
}
