use crate::{
    expression::Expression,
    object::{Context, Object},
};

pub(crate) fn eval(expression: Expression, context: Context) -> Object {
    match &expression {
        Expression::Int(val) => Object::Int(*val),
        Expression::Double(val) => Object::Double(*val),
        Expression::Str(val) => Object::Str(val.clone()),
        Expression::Bool(val) => Object::Bool(*val),
        _ => todo!(),
    }
}
