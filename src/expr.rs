use std::fmt;

use crate::{env::Env, error::Error, list::List};

use self::func::Func;

pub mod func {
    use std::{hash::{Hash, Hasher}, rc::Rc};

    use super::*;

    #[derive(Clone, Hash, PartialEq, Eq, Debug)]
    pub enum Func {
        Primitive(Box<Primitive>),
        Lambda(Box<Lambda>),
    }

    pub trait Callable {
        fn call(&self, args: List, env: &Env) -> Result<Expr, Error>;
    }

    impl Callable for Func {
        fn call(&self, args: List, env: &Env) -> Result<Expr, Error> {
            match self {
                Func::Lambda(f) => f.call(args, env),
                Func::Primitive(f) => f.call(args, env),
            }
        }
    }

    impl fmt::Display for Func {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Func::Lambda(l) => {
                    write!(f, "(lambda {} {})", l.params, l.body)
                }
                Func::Primitive(p) => {
                    write!(f, "{}", p.expr)
                }
            }
        }
    }

    pub type PrimitiveFn = fn(List, &Env) -> Result<Expr, Error>;

    pub struct Primitive {
        pub expr: Expr,
        pub f: PrimitiveFn,
    }

    impl fmt::Debug for Primitive {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.expr)
        }
    }

    impl Clone for Primitive {
        fn clone(&self) -> Self {
            Primitive {
                expr: self.expr.clone(),
                f: self.f,
            }
        }
    }

    impl Hash for Primitive {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.expr.hash(state)
        }
    }

    impl PartialEq for Primitive {
        fn eq(&self, other: &Self) -> bool {
            self.expr == other.expr
        }
    }

    impl Eq for Primitive {}

    impl Callable for Primitive {
        fn call(&self, args: List, env: &Env) -> Result<Expr, Error> {
            (self.f)(args, env)
        }
    }

    #[derive(Clone, Hash, PartialEq, Eq, Debug)]
    pub struct Lambda {
        pub body: Expr,
        pub params: Expr,
        pub env: Rc<Env>,
    }

    impl Callable for Lambda {
        fn call(&self, args: List, env: &Env) -> Result<Expr, Error> {
            let new_scope = match &self.params {
                Expr::Atom(_) => vec![(self.params.clone(), Expr::List(args))],
                Expr::List(ls) => ls
                    .iter()
                    .zip(args.iter())
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect::<Vec<_>>(),
                Expr::Func(_) => return Err(Error::EvalError("encountered func".to_string())),
            };
            let mut new_env = env.clone();
            new_env.extend(new_scope);
            self.body.eval(&mut new_env)
        }
    }
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct Atom(pub(crate) String);

impl From<&str> for Atom {
    fn from(value: &str) -> Self {
        Atom(value.to_string())
    }
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub enum Expr {
    Atom(Atom),
    List(List),
    Func(Func),
}

impl Expr {
    pub fn new_nil() -> Self {
        Expr::List(List::new())
    }
}

impl From<&str> for Expr {
    fn from(value: &str) -> Self {
        Expr::Atom(Atom(value.to_string()))
    }
}

impl From<List> for Expr {
    fn from(value: List) -> Self {
        Expr::List(value)
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Expr::Atom(a) => a.0.clone(),
            Expr::List(ls) => ls.to_string(),
            Expr::Func(f) => f.to_string(),
        };
        write!(f, "{}", s)
    }
}
