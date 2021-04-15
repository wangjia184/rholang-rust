use super::*;



#[async_trait]
impl AsyncEvaluator for Par {

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
    async fn evaluate(&mut self, context : &Arc<InterpreterContext>, env : &Env) -> Result<(), ExecutionError> {
 
        context.may_raise_aborted_error()?;

        // val terms: Seq[GeneratedMessage] = Seq(
        //     par.sends,
        //     par.receives,
        //     par.news,
        //     par.matches,
        //     par.bundles,
        //     par.exprs.filter { expr =>
        //       expr.exprInstance match {
        //         case _: EVarBody    => true
        //         case _: EMethodBody => true
        //         case _              => false
        //       }
        //     }
        //   ).filter(_.nonEmpty).flatten
      
        //   def split(id: Int): Blake2b512Random =
        //     if (terms.size == 1) rand
        //     else if (terms.size > 256) rand.splitShort(id.toShort)
        //     else rand.splitByte(id.toByte)
      
        //   // Term split size is limited to half of Int16 because other half is for injecting
        //   // things to system deploys through NormalizerEnv
        //   val termSplitLimit = Short.MaxValue
        //   if (terms.size > termSplitLimit)
        //     ReduceError(
        //       s"The number of terms in the Par is ${terms.size}, which exceeds the limit of ${termSplitLimit}."
        //     ).raiseError[M, Unit]
        //   else {
      
        //     // Collect errors from all parallel execution paths (pars)
        //     parTraverseSafe(terms.zipWithIndex.toVector) {
        //       case (term, index) =>
        //         eval(term)(env, split(index))
        //     }
        //   }

        let mut handles = Vec::new();

        while let Some(s) = self.sends.pop() {
            handles.push(context.spawn_evaluation(s, &env));
        }
        while let Some(r) = self.receives.pop() {
            handles.push(context.spawn_evaluation(r, &env));
        }

        for handle in handles {
            let result = handle.await;
            if result.is_err() {
                break; // we dont need log the error since it is already handled in spawn_evaluation
            }
        }

        Ok(())
    }
}




