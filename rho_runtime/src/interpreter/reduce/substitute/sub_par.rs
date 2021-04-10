use super::*;

impl Substitutable for Par {
    fn substitute(&mut self, reducer : &DebruijnInterpreter, depth : i32, env : &Env) -> Result<(), ExecutionError> {
        Ok(())
    }
    fn substitute_no_sort(&mut self, reducer : &DebruijnInterpreter, depth : i32, env : &Env) -> Result<(), ExecutionError> {

        let par_contains_expressions = substitute_expressions(&mut self.exprs, reducer, depth, env)?;
        let par_contains_connectives = substitute_connectives(&mut self.connectives, reducer, depth, env)?;


        for s in &mut self.sends {
            s.substitute_no_sort(reducer, depth, env)?;
        }
        for b in &mut self.bundles {
            b.substitute_no_sort(reducer, depth, env)?;
        }
        for r in &mut self.receives {
            r.substitute_no_sort(reducer, depth, env)?;
        }
        for n in &mut self.news {
            n.substitute_no_sort(reducer, depth, env)?;
        }
        for m in &mut self.matches {
            m.substitute_no_sort(reducer, depth, env)?;
        }
 

        Ok(())
    }

    
}

fn substitute_expressions(vector : &mut Vec<Expr>, reducer : &DebruijnInterpreter, depth : i32, env : &Env) -> Result<Par, ExecutionError> {
    // the scala code use fold(), here we use imperative style instead to avoid extra allocation
    // note that we dont reverse here, thus it appends instead of prepending

    let mut par = Par::default();

    while !vector.is_empty() {
        let mut expression = vector.remove(0);

        match expression.expr_instance {
            Some(ExprInstance::EVarBody(EVar{ v : Some(var) })) => {
                // case EVarBody(e) =>
                //     maybeSubstitute[M](e).map {
                //         case Left(_e)    => par.prepend(_e, depth)
                //         case Right(_par) => _par ++ par
                //     }
                let _ = var;
                unimplemented!("Some(ExprInstance::EVarBody(EVar))");
            },
            Some(ExprInstance::EVarBody(EVar{ v : None })) => {
                return Err(ExecutionError::new(ExecutionErrorKind::InvalidExpression,
                    "Expr::expr_instance::EVarBody::Var is None")
                );
            },
            None => {
                return Err(ExecutionError::new(ExecutionErrorKind::InvalidExpression,
                    "Expr::expr_instance is None")
                );
            }
            _ => {
                expression.substitute(reducer, depth, env)?;
                par.append_expr(expression, depth);
            }
        }
    }

    Ok(par)
}


fn substitute_connectives(vector : &mut Vec<Connective>, reducer : &DebruijnInterpreter, depth : i32, env : &Env) -> Result<Par, ExecutionError> {

    if !vector.is_empty() {
        unimplemented!("substitute_connectives() is not implemented");
    }

    Ok(Par::default())

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