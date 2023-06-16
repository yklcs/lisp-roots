use crate::{
    env::Env,
    error::Error,
    expr::{
        func::{Func, Lambda},
        Expr,
    },
    list::List,
};

fn check_args_count(expected: usize, got: usize) -> Result<(), Error> {
    if expected != got {
        Err(Error::EvalError(format!(
            "expected {} argument(s), got {}",
            expected, got
        )))
    } else {
        Ok(())
    }
}

/* Special forms */

pub(crate) fn quote(cdr: List, _env: &mut Env) -> Result<Expr, Error> {
    check_args_count(1, cdr.len())?;

    Ok(cdr.car().clone())
}

fn cond_branch(cdr: List, env: &mut Env) -> Result<Option<Expr>, Error> {
    check_args_count(2, cdr.len())?;

    let condition = cdr.car().clone().eval(env)?;
    if condition == Expr::from("t") {
        let result = cdr.cdr().car().clone().eval(env)?;
        Ok(Some(result))
    } else {
        Ok(None)
    }
}

pub(crate) fn cond(cdr: List, env: &mut Env) -> Result<Expr, Error> {
    for (i, branch) in cdr.iter().enumerate() {
        match branch {
            Expr::List(ls) => {
                let result = cond_branch(ls.clone(), env)?;
                match result {
                    Some(r) => return Ok(r),
                    None => continue,
                }
            }
            _ => {
                return Err(Error::EvalError(format!(
                    "expected list in argument {}, got atom",
                    i
                )))
            }
        }
    }

    Err(Error::EvalError("cond does not match".to_string()))
}

pub(crate) fn lambda(cdr: List, env: &Env) -> Result<Expr, Error> {
    check_args_count(2, cdr.len())?;

    let params = cdr.car();
    let rest = cdr.cdr();
    let body = rest.car();

    Ok(Expr::Func(Func::Lambda(Box::new(Lambda {
        body: body.clone(),
        params: params.clone(),
        env: env.clone(),
    }))))
}

pub(crate) fn defun(cdr: List, env: &mut Env) -> Result<Expr, Error> {
    check_args_count(3, cdr.len())?;

    let id = cdr.car().clone();
    let params = cdr.cdr().car().clone();
    let body = cdr.cdr().cdr().car().clone();

    let mut f = Expr::Func(Func::Lambda(Box::new(Lambda {
        body,
        params,
        env: env.clone(),
    })));

    env.insert(&id, &f);
    if let Expr::Func(Func::Lambda(ref mut l)) = f {
        l.env = env.clone();
    }

    Ok(Expr::new_nil())
}

/* Functions */

pub(crate) fn car(cdr: List, _env: &Env) -> Result<Expr, Error> {
    check_args_count(1, cdr.len())?;

    if let Expr::List(ls) = cdr.car() {
        Ok(ls.car().clone())
    } else {
        Err(Error::EvalError("passed non list to car".to_string()))
    }
}

pub(crate) fn cdr(cdr: List, _env: &Env) -> Result<Expr, Error> {
    check_args_count(1, cdr.len())?;

    if let Expr::List(ls) = cdr.car() {
        Ok(Expr::List(ls.cdr()))
    } else {
        Err(Error::EvalError("passed non list to cdr".to_string()))
    }
}

pub(crate) fn atom(cdr: List, _env: &Env) -> Result<Expr, Error> {
    check_args_count(1, cdr.len())?;

    match cdr.car().clone() {
        Expr::Atom(_) => Ok(Expr::from("t")),
        Expr::List(ls) => {
            if ls.is_empty() {
                Ok(Expr::from("t"))
            } else {
                Ok(Expr::new_nil())
            }
        }
        Expr::Func(_) => Ok(Expr::from("t")),
    }
}

pub(crate) fn eq(cdr: List, _env: &Env) -> Result<Expr, Error> {
    check_args_count(2, cdr.len())?;

    if cdr.car() == cdr.cdr().car() {
        Ok(Expr::from("t"))
    } else {
        Ok(Expr::new_nil())
    }
}

pub(crate) fn cons(cdr: List, _env: &Env) -> Result<Expr, Error> {
    check_args_count(2, cdr.len())?;

    let car = cdr.car();
    let rest = cdr.cdr();
    let cdr = rest.car();

    match cdr {
        Expr::List(ls) => {
            let mut new_ls = List::new();
            new_ls.push(car.clone());

            for x in ls.iter() {
                new_ls.push(x.clone())
            }

            Ok(Expr::List(new_ls))
        }
        _ => Err(Error::EvalError("passed non list to cons cdr".to_string())),
    }
}
