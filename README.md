# lisp-roots

A Lisp implementation à la [A Micro-Manual for Lisp (John McCarthy)](https://dl.acm.org/doi/10.1145/960118.808386) and [The Roots of Lisp (Paul Graham)](http://www.paulgraham.com/rootsoflisp.html).

Implemented in Rust with a simple tree-walking interpreter.

Metacircular evaluation works!

## Primitives

\* denotes special forms.
- `quote`\*
- `atom`
- `eq`
- `car`
- `cdr`
- `cons`
- `cond`\*
- `lambda`\*
- `defun`\*
