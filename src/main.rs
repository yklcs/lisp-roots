use std::{env::args, fs};

use lisp_roots::{env::Env, expr::Expr, read::read};
use rustyline::{DefaultEditor, Result};

fn eval_exprs(xs: &[Expr], env: &mut Env) {
    for x in xs {
        match x.eval(env) {
            Ok(ok) => {
                if ok != Expr::new_nil() {
                    println!("{}", ok);
                }
            }
            Err(err) => {
                eprintln!("{}", err);
            }
        }
    }
}

fn repl() -> Result<()> {
    let mut rl = DefaultEditor::new()?;
    let mut env = Env::new_global();

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => match read(line) {
                Ok(ok) => eval_exprs(&ok, &mut env),
                Err(err) => eprintln!("{}", err),
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() == 1 {
        let _ = repl();
    } else {
        let src = fs::read_to_string(args[1].clone()).unwrap();
        let xs = read(src).unwrap();
        let mut env = Env::new_global();
        for x in xs {
            let result = x.eval(&mut env).unwrap();
            if result != Expr::new_nil() {
                println!("{}", result);
            }
        }
    }
}
