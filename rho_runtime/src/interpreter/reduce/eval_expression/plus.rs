use super::*;

impl DebruijnInterpreter {

    pub fn evaluate_plus(&self, p1 : Par, p2 : Par) -> Result<Expr, ExecutionError> {
        let v1 = self.evaluate_single_expression(p1)?;
        let v2 = self.evaluate_single_expression(p2)?;
        match (v1.expr_instance, v2.expr_instance) {
            (Some(ExprInstance::GInt(left)), Some(ExprInstance::GInt(right))) => {
                // charge[M](SUM_COST)
                let (sum, _) = left.overflowing_add(right);
                Ok(Expr {
                    expr_instance : Some(ExprInstance::GInt(sum))
                })
            },
            (Some(ExprInstance::GInt(_)), Some(right)) => {
                let msg = format!("Unexpected operand {} for `+` operator", print_type_of(&right));
                Err(ExecutionError::new(ExecutionErrorKind::UnexpectedOperand, &msg))
            },
            (Some(ExprInstance::ESetBody(left)), Some(right)) => {
                // case (lhs: ESetBody, rhs) =>
                //     for {
                //     _         <- charge[M](OP_CALL_COST)
                //     resultPar <- add(lhs, List[Par](rhs))
                //     resultExp <- evalSingleExpr(resultPar)
                //     } yield resultExp
                unimplemented!("(Some(ExprInstance::ESetBody(left)), Some(right))")
            },
            (Some(left), _) => {
                let msg = format!("Undefined operator on {}", print_type_of(&left));
                Err(ExecutionError::new(ExecutionErrorKind::UndefinedOperator, &msg))
            },
            _ => {
                Err(ExecutionError::new(ExecutionErrorKind::InvalidExpression, "Invalid expression for `+` operator"))
            }
        }
    }

}


fn print_type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}