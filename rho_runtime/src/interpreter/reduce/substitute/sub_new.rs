use super::*;

impl Substitutable for New {

    fn substitute(mut self, reducer : &DebruijnInterpreter, depth : i32, env : &Env) -> Result<Self, ExecutionError> {
       
        // substituteNoSort(term).flatMap(newSub => Sortable.sortMatch(newSub)).map(_.term)
        unimplemented!("New::substitute")
        //Ok(self)
    }


    fn substitute_no_sort(mut self, reducer : &DebruijnInterpreter, depth : i32, env : &Env) -> Result<Self, ExecutionError> {


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
