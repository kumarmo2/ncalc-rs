#![allow(unused_variables, unreachable_code)]
use ncalc_rs::evaluator;
use ncalc_rs::object::{Context, Object};
use std::ffi::{c_char, CStr, CString};
use std::ptr;
use std::rc::Rc;

#[repr(C)]
#[derive(Debug)]
pub struct CResult {
    int_result: *const i64,
    float_result: *const f64,
    bool_result: *const u8,
    string_result: *const u8,
    error: *const u8,
}

impl From<Object> for CResult {
    fn from(value: Object) -> Self {
        let result = match value {
            Object::Int(int) => {
                let boxed = Box::new(int);
                CResult {
                    int_result: Box::into_raw(boxed),
                    float_result: ptr::null(),
                    bool_result: ptr::null(),
                    string_result: ptr::null(),
                    error: ptr::null(),
                }
            }
            Object::Double(double) => {
                let boxed = Box::new(double);
                let result = CResult {
                    int_result: ptr::null(),
                    float_result: Box::into_raw(boxed) as *const f64,
                    bool_result: ptr::null(),
                    string_result: ptr::null(),
                    error: ptr::null(),
                };
                result
            }
            Object::Bool(bool) => CResult {
                int_result: ptr::null(),
                float_result: ptr::null(),
                bool_result: match bool {
                    true => &(1 as u8) as *const u8,
                    false => &(0 as u8) as *const u8,
                },
                string_result: ptr::null(),
                error: ptr::null(),
            },
            Object::Str(string) => {
                // TODO: remove unwraps();
                let string: String = Rc::into_inner(string).unwrap();
                let c_string = CString::new(string).unwrap().into_raw() as *const u8;
                // let c_string = CString::new().unwrap().into_raw();
                let result = CResult {
                    int_result: ptr::null(),
                    float_result: ptr::null(),
                    bool_result: ptr::null(),
                    string_result: c_string,
                    error: ptr::null(),
                };
                result
            }
        };
        // println!("result: {:?}", result);
        result
    }
}

/* TODO:
* 1. Need to expose a method for freeing the created CResult.
* */

#[no_mangle]
pub extern "C" fn free_cresult(result: CResult) {
    // TODO: Need to free correctly the string in `CResult`.
    unsafe {
        if !result.error.is_null() {
            let _ = CString::from_raw(result.error as *mut c_char);
        }
        if !result.float_result.is_null() {
            let _ = Box::from_raw(result.float_result as *mut f64);
        }
        if !result.int_result.is_null() {
            let _ = Box::from_raw(result.int_result as *mut i64);
        }
        if !result.string_result.is_null() {
            let _ = CString::from_raw(result.string_result as *mut c_char);
        }
    };
    println!("free called");
}

#[no_mangle]
pub extern "C" fn evaluate(formula: *const c_char) -> CResult {
    if formula.is_null() {
        return CResult {
            error: CString::new("null pointer for formula not allowed")
                .unwrap()
                .into_raw() as *const u8,
            int_result: ptr::null(),
            float_result: ptr::null(),
            bool_result: ptr::null(),
            string_result: ptr::null(),
        };
    }
    let formula: &CStr = unsafe { CStr::from_ptr(formula as *const i8) };
    // ""
    let formula_str = match formula.to_str() {
        Ok(formula) => formula,
        Err(e) => {
            return CResult {
                error: CString::new("Not a valid utf8 string").unwrap().into_raw() as *const u8,
                int_result: ptr::null(),
                float_result: ptr::null(),
                bool_result: ptr::null(),
                string_result: ptr::null(),
            };
        }
    };
    let object = match evaluator::eval_input(formula_str, Context::default()) {
        Ok(object) => object,
        Err(e) => {
            // TODO: instead of returning a hardcoded string, create an appropriate
            // error message.
            return CResult {
                error: CString::new("error while evaluating the formula")
                    .unwrap()
                    .into_raw() as *const u8,
                int_result: ptr::null(),
                float_result: ptr::null(),
                bool_result: ptr::null(),
                string_result: ptr::null(),
            };
        }
    };
    CResult::from(object)
}
