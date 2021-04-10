use super::*;

impl Substitutable for Send {

    fn substitute(mut self, reducer : &DebruijnInterpreter, depth : i32, env : &Env) -> Result<Self, ExecutionError> {
       
        // substituteNoSort(term).flatMap(send => Sortable.sortMatch(send)).map(_.term)
        unimplemented!("Send::substitute")
        //Ok(self)
    }


    fn substitute_no_sort(mut self, reducer : &DebruijnInterpreter, depth : i32, env : &Env) -> Result<Self, ExecutionError> {


        // for {
        //     channelsSub <- substitutePar[M].substituteNoSort(term.chan)
        //     parsSub     <- term.data.toVector.traverse(substitutePar[M].substituteNoSort(_))
        //     send = Send(
        //       chan = channelsSub,
        //       data = parsSub,
        //       persistent = term.persistent,
        //       locallyFree = term.locallyFree.until(env.shift),
        //       connectiveUsed = term.connectiveUsed
        //     )
        //   } yield send

        unimplemented!("Send::substitute_no_sort")

        //Ok(self)
    }
    
}
