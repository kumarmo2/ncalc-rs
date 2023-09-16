#![allow(unused_variables, unreachable_code)]
use ncalc_rs::evaluator;
use ncalc_rs::object::Context;
use std::ffi::{c_char, CStr};
use std::ptr;

#[repr(C)]
pub struct CResult {
    int_result: i64,
    float_result: f64,
    bool_result: bool,
    string_result: *const u8,
    error: *const u8,
}

/* TODO:
* 1. Need to expose a method for freeing the created CResult.
* */

pub extern "C" fn evaluate(formula: *const c_char) -> CResult {
    let formula: &CStr = unsafe { CStr::from_ptr(formula as *const i8) };
    let formula_str = match formula.to_str() {
        Ok(formula) => formula,
        Err(e) => {
            return CResult {
                error: b"Not a valid utf8 string\0".as_ptr(),
                success: ptr::null(),
            }
        }
    };
    // evaluator::e
    evaluator::eval_input(formula_str, Context::default());

    todo!()
}
