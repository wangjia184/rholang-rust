

use super::*;




impl<S> Evaluator<S, ()> for Match 
    where S : Storage + std::marker::Send + std::marker::Sync + 'static
{


    fn evaluate(&mut self, context : &Arc<InterpreterContext<S>>, env : &Env) -> Result<(), ExecutionError> {
 

        // charge[M](MATCH_EVAL_COST)

        if let Some(target) = &mut self.target {
            target.evaluate_nested_expressions(context, env)?;

            // substituteAndCharge[Par, M](evaledTarget, 0, env)
            target.substitute(context, 0, env)?;


            // cases match {
            //     case Nil => ().asRight[(Par, Seq[MatchCase])].pure[M]
            //     case singleCase +: caseRem =>
            //       for {
            //         pattern     <- substituteAndCharge[Par, M](singleCase.pattern, 1, env)
            //         matchResult <- spatialMatchResult[M](target, pattern)
            //         res <- matchResult match {
            //                 case None =>
            //                   (target, caseRem).asLeft[Unit].pure[M]
            //                 case Some((freeMap, _)) =>
            //                   eval(singleCase.source)(
            //                     addToEnv(env, freeMap, singleCase.freeCount),
            //                     implicitly
            //                   ).map(_.asRight[(Par, Seq[MatchCase])])
            //               }
            //       } yield res
            //   }

            // prototype is simplified and only supports boolean
            if target.exprs.len() == 1 {
                match target.exprs[0] {
                    Expr { expr_instance: Some(expr::ExprInstance::GBool(target_value)) } => {

                        for case in &mut self.cases {
                            if let Some(pattern) = &mut case.pattern {
                                pattern.substitute(context, 1, env)?;
                                
                                if pattern.exprs.len() == 1 {
                                    match pattern.exprs[0] {
                                        Expr {
                                            expr_instance: Some(expr::ExprInstance::GBool(pattern_value))
                                        } => {
                                            if target_value == pattern_value {
                                                if let Some(mut source) = case.source.take() {
                                                    source.evaluate(context, env)?;
                                                }
                                                return Ok(())
                                            }
                                        },
                                        _ => {
                                            warn!("Unsupported match");
                                        }
                                    }
                                }
                            }
                        }

                    },
                    _ => {
                        warn!("Unsupported match");
                    }
                }
                
            }
            


        } else {
            return Err((ExecutionErrorKind::InvalidMatch, "`Match::target` is missing").into());
        }

        


        Ok(())
    }
}

