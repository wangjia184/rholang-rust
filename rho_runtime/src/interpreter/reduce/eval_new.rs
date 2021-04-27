

use model::g_unforgeable::UnfInstance;


use super::*;




impl<S : Storage + std::marker::Send + std::marker::Sync + 'static> Evaluator<S> for New {

   /** Algorithm as follows:
    *
    * 1. Fully evaluate the channel in given environment.
    * 2. Substitute any variable references in the channel so that it can be
    *    correctly used as a key in the tuple space.
    * 3. Evaluate any top level expressions in the data being sent.
    * 4. Call produce
    * 5. If produce returned a continuation, evaluate it.
    *
    * @param send An output process
    * @param env An execution context
    *
    */
    fn evaluate(&mut self, context : &Arc<InterpreterContext<S>>, env : &Env) -> Result<(), ExecutionError> {
 
        context.may_raise_aborted_error()?;

        // charge[M](newBindingsCost(neu.bindCount))

        let mut new_env = env.clone();

        let count = self.bind_count as usize - self.uri.len();

        for _ in 0..count {
            let name_par = {
                let mut par = Par::default();
                par.unforgeables.push(
                    {
                        GUnforgeable {
                            unf_instance : Some(UnfInstance::GPrivateBody(
                                GPrivate {
                                    id : HashRand::next(),
                                }
                            ))
                        }
                    }
                );
                par
            };
            new_env.put(name_par);
        }

        for urn in &self.uri {
            let option = match context.urn_map.read()  {
                Ok(urn_map) => {
                    urn_map.get(urn).map(|x| x.clone())
                },
                Err(e) => {
                    return Err((ExecutionErrorKind::SystemFailure, format!("Unable to read `urn_map` : {} - {:?}", &e, e)).into());
                }
            };

            match option {
                None => {
                    todo!("If `urn` can't be found in `urnMap`, it must be referencing an injection");

                    // neu.injections
                    //     .get(urn)
                    //     .map {
                    //     case RhoType.Unforgeable(GUnforgeable(unfInstance)) if unfInstance.isDefined =>
                    //         newEnv.put(unfInstance).asRight[InterpreterError]

                    //     case RhoType.Expression(Expr(exprInstance)) if exprInstance.isDefined =>
                    //         newEnv.put(exprInstance).asRight[InterpreterError]

                    //     case _ =>
                    //         BugFoundError("Invalid injection.").asLeft[Env[Par]]
                    //     }
                    //     .getOrElse(normalizerBugFound(urn).asLeft[Env[Par]])
                },
                Some(p) => {
                    new_env.put(p);
                }
            };
        }// for urn in &self.uri

        if let Some(mut par) = self.p.take() {
            par.evaluate(context, &new_env)
        } 
        else {
            // no body for this new
            Ok(())
        }
    }
}

