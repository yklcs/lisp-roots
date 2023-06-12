use std::{fmt, slice};

use crate::expr::Expr;

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct List(Vec<Expr>);

// pub trait List {
//     fn push(&mut self, x: Expr);
//     fn new() -> Self;
// }

impl List {
    pub fn push(&mut self, x: Expr) {
        self.0.push(x);
    }

    pub fn car(&self) -> &Expr {
        &self.0[0]
    }

    pub fn cadr(&self) -> &Expr {
        &self.0[1]
    }

    pub fn cdr(&self) -> Self {
        Self(self.0[1..].to_vec())
    }

    pub fn new() -> Self {
        List(Vec::new())
    }

    pub fn iter(&self) -> slice::Iter<Expr> {
        self.0.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl FromIterator<Expr> for List {
    fn from_iter<T: IntoIterator<Item = Expr>>(iter: T) -> Self {
        List(iter.into_iter().collect())
    }
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if !self.is_empty() && self.car() == &Expr::from("quote") {
            write!(f, "'{}", self.cadr())
        } else {
            let mut ss = "(".to_string();
            ss.extend(self.iter().map(|l| l.to_string() + " "));
            if !self.is_empty() {
                ss.pop();
            }
            ss += ")";
            write!(f, "{}", ss)
        }
    }
}
