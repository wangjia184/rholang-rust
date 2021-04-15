use super::*;

impl Substitutable for New {

    fn substitute(&mut self, context : &InterpreterContext, depth : i32, env : &Env) -> Result<(), ExecutionError> {
       
        // substituteNoSort(term).flatMap(newSub => Sortable.sortMatch(newSub)).map(_.term)
        self.substitute_no_sort(context, depth, env)?;
        self.sort();
        Ok(())
    }


    fn substitute_no_sort(&mut self, context : &InterpreterContext, depth : i32, env : &Env) -> Result<(), ExecutionError> {


        // substitutePar[M]
        //   .substituteNoSort(term.p)(depth, env.shift(term.bindCount))
        //   .map(
        //     newSub =>
        //       New(
        //         bindCount = term.bindCount,
        //         p = newSub,
        //         uri = term.uri,
        //         injections = term.injections,
        //         locallyFree = term.locallyFree.until(env.shift)
        //       )
        //   )

        unimplemented!("New::substitute_no_sort")

        //Ok(self)
    }
    
}
