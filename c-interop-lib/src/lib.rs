#![allow(unused_variables, unreachable_code)]
use ncalc_rs::evaluator;
use ncalc_rs::object::{Context, Object};
use std::ffi::{c_char, CStr};
use std::ptr;

#[repr(C)]
pub struct CResult {
    int_result: *const i64,
    float_result: *const f64,
    bool_result: *const bool,
    string_result: *const u8,
    error: *const u8,
}

impl From<Object> for CResult {
    fn from(value: Object) -> Self {
        match value {
            Object::Int(int) => CResult {
                int_result: &int as *const i64,
                float_result: ptr::null(),
                bool_result: ptr::null(),
                string_result: ptr::null(),
                error: ptr::null(),
            },
            Object::Double(double) => CResult {
                int_result: ptr::null(),
                float_result: &double as *const f64,
                bool_result: ptr::null(),
                string_result: ptr::null(),
                error: ptr::null(),
            },
            Object::Bool(bool) => CResult {
                int_result: ptr::null(),
                float_result: ptr::null(),
                bool_result: &bool as *const bool,
                string_result: ptr::null(),
                error: ptr::null(),
            },
            Object::Str(string) => CResult {
                int_result: ptr::null(),
                float_result: ptr::null(),
                bool_result: ptr::null(),
                string_result: string.as_ptr(),
                error: ptr::null(),
            },
        }
    }
}

/* TODO:
* 1. Need to expose a method for freeing the created CResult.
* */

#[no_mangle]
pub extern "C" fn evaluate(formula: *const c_char) -> CResult {
    let formula: &CStr = unsafe { CStr::from_ptr(formula as *const i8) };
    let formula_str = match formula.to_str() {
        Ok(formula) => formula,
        Err(e) => {
            return CResult {
                error: b"Not a valid utf8 string\0".as_ptr(),
                int_result: ptr::null(),
                float_result: ptr::null(),
                bool_result: ptr::null(),
                string_result: ptr::null(),
            }
        }
    };
    let object = match evaluator::eval_input(formula_str, Context::default()) {
        Ok(object) => object,
        Err(e) => {
            return CResult {
                error: b"error while evaluating the formula\0".as_ptr(),
                int_result: ptr::null(),
                float_result: ptr::null(),
                bool_result: ptr::null(),
                string_result: ptr::null(),
            }
        }
    };
    CResult::from(object)
}
