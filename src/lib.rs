pub mod env;
mod error;
pub mod eval;
pub mod expr;
mod list;
mod primitives;
pub mod read;

pub fn read_eval(src: String) -> String {
    let xs = read::read(src).unwrap();
    let mut env = env::Env::new_global();

    let mut output = String::new();

    for x in xs {
        let result = x.eval(&mut env).unwrap();
        if result != expr::Expr::new_nil() {
            output += &format!("{}\n", result);
        }
    }

    output
}
