use crate::{
    expr::{
        func::{Func, Primitive, PrimitiveFn},
        Expr,
    },
    primitives,
};

type Scope = Vec<(Expr, Expr)>;

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct Env {
    scopes: Vec<Scope>,
}

fn new_primitive_scope_pair(name: &str, func: PrimitiveFn) -> (Expr, Expr) {
    (
        Expr::from(name),
        Expr::Func(Func::Primitive(Box::new(Primitive {
            expr: Expr::from(name),
            f: func,
        }))),
    )
}

impl Env {
    pub fn new() -> Self {
        Env { scopes: Vec::new() }
    }

    pub fn new_global() -> Self {
        Env {
            scopes: vec![vec![
                new_primitive_scope_pair("car", primitives::car),
                new_primitive_scope_pair("cdr", primitives::cdr),
                new_primitive_scope_pair("atom", primitives::atom),
                new_primitive_scope_pair("eq", primitives::eq),
                new_primitive_scope_pair("cons", primitives::cons),
            ]],
        }
    }

    pub fn get(&self, k: &Expr) -> Option<&Expr> {
        for env in self.scopes.iter().rev() {
            for (key, val) in env {
                if key == k {
                    return Some(val);
                }
            }
        }

        None
    }

    pub fn insert(&mut self, k: &Expr, v: &Expr) {
        self.scopes.push(vec![(k.clone(), v.clone())]);
    }

    pub fn extend(&mut self, scope: Scope) {
        self.scopes.push(scope);
    }
}

impl From<Vec<Scope>> for Env {
    fn from(value: Vec<Scope>) -> Self {
        Env { scopes: value }
    }
}
