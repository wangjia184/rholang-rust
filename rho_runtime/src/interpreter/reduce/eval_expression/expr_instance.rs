use super::*;


#[async_trait]
impl<S : Storage + std::marker::Send + std::marker::Sync> AsyncEvaluator<S> for Expr {
    async fn evaluate(&mut self, context : &Arc<InterpreterContext<S>>, env : &Env) -> Result<(), ExecutionError> {

        fn relop<FB, FL, FS>(left : Expr, right : Expr, relopb : FB, relopi : FL, relops : FS) -> Result<ExprInstance, ExecutionError>
            where FB : FnOnce(bool, bool) -> bool,
                  FL : FnOnce(i64, i64) -> bool,
                  FS : FnOnce(&String, &String) -> bool
        {
            let value = match (&left.expr_instance, &right.expr_instance) {
                (Some(ExprInstance::GBool(l)), Some(ExprInstance::GBool(r))) => {
                    relopb(*l, *r)
                },
                (Some(ExprInstance::GInt(l)), Some(ExprInstance::GInt(r))) => {
                    relopi(*l, *r)
                },
                (Some(ExprInstance::GString(ref l)), Some(ExprInstance::GString(ref r))) => {
                    relops(l, r)
                },
                _ => {
                    return Err((ExecutionErrorKind::IllegalComparison, format!("Unable to compare {:?} and {:?}", &left.expr_instance, &right.expr_instance)).into());
                }
            };
            Ok(ExprInstance::GBool(value))
        }
  
        match &mut self.expr_instance {
            Some(ExprInstance::GBool(_)) => Ok(()),
            Some(ExprInstance::GInt(_)) => Ok(()),
            Some(ExprInstance::GString(_)) => Ok(()),
            Some(ExprInstance::GUri(_)) => Ok(()),
            Some(ExprInstance::GByteArray(_)) => Ok(()),
            Some(ExprInstance::EPlusBody(eplus)) => {
                self.expr_instance = Some(eplus.evaluate(context, env).await?);
                Ok(())
            },

            Some(ExprInstance::EVarBody(evar)) => {
                if let Some(ref mut var) = evar.v {
                    let mut par = var.evaluate(context, env).await?;
                    let expression = par.evaluate_single_expression(context, env).await?;
                    self.expr_instance = expression.expr_instance;
                }
                Ok(())
            },

            Some(ExprInstance::ELtBody(ELt { p1:Some(p1), p2:Some(p2)})) => {
                // charge[M](COMPARISON_COST)
                let left = p1.evaluate_single_expression(context, env).await?;
                let right = p2.evaluate_single_expression(context, env).await?;
                let expression = relop(left, right
                    , |l,r| l < r
                    , |l, r| l < r
                    , |l, r | l < r
                )?;
                self.expr_instance = Some(expression);
                Ok(())
            },

            Some(ExprInstance::ELteBody(ELte { p1:Some(p1), p2:Some(p2)})) => {
                // charge[M](COMPARISON_COST)
                let left = p1.evaluate_single_expression(context, env).await?;
                let right = p2.evaluate_single_expression(context, env).await?;
                let expression = relop(left, right
                    , |l,r| l <= r
                    , |l, r| l <= r
                    , |l, r | l <= r
                )?;
                self.expr_instance = Some(expression);
                Ok(())
            },

            Some(ExprInstance::EGtBody(EGt { p1:Some(p1), p2:Some(p2)})) => {
                // charge[M](COMPARISON_COST)
                let left = p1.evaluate_single_expression(context, env).await?;
                let right = p2.evaluate_single_expression(context, env).await?;
                let expression = relop(left, right
                    , |l,r| l > r
                    , |l, r| l > r
                    , |l, r | l > r
                )?;
                self.expr_instance = Some(expression);
                Ok(())
            },

            Some(ExprInstance::EGteBody(EGte { p1:Some(p1), p2:Some(p2)})) => {
                // charge[M](COMPARISON_COST)
                let left = p1.evaluate_single_expression(context, env).await?;
                let right = p2.evaluate_single_expression(context, env).await?;
                let expression = relop(left, right
                    , |l,r| l >= r
                    , |l, r| l >= r
                    , |l, r | l >= r
                )?;
                self.expr_instance = Some(expression);
                Ok(())
            },

            _ => {
                panic!("Unhandled branch in ExprInstance::evaluate() : {:?}", self.expr_instance)
            }
        }
    }
}


