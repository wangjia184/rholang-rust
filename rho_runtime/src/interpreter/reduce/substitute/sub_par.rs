use super::*;

impl<S : Storage + std::marker::Send + std::marker::Sync> Substitutable<S> for Par {
    fn substitute(&mut self, context : &InterpreterContext<S>, depth : i32, env : &Env) -> Result<(), ExecutionError> {
        // substituteNoSort(term).flatMap(par => Sortable.sortMatch(par)).map(_.term)
        self.substitute_no_sort(context, depth, env)?;
        self.sort();
        Ok(())
    }
    fn substitute_no_sort(&mut self, context : &InterpreterContext<S>, depth : i32, env : &Env) -> Result<(), ExecutionError> {

        for s in &mut self.sends {
            s.substitute_no_sort(context, depth, env)?;
        }
        for b in &mut self.bundles {
            b.substitute_no_sort(context, depth, env)?;
        }
        for r in &mut self.receives {
            r.substitute_no_sort(context, depth, env)?;
        }
        for n in &mut self.news {
            n.substitute_no_sort(context, depth, env)?;
        }
        for m in &mut self.matches {
            m.substitute_no_sort(context, depth, env)?;
        }

        if let Some(ref mut bitset) = self.locally_free {
            bitset.truncate(env.shift); // term.locallyFree.until(env.shift)  need test
        }
 
        substitute_connectives_in_par(self, context, depth, env)?;
        substitute_expressions_in_par(self, context, depth, env)?;
        Ok(())
    }

    
}

enum VarOrPar<'a> {
    Var(Var),
    ParRef(&'a Par)
}

fn may_substitute_var<'a>(var : Var, depth : i32, env : &'a Env) -> Result<VarOrPar<'a>, ExecutionError> {
    if depth != 0{
        Ok(VarOrPar::Var(var))
    } else {
        match var.var_instance {
            Some(VarInstance::BoundVar(index)) => {
                match env.get(index) {
                    Some(par) => Ok(VarOrPar::ParRef(par)),
                    None => Ok(VarOrPar::Var(var)),
                }
            },
            _ => {
                return Err((ExecutionErrorKind::IllegalSubstitution, format!("Illegal Substitution {:#?}", &var)).into());
            }
        }
    }
    
    
}

fn substitute_expressions_in_par<S>(par : &mut Par, context : &InterpreterContext<S>, depth : i32, env : &Env) -> Result<(), ExecutionError> 
    where S : Storage + std::marker::Send + std::marker::Sync
{
    // the scala code use fold(), here we use imperative style instead to avoid extra allocation
 
    // move exprs into a new vector, consider replace Vec with SmallVec after benchmark
    let mut expressions : Vec<Expr> = Vec::with_capacity(par.exprs.len());
    expressions.append(&mut par.exprs);

    for mut expression in expressions {
        match &mut expression.expr_instance {

            // case EVarBody(e) =>
            //     maybeSubstitute[M](e).map {
            //         case Left(_e)    => par.prepend(_e, depth)
            //         case Right(_par) => _par ++ par
            //     }

            Some(ExprInstance::EVarBody(evar)) => {
                match evar.v.take() {
                    Some(var) => {
                        match may_substitute_var(var, depth, env)? {
                            VarOrPar::Var(v) => {
                                par.append_expr(Expr { 
                                        expr_instance : Some(ExprInstance::EVarBody(EVar{
                                            v : Some(v)
                                        }))
                                    }
                                    , depth 
                                );
                            },
                            VarOrPar::ParRef(p) => {
                                par.append(p);
                            }
                        };
                    },
                    None => return Err((ExecutionErrorKind::InvalidExpression,"EVar::v is missing").into()),
                };
            },
            None => {
                return Err((ExecutionErrorKind::InvalidExpression,"Expr::expr_instance is missing").into());
            }
            _ => {
                expression.substitute_no_sort(context, depth, env)?;
                par.append_expr(expression, depth);
            }
        }
    }
    
    Ok(())
}


fn substitute_connectives_in_par<S>(par : &mut Par, context : &InterpreterContext<S>, depth : i32, env : &Env) -> Result<(), ExecutionError>
    where S : Storage  + std::marker::Send + std::marker::Sync
{

    if !par.connectives.is_empty() {
        unimplemented!("substitute_connectives() is not implemented");
    }

    Ok(())

    // conns.toList.reverse.foldM(VectorPar()) { (par, conn) =>
    //     conn.connectiveInstance match {
    //       case VarRefBody(v) =>
    //         maybeSubstitute[M](v).map {
    //           case Left(_)       => par.prepend(conn, depth)
    //           case Right(newPar) => newPar ++ par
    //         }
    //       case ConnectiveInstance.Empty => par.pure[M]
    //       case ConnAndBody(ConnectiveBody(ps)) =>
    //         ps.toVector
    //           .traverse(substitutePar[M].substituteNoSort(_))
    //           .map(ps => par.prepend(Connective(ConnAndBody(ConnectiveBody(ps))), depth))
    //       case ConnOrBody(ConnectiveBody(ps)) =>
    //         ps.toVector
    //           .traverse(substitutePar[M].substituteNoSort(_))
    //           .map(ps => par.prepend(Connective(ConnOrBody(ConnectiveBody(ps))), depth))
    //       case ConnNotBody(p) =>
    //         substitutePar[M]
    //           .substituteNoSort(p)
    //           .map(p => Connective(ConnNotBody(p)))
    //           .map(par.prepend(_, depth))
    //       case c: ConnBool      => par.prepend(Connective(c), depth).pure[M]
    //       case c: ConnInt       => par.prepend(Connective(c), depth).pure[M]
    //       case c: ConnString    => par.prepend(Connective(c), depth).pure[M]
    //       case c: ConnUri       => par.prepend(Connective(c), depth).pure[M]
    //       case c: ConnByteArray => par.prepend(Connective(c), depth).pure[M]
    //     }
}