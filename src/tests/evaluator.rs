use crate::{
    evaluator,
    object::{Context, Object},
};

#[test]
fn basic_eval_tests() {
    let tests = [("!true", Object::Bool(false))].into_iter();
    for (test, expected) in tests {
        evaluator::eval_input(test.to_string(), Context::default()).unwrap();
    }
}
