use std::fmt;

use crate::expr::Expr;

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct Cons {
    value: Expr,
    next: Option<Box<Cons>>,
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct List {
    head: Option<Box<Cons>>,
    len: usize,
}

impl List {
    pub fn push(&mut self, x: Expr) {
        let new = Box::new(Cons {
            value: x,
            next: None,
        });

        let mut cur = self.head.as_mut();
        while let Some(node) = cur {
            if node.next.is_none() {
                node.next = Some(new);
                self.len += 1;
                return;
            }
            cur = node.next.as_mut();
        }

        self.head = Some(new);
        self.len = 1;
    }

    pub fn car(&self) -> &Expr {
        if let Some(car) = &self.head {
            &car.value
        } else {
            panic!("car on empty list")
        }
    }

    pub fn cdr(&self) -> Self {
        if let Some(car) = &self.head {
            List {
                head: car.next.clone(),
                len: self.len - 1,
            }
        } else {
            panic!("cdr on empty list")
        }
    }

    pub fn new() -> Self {
        List { head: None, len: 0 }
    }

    pub fn iter(&self) -> ConsIter {
        ConsIter(self.head.as_deref())
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

pub struct ConsIter<'a>(Option<&'a Cons>);

impl<'a> Iterator for ConsIter<'a> {
    type Item = &'a Expr;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.map(|cons| {
            self.0 = cons.next.as_deref();
            &cons.value
        })
    }
}

impl FromIterator<Expr> for List {
    fn from_iter<T: IntoIterator<Item = Expr>>(iter: T) -> Self {
        let mut ls = List::new();
        for x in iter.into_iter() {
            ls.push(x);
        }
        ls
    }
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if !self.is_empty() && self.car() == &Expr::from("quote") {
            write!(f, "'{}", self.cdr().car())
        } else {
            let mut ss = "(".to_string();
            ss.extend(self.iter().map(|l| l.to_string() + " "));
            if !self.is_empty() {
                ss.pop();
            }
            ss += ")";
            write!(f, "{}", ss)
        }
    }
}
