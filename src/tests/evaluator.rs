use std::collections::HashMap;

use crate::{
    evaluator,
    object::{Context, Object},
};

#[test]
fn basic_eval_tests() {
    let tests = [("!true", Object::Bool(false))].into_iter();
    for (test, expected) in tests {
        assert_eq!(
            expected,
            evaluator::eval_input(test.to_string(), Context::default()).unwrap()
        );
    }
}

#[test]
fn evaluate_ident_for_expr() {
    let test = "[x] + 5";
    let mut map = HashMap::new();
    map.insert("x".to_string(), Object::Int(10));
    let context = Context::from_map(map);
    assert_eq!(
        Object::Double(15.0),
        evaluator::eval_input(test.to_string(), context).unwrap()
    );

    let test = "[x] - 5";
    let mut map = HashMap::new();
    map.insert("x".to_string(), Object::Int(10));
    let context = Context::from_map(map);
    assert_eq!(
        Object::Double(5.0),
        evaluator::eval_input(test.to_string(), context).unwrap()
    );

    let test = "[x] * 5";
    let mut map = HashMap::new();
    map.insert("x".to_string(), Object::Int(10));
    let context = Context::from_map(map);
    assert_eq!(
        Object::Double(50.0),
        evaluator::eval_input(test.to_string(), context).unwrap()
    );
    let test = "[x] / [y]";
    let mut map = HashMap::new();
    map.insert("x".to_string(), Object::Int(10));
    map.insert("y".to_string(), Object::Int(2));
    let context = Context::from_map(map);
    assert_eq!(
        Object::Double(5.0),
        evaluator::eval_input(test.to_string(), context).unwrap()
    );
    let test = "[x] < [y]";
    let mut map = HashMap::new();
    map.insert("x".to_string(), Object::Int(10));
    map.insert("y".to_string(), Object::Int(2));
    let context = Context::from_map(map);
    assert_eq!(
        Object::Bool(false),
        evaluator::eval_input(test.to_string(), context).unwrap()
    );

    let test = "[x] <= [y]";
    let mut map = HashMap::new();
    map.insert("x".to_string(), Object::Int(10));
    map.insert("y".to_string(), Object::Int(10));
    let context = Context::from_map(map);
    assert_eq!(
        Object::Bool(true),
        evaluator::eval_input(test.to_string(), context).unwrap()
    );

    let test = "[x] <= [y]";
    let mut map = HashMap::new();
    map.insert("x".to_string(), Object::Int(11));
    map.insert("y".to_string(), Object::Int(10));
    let context = Context::from_map(map);
    assert_eq!(
        Object::Bool(false),
        evaluator::eval_input(test.to_string(), context).unwrap()
    );

    let test = "[x] > [y]";
    let mut map = HashMap::new();
    map.insert("x".to_string(), Object::Int(11));
    map.insert("y".to_string(), Object::Int(10));
    let context = Context::from_map(map);
    assert_eq!(
        Object::Bool(true),
        evaluator::eval_input(test.to_string(), context).unwrap()
    );

    let test = "[x] >= [y]";
    let mut map = HashMap::new();
    map.insert("x".to_string(), Object::Int(11));
    map.insert("y".to_string(), Object::Int(10));
    let context = Context::from_map(map);
    assert_eq!(
        Object::Bool(true),
        evaluator::eval_input(test.to_string(), context).unwrap()
    );

    let test = "[x] >= [y]";
    let mut map = HashMap::new();
    map.insert("x".to_string(), Object::Int(9));
    map.insert("y".to_string(), Object::Int(10));
    let context = Context::from_map(map);
    assert_eq!(
        Object::Bool(false),
        evaluator::eval_input(test.to_string(), context).unwrap()
    );

    let test = "[x] == [y]";
    let mut map = HashMap::new();
    map.insert("x".to_string(), Object::Bool(true));
    map.insert("y".to_string(), Object::Bool(false));
    let context = Context::from_map(map);
    assert_eq!(
        Object::Bool(false),
        evaluator::eval_input(test.to_string(), context).unwrap()
    );
    let test = "[x] == [y]";
    let mut map = HashMap::new();
    map.insert("x".to_string(), Object::Bool(false));
    map.insert("y".to_string(), Object::Bool(false));
    let context = Context::from_map(map);
    assert_eq!(
        Object::Bool(true),
        evaluator::eval_input(test.to_string(), context).unwrap()
    );
    let test = "[x] || [y]";
    let mut map = HashMap::new();
    map.insert("x".to_string(), Object::Bool(false));
    map.insert("y".to_string(), Object::Bool(false));
    let context = Context::from_map(map);
    assert_eq!(
        Object::Bool(false),
        evaluator::eval_input(test.to_string(), context).unwrap()
    );
    let test = "[x] or [y]";
    let mut map = HashMap::new();
    map.insert("x".to_string(), Object::Bool(false));
    map.insert("y".to_string(), Object::Bool(false));
    let context = Context::from_map(map);
    assert_eq!(
        Object::Bool(false),
        evaluator::eval_input(test.to_string(), context).unwrap()
    );
}

#[test]
fn evaluate_ident_for_bool() {
    // TODO: do some test for negative cases.
    let test = "![x]";
    let mut map = HashMap::new();
    map.insert("x".to_string(), Object::Bool(true));
    let context = Context::from_map(map);
    assert_eq!(
        Object::Bool(false),
        evaluator::eval_input(test.to_string(), context).unwrap()
    );
}
