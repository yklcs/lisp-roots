use crate::{error::Error, expr::Expr};

pub fn read(src: String) -> Result<Vec<Expr>, Error> {
    let tokens = tokenize(src);
    parse(&tokens)
}

fn tokenize(src: String) -> Vec<String> {
    src.replace("(", " ( ")
        .replace(")", " ) ")
        .replace("'", "' ")
        .split_whitespace()
        .map(|t| t.to_string())
        .collect()
}

fn parse(tokens: &Vec<String>) -> Result<Vec<Expr>, Error> {
    let mut exprs = Vec::new();
    let (mut expr, mut rest) = parse_partial(&Expr::new_nil(), tokens)?;
    exprs.push(expr);

    while !rest.is_empty() {
        (expr, rest) = parse_partial(&Expr::new_nil(), &rest)?;
        exprs.push(expr);
    }
    Ok(exprs)
}

fn parse_partial(expr: &Expr, tokens: &Vec<String>) -> Result<(Expr, Vec<String>), Error> {
    let (car, cdr_arr) = tokens
        .split_first()
        .ok_or(Error::ReadError("no tokens to parse".to_string()))?;
    let mut cdr = cdr_arr.to_vec();

    let mut list = if let Expr::List(ls) = expr.clone() {
        ls
    } else {
        panic!("non list encountered while parsing")
    };

    match car.as_str() {
        "'" => {
            list.push(Expr::from("quote"));
            let (parsed, remaining) = parse_partial(expr, &cdr)?;
            list.push(parsed);
            Ok((Expr::List(list), remaining))
        }
        "(" => {
            if cdr.is_empty() {
                return Err(Error::ReadError("unmatched parens".to_string()));
            }
            while cdr[0] != ")" {
                let l: Expr;
                (l, cdr) = parse_partial(expr, &cdr)?;
                list.push(l);
            }
            let rest = cdr[1..].to_vec();
            Ok((Expr::List(list), rest))
        }
        ")" => Err(Error::ReadError("unexpected )".to_string())),
        _ => Ok((Expr::from(car.as_str()), cdr.to_vec())),
    }
}
