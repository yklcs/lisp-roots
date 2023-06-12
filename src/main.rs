use lisp_roots::{env::Env, expr::Expr, read::read};

fn main() {
    let src = "
        (defun subst (x y z)
            (cond ((atom z) (cond ((eq z y) x) ('t z)))
                  ('t (cons (subst x y (car z))
                            (subst x y (cdr z))))  
            )
        )
        (subst 'to 'from '(from to x (x from)))
    ";

    let src = "
        (defun null (x) (eq x '()))
        (defun notnot (x)
            (not (not x))
        )
        (defun not (x)
            (cond ((null x) 't) ('t '()))
        )
        (notnot '())
    ";

    let src = "
    (defun caar (lst) (car (car lst)))
    (defun cddr (lst) (cdr (cdr lst)))
    (defun cadr (lst) (car (cdr lst)))
    (defun cdar (lst) (cdr (car lst)))
    (defun cadar (lst) (car (cdr (car lst))))
    (defun caddr (lst) (car (cdr (cdr lst))))
    (defun caddar (lst) (car (cdr (cdr (car lst)))))
    
    (defun null (x)
        (eq x '()))
    
    (defun and (x y)
        (cond (x (cond (y 't) ('t '())))
              ('t '())))
    
    (defun or (x y)
        (cond (x 't) 
              ('t (cond (y 't) ('t '())))))
    
    (defun not (x)
        (cond (x '())
              ('t 't)))
    
    (defun append (x y)
        (cond ((null x) y)
              ('t (cons (car x) (append (cdr x) y)))))
    
    (defun pair (x y)
      (cons x (cons y '())))
    
    (defun zip (x y)
      (cond ((and (null x) (null y)) '())
            ((and (not (atom x)) (not (atom y)))
             (cons (pair (car x) (car y))
                   (zip (cdr x) (cdr y))))))
    
    (defun assoc (x y)
      (cond ((eq (caar y) x) (cadar y))
            ('t (assoc x (cdr y)))))
    
    (defun eval (exp env)
      (cond
        ((atom exp) (assoc exp env))
        ((atom (car exp))
         (cond
           ((eq (car exp) 'quote) (cadr exp))
           ((eq (car exp) 'atom)  (atom (eval (cadr exp) env)))
           ((eq (car exp) 'eq)    (eq   (eval (cadr exp) env)
                                        (eval (caddr exp) env)))
           ((eq (car exp) 'car)   (car  (eval (cadr exp) env)))
           ((eq (car exp) 'cdr)   (cdr  (eval (cadr exp) env)))
           ((eq (car exp) 'cons)  (cons (eval (cadr exp) env)
                                        (eval (caddr exp) env)))
           ((eq (car exp) 'cond)  (evcon (cdr exp) env))
           ('t (eval (cons (assoc (car exp) env)
                            (cdr exp))
                      env))))
        ((eq (caar exp) 'label)
         (eval (cons (caddar exp) (cdr exp))
                (cons (pair (cadar exp) (car exp)) env)))
        ((eq (caar exp) 'lambda)
         (eval (caddar exp)
                (append (zip (cadar exp) (evlis (cdr exp) env))
                         env)))))
    
    (defun evcon (c env)
      (cond ((eval (caar c) env)
             (eval (cadar c) env))
            ('t (evcon (cdr c) env))))
    
    (defun evlis (m env)
      (cond ((null m) '())
            ('t (cons (eval  (car m) env)
                      (evlis (cdr m) env)))))

    (eval
        '((lambda (x) (cons x '())) x)
        '((x 1))
    )
    ";

    let xs = read(src.to_string()).unwrap();
    let mut env = Env::new_global();
    for x in xs {
        let result = x.eval(&mut env).unwrap();
        if result != Expr::new_nil() {
            println!("{}", result);
        }
    }
}
