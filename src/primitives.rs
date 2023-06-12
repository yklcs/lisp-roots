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

pub(crate) fn quote(cdr: List, env: &Env) -> Result<Expr, Error> {
    if let Err(e) = check_args_count(1, cdr.len()) {
        return Err(e);
    }

    Ok(cdr.car().clone())
}

fn cond_branch(cdr: List, env: &Env) -> Result<Option<Expr>, Error> {
    if let Err(e) = check_args_count(2, cdr.len()) {
        return Err(e);
    }

    let condition = cdr.car().clone().eval(env)?;
    if condition == Expr::from("t") {
        let result = cdr.cadr().clone().eval(env)?;
        Ok(Some(result))
    } else {
        Ok(None)
    }
}

pub(crate) fn cond(cdr: List, env: &Env) -> Result<Expr, Error> {
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

    return Err(Error::EvalError("cond does not match".to_string()));
}

pub(crate) fn lambda(cdr: List, env: &Env) -> Result<Expr, Error> {
    if let Err(e) = check_args_count(2, cdr.len()) {
        return Err(e);
    }

    let params = cdr.car();
    let body = cdr.cadr();
    Ok(Expr::Func(Func::Lambda(Box::new(Lambda {
        body: body.clone(),
        params: params.clone(),
        env: env.clone(),
    }))))
}

/* Functions */

pub(crate) fn car(cdr: List, _env: &Env) -> Result<Expr, Error> {
    if let Err(e) = check_args_count(1, cdr.len()) {
        return Err(e);
    }

    if let Expr::List(ls) = cdr.car() {
        Ok(ls.car().clone())
    } else {
        Err(Error::EvalError("passed non list to car".to_string()))
    }
}

pub(crate) fn cdr(cdr: List, _env: &Env) -> Result<Expr, Error> {
    if let Err(e) = check_args_count(1, cdr.len()) {
        return Err(e);
    }

    if let Expr::List(ls) = cdr.car() {
        Ok(Expr::List(ls.cdr()))
    } else {
        Err(Error::EvalError("passed non list to cdr".to_string()))
    }
}

pub(crate) fn atom(cdr: List, _env: &Env) -> Result<Expr, Error> {
    if let Err(e) = check_args_count(1, cdr.len()) {
        return Err(e);
    }

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
    if let Err(e) = check_args_count(2, cdr.len()) {
        return Err(e);
    }

    if cdr.car() == cdr.cadr() {
        Ok(Expr::from("t"))
    } else {
        Ok(Expr::new_nil())
    }
}

pub(crate) fn cons(cdr: List, _env: &Env) -> Result<Expr, Error> {
    if let Err(e) = check_args_count(2, cdr.len()) {
        return Err(e);
    }

    let car = cdr.car();
    let cdr = cdr.cadr();

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
