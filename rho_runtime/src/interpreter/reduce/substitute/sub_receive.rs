use super::*;

impl Substitutable for Receive {

    fn substitute(&mut self, context : &InterpreterContext, depth : i32, env : &Env) -> Result<(), ExecutionError> {

        self.substitute_no_sort(context, depth, env)?;
        self.sort();
        Ok(())
    }


    fn substitute_no_sort(&mut self, context : &InterpreterContext, depth : i32, env : &Env) -> Result<(), ExecutionError> {


        // for {
        //     bindsSub <- term.binds.toVector.traverse {
        //                  case ReceiveBind(patterns, chan, rem, freeCount) =>
        //                    for {
        //                      subChannel <- substitutePar[M].substituteNoSort(chan)
        //                      subPatterns <- patterns.toVector.traverse(
        //                                      pattern =>
        //                                        substitutePar[M]
        //                                          .substituteNoSort(pattern)(depth + 1, env)
        //                                    )
        //                    } yield ReceiveBind(subPatterns, subChannel, rem, freeCount)
        //                }
        //     bodySub <- substitutePar[M].substituteNoSort(term.body)(
        //                 depth,
        //                 env.shift(term.bindCount)
        //               )
        //     rec = Receive(
        //       binds = bindsSub,
        //       body = bodySub,
        //       persistent = term.persistent,
        //       peek = term.peek,
        //       bindCount = term.bindCount,
        //       locallyFree = term.locallyFree.until(env.shift),
        //       connectiveUsed = term.connectiveUsed
        //     )
        //   } yield rec

        unimplemented!("Receive::substitute_no_sort")

        //Ok(self)
    }
    
}
