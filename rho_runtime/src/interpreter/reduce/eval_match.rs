

use super::*;



#[async_trait]
impl<S> AsyncEvaluator<S, ()> for Match 
    where S : Storage + std::marker::Send + std::marker::Sync 
{


    async fn evaluate(&mut self, context : &Arc<InterpreterContext<S>>, env : &Env) -> Result<(), ExecutionError> {
 

        // charge[M](MATCH_EVAL_COST)

        if let Some(target) = &mut self.target {
            target.evaluate_nested_expressions(context, env).await?;

            // substituteAndCharge[Par, M](evaledTarget, 0, env)
            target.substitute(context, 0, env)?;

            for case in &mut self.cases {
                if let Some(pattern) = &mut case.pattern {
                    pattern.substitute(context, 1, env)?;
                }
            }


        } else {
            return Err((ExecutionErrorKind::InvalidMatch, "`Match::target` is missing").into());
        }

        


        Ok(())
    }
}

