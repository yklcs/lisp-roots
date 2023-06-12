use lisp_roots::{env::Env, read::read};

fn main() {
    let src = "
        ((lambda (f) (f '(b c))) (lambda (x) (cons 'a x)))
    ";

    let xs = read(src.to_string()).unwrap();
    let env = Env::new_global();
    for x in xs {
        println!("{}", x.eval(&env).unwrap());
    }
}
