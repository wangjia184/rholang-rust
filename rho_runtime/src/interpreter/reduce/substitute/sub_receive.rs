use super::*;

impl<S : Storage + std::marker::Send + std::marker::Sync> Substitutable<S> for Receive {

    fn substitute(&mut self, context : &InterpreterContext<S>, depth : i32, env : &Env) -> Result<(), ExecutionError> {

        self.substitute_no_sort(context, depth, env)?;
        self.sort();
        Ok(())
    }


    fn substitute_no_sort(&mut self, context : &InterpreterContext<S>, depth : i32, env : &Env) -> Result<(), ExecutionError> {

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
        for bind in &mut self.binds {
            if let Some(source) = &mut bind.source {
                source.substitute_no_sort(context, depth, env)?;   
            }

            for pattern in &mut bind.patterns {
                pattern.substitute_no_sort(context, depth + 1, env)?;
            }
        }

        //     bodySub <- substitutePar[M].substituteNoSort(term.body)(
        //                 depth,
        //                 env.shift(term.bindCount)
        //               )
        let sub_env = env.clone_then_shift(self.bind_count as usize);
        if let Some(body) = &mut self.body {
            body.substitute_no_sort(context, depth, &sub_env)?;
        }

        //     locallyFree = term.locallyFree.until(env.shift),
        if let Some(bitset) = &mut self.locally_free {
            bitset.truncate(env.shift);
        }

        Ok(())
    }
    
}
