use super::*;

impl Substitutable for Match {

    fn substitute(mut self, reducer : &DebruijnInterpreter, depth : i32, env : &Env) -> Result<Self, ExecutionError> {
       
        // substituteNoSort(term).flatMap(mat => Sortable.sortMatch(mat)).map(_.term)
        unimplemented!("Match::substitute")
        //Ok(self)
    }


    fn substitute_no_sort(mut self, reducer : &DebruijnInterpreter, depth : i32, env : &Env) -> Result<Self, ExecutionError> {

        if self.target.is_none() {
            return Err(ExecutionError::new(ExecutionErrorKind::InvalidMatch, "No `target` in match"));
        }
        let target = self.target.take().unwrap();
        self.target.replace( target.substitute_no_sort(reducer, depth, env)? );


        // for {
        //     targetSub <- substitutePar[M].substituteNoSort(term.target)
        //     casesSub <- term.cases.toVector.traverse {
        //                  case MatchCase(_case, _par, freeCount) =>
        //                    for {
        //                      par <- substitutePar[M].substituteNoSort(_par)(
        //                              depth,
        //                              env.shift(freeCount)
        //                            )
        //                      subCase <- substitutePar[M].substituteNoSort(_case)(depth + 1, env)
        //                    } yield MatchCase(subCase, par, freeCount)
        //                }
        //     mat = Match(targetSub, casesSub, term.locallyFree.until(env.shift), term.connectiveUsed)
        //   } yield mat

        unimplemented!("Match::substitute_no_sort")

        //Ok(self)
    }
    
}
