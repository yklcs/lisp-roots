use lisp_roots::read_eval;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn lisp_root_read_eval(src: &str) -> String {
    read_eval(src.to_string())
}
