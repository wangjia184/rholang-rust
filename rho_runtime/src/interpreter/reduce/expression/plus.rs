use super::*;

impl DebruijnInterpreter {

    pub fn evaluate_plus(&self, p1 : Par, p2 : Par) -> Result<Expr, ExecutionError> {
        let v1 = self.evaluate_single_expression(p1)?;
        let v2 = self.evaluate_single_expression(p2)?;
        match (v1.expr_instance, v2.expr_instance) {
            (Some(ExprInstance::GInt(left)), Some(ExprInstance::GInt(right))) => {
                // charge[M](SUM_COST)
                let (sum, overflowed) = left.overflowing_add(right);
                if !overflowed {
                    Ok(Expr {
                        expr_instance : Some(ExprInstance::GInt(sum))
                    })
                } else {
                    Err(self.add_error(ExecutionErrorKind::ArithmeticOverflow, "Arithmetic overflow"))
                }
                
            },
            // case (lhs: ESetBody, rhs) =>
            //     for {
            //     _         <- charge[M](OP_CALL_COST)
            //     resultPar <- add(lhs, List[Par](rhs))
            //     resultExp <- evalSingleExpr(resultPar)
            //     } yield resultExp
            // case (_: GInt, other) =>
            //     OperatorExpectedError("+", "Int", other.typ).raiseError[M, Expr]
            // case (other, _) => OperatorNotDefined("+", other.typ).raiseError[M, Expr]
            _ => {
                unimplemented!("match (v1.expr_instance, v2.expr_instance)")
            }
        }
    }

}