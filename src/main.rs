use lisp_roots::{env::Env, read::read};

fn main() {
    let xs = read("((lambda x x) 'a)".to_string()).unwrap();
    let env = Env::new_global();
    for x in xs {
        println!("{}", x.eval(&env).unwrap());
    }
}
