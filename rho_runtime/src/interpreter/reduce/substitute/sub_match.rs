use super::*;

impl<S : Storage + std::marker::Send + std::marker::Sync> Substitutable<S> for Match {

    fn substitute(&mut self, context : &InterpreterContext<S>, depth : i32, env : &Env) -> Result<(), ExecutionError> {
       
        // substituteNoSort(term).flatMap(mat => Sortable.sortMatch(mat)).map(_.term)
        self.substitute_no_sort(context, depth, env)?;
        self.sort();
        Ok(())
    }


    fn substitute_no_sort(&mut self, context : &InterpreterContext<S>, depth : i32, env : &Env) -> Result<(), ExecutionError> {

        // targetSub <- substitutePar[M].substituteNoSort(term.target)
        match self.target {
            Some(ref mut target) => {
                target.substitute_no_sort(context, depth, env)?;
            },
            _ => {
                return Err(ExecutionError::new(ExecutionErrorKind::InvalidMatch, "No `target` in match"));
            }
        };

        for case in &mut self.cases {
            
            // par <- substitutePar[M].substituteNoSort(_par)(
            //         depth,
            //         env.shift(freeCount)
            //     )
            if let Some(source) = &mut case.source {
                let new_env = env.clone_then_shift(case.free_count as usize);
                source.substitute_no_sort(context, depth, &new_env)?;
            }

            // subCase <- substitutePar[M].substituteNoSort(_case)(depth + 1, env)
            if let Some(pattern) = &mut case.pattern {
                pattern.substitute_no_sort(context, depth + 1, env)?;
            }

        }
 

        // term.locallyFree.until(env.shift)
        if let Some(bitset) = &mut self.locally_free {
            bitset.truncate(env.shift);
        }

        Ok(())
    }
    
}
