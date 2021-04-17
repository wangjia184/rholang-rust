use super::*;

impl<S : Storage + std::marker::Send + std::marker::Sync> Substitutable<S> for Send {

    fn substitute(&mut self, context : &InterpreterContext<S>, depth : i32, env : &Env) -> Result<(), ExecutionError> {
        self.substitute_no_sort(context, depth, env)?;
        self.sort();
        Ok(())
    }


    fn substitute_no_sort(&mut self, context : &InterpreterContext<S>, depth : i32, env : &Env) -> Result<(), ExecutionError> {

        //     channelsSub <- substitutePar[M].substituteNoSort(term.chan)
        if let Some(ref mut chan) = self.chan {
            chan.substitute_no_sort(context, depth, env)?;
        }

        //     parsSub     <- term.data.toVector.traverse(substitutePar[M].substituteNoSort(_))
        for p in &mut self.data {
            p.substitute_no_sort(context, depth, env)?;
        }


        //     send = Send(
        //       chan = channelsSub,
        //       data = parsSub,
        //       persistent = term.persistent,
        //       locallyFree = term.locallyFree.until(env.shift),
        //       connectiveUsed = term.connectiveUsed
        //     )
        if let Some(ref mut bitset) = self.locally_free {
            bitset.truncate(env.shift);
        }

        Ok(())
    }
    
}
