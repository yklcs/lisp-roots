use std::{fmt::format, primitive};

use crate::{
    env::Env,
    error::Error,
    expr::{func::Callable, Atom, Expr},
    list::List,
    primitives,
};

fn evlis(ls: &List, env: &Env) -> Result<List, Error> {
    let ls_eval = ls
        .iter()
        .map(|x| x.clone().eval(env))
        .collect::<Result<List, Error>>()?;
    Ok(ls_eval)
}

fn eval_special_form(ls: &List, env: &Env) -> Option<Result<Expr, Error>> {
    match ls.car() {
        Expr::Atom(atom) => match atom.0.as_str() {
            "quote" => Some(primitives::quote(ls.cdr(), env)),
            "cond" => Some(primitives::cond(ls.cdr(), env)),
            "lambda" => Some(primitives::lambda(ls.cdr(), env)),
            _ => None,
        },
        _ => None,
    }
}

impl Expr {
    pub fn eval(&self, env: &Env) -> Result<Expr, Error> {
        println!("{}", self);

        let x = match self {
            Expr::Atom(_) => env
                .get(self)
                .ok_or(Error::EvalError(format!("{} does not exist", self)))?
                .clone(),
            Expr::List(ls) => {
                if let Some(result) = eval_special_form(ls, env) {
                    return result;
                } else {
                    Expr::List(evlis(ls, env)?)
                }
            }
            _ => self.clone(),
        };

        let (car, cdr) = match &x {
            Expr::Atom(_) => {
                return Ok(x);
            }
            Expr::List(ls) => {
                if ls.is_empty() {
                    return Ok(x);
                }

                println!("REACH {}", ls);

                // let ls_eval = evlis(ls, env)?;
                let car_eval = ls.car().eval(env)?;
                (car_eval.clone(), ls.cdr())
            }
            _ => return Ok(x),
        };

        if let Expr::Func(f) = car {
            println!("calling {} with {}", f, cdr);
            f.call(cdr, env)
        } else {
            Err(Error::EvalError("not a function".to_string()))
        }

        // Ok(self)
    }
}
