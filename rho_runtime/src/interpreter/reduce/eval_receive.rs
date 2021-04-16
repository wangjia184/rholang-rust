use super::*;


#[async_trait]
impl AsyncEvaluator for Receive {
    async fn evaluate(&mut self, context : &Arc<InterpreterContext>, env : &Env) -> Result<(), ExecutionError> {
        context.may_raise_aborted_error()?;

        // charge[M](RECEIVE_EVAL_COST)
        let mut binds : Vec<(BindPattern, Par)> = Vec::with_capacity(self.binds.len());
        for receive_bind in &mut self.binds {
            let unbundled_source = match receive_bind.source.take() {
                Some(mut source) => {
                    source.evaluate_nested_expressions(context, env).await?;

                    // substituteAndCharge
                    source.substitute(context, 0, env)?;

                    match source.single_bundle() {
                        Some(ref mut bundle) => {
                            if !bundle.read_flag {
                                return Err((ExecutionErrorKind::NonReadableChannel, "Trying to read from non-readable channel.").into());
                            }
                            match bundle.body.take() {
                                Some(body) => body,
                                None => return Err((ExecutionErrorKind::InvalidBundle, "Bundle::body is missing").into()),
                            }
                        },
                        None => source,
                    }
                },
                None => return Err((ExecutionErrorKind::InvalidReceive, "ReceiveBind::source is missing").into()),
            };

            for pattern in &mut receive_bind.patterns {
                // substituteAndCharge
                pattern.substitute(context, 1, env)?;
                
            }
            
            // patterns
            let mut bind_pattern = BindPattern {
                patterns : Vec::with_capacity(receive_bind.patterns.len()),
                remainder : receive_bind.remainder.take(),
                free_count : receive_bind.free_count,
            };
            bind_pattern.patterns.append(&mut receive_bind.patterns);

            binds.push((bind_pattern, unbundled_source));
        }// ReceiveBind

        let body = match self.body.take() {
            Some(mut par) => {
                let env = env.clone_then_shift(self.bind_count as usize);
                par.substitute_no_sort(context, 0, &env)?;
                par
            },
            None => return Err((ExecutionErrorKind::InvalidReceive, "Receive::body is missing").into()),
        };

        // _ <- consume(
        //     binds,
        //     ParWithRandom(substBody, rand),
        //     receive.persistent,
        //     receive.peek
        //   )

        Ok(())
    }
}