use super::*;

#[async_trait]
impl AsyncExprInstanceEvaluator for EPlus {
    async fn evaluate(&mut self, context : &Arc<InterpreterContext>, env : &Env) -> Result<ExprInstance, ExecutionError> {

        let v1 = match self.p1 {
            Some(ref mut par) => {
                par.evaluate_single_expression(context, env).await?
            },
            None => {
                return Err((ExecutionErrorKind::InvalidExpression, "EPlus::p2 is None").into());
            }
        };

        let v2 = match self.p2 {
            Some(ref mut par) => {
                par.evaluate_single_expression(context, env).await?
            },
            None => {
                return Err((ExecutionErrorKind::InvalidExpression, "EPlus::p2 is None").into());
            }
        };

        match (v1.expr_instance, v2.expr_instance) {
            (Some(ExprInstance::GInt(left)), Some(ExprInstance::GInt(right))) => {
                // charge[M](SUM_COST)
                let (sum, _) = left.overflowing_add(right);
                Ok(ExprInstance::GInt(sum))
            },
            (Some(ExprInstance::GInt(_)), Some(right)) => {
                let msg = format!("Unexpected operand {} for `+` operator", print_type_of(&right));
                Err(ExecutionError::new(ExecutionErrorKind::UnexpectedOperand, &msg))
            },
            (Some(ExprInstance::ESetBody(_left)), Some(_right)) => {
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