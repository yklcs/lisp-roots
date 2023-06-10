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
