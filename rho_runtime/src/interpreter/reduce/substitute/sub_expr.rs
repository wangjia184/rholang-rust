use super::*;

impl Substitutable for Expr {

    fn substitute(&mut self, reducer : &DebruijnInterpreter, depth : i32, env : &Env) -> Result<(), ExecutionError> {
        
        let func = |par : &mut Par| { 
            par.substitute(reducer, depth, env)
        };

        dispatch( self, func)
    }


    fn substitute_no_sort(&mut self, reducer : &DebruijnInterpreter, depth : i32, env : &Env) -> Result<(), ExecutionError> {

        let func = |par : &mut Par| {
            par.substitute_no_sort(reducer, depth, env)
        };

        dispatch( self, func)
    }
    
}

macro_rules! unary_expr{
    ($x:expr, $f:expr)=>{
        {
            if let Some(ref mut par) = $x.p {
                $f(par)?;
            }
        }
    }
}

macro_rules! binary_expr{
    ($x:expr, $f:expr)=>{
        {
            if let Some(ref mut par) = $x.p1 {
                $f(par)?;
            }
            if let Some(ref mut par) = $x.p2 {
                $f(par)?;
            }
        }
    }
}

fn dispatch<F>(expression : &mut Expr,  func : F) -> Result<(), ExecutionError> 
        where F : Fn(&mut Par) -> Result<(), ExecutionError>
{

    match expression.expr_instance {
        Some(ExprInstance::ENotBody(ref mut instance)) if instance.p.is_some() => {
            unary_expr!(instance, func)
        },
        Some(ExprInstance::ENegBody(ref mut instance)) if instance.p.is_some() => {
            unary_expr!(instance, func)
        },
        Some(ExprInstance::EMultBody(ref mut instance)) if instance.p1.is_some() && instance.p2.is_some() => {
            binary_expr!(instance, func)
        },
        Some(ExprInstance::EDivBody(ref mut instance)) if instance.p1.is_some() && instance.p2.is_some() => {
            binary_expr!(instance, func)
        },
        Some(ExprInstance::EModBody(ref mut instance)) if instance.p1.is_some() && instance.p2.is_some() => {
            binary_expr!(instance, func)
        },
        Some(ExprInstance::EPercentPercentBody(ref mut instance)) if instance.p1.is_some() && instance.p2.is_some() => {
            binary_expr!(instance, func)
        },
        Some(ExprInstance::EPlusBody(ref mut instance)) if instance.p1.is_some() && instance.p2.is_some() => {
            binary_expr!(instance, func)
        },
        Some(ExprInstance::EMinusBody(ref mut instance)) if instance.p1.is_some() && instance.p2.is_some() => {
            binary_expr!(instance, func)
        },
        Some(ExprInstance::EPlusPlusBody(ref mut instance)) if instance.p1.is_some() && instance.p2.is_some() => {
            binary_expr!(instance, func)
        },
        Some(ExprInstance::EMinusMinusBody(ref mut instance)) if instance.p1.is_some() && instance.p2.is_some() => {
            binary_expr!(instance, func)
        },
        Some(ExprInstance::ELtBody(ref mut instance)) if instance.p1.is_some() && instance.p2.is_some() => {
            binary_expr!(instance, func)
        },
        Some(ExprInstance::ELteBody(ref mut instance)) if instance.p1.is_some() && instance.p2.is_some() => {
            binary_expr!(instance, func)
        },
        Some(ExprInstance::EGtBody(ref mut instance)) if instance.p1.is_some() && instance.p2.is_some() => {
            binary_expr!(instance, func)
        },
        Some(ExprInstance::EGteBody(ref mut instance)) if instance.p1.is_some() && instance.p2.is_some() => {
            binary_expr!(instance, func)
        },
        Some(ExprInstance::EEqBody(ref mut instance)) if instance.p1.is_some() && instance.p2.is_some() => {
            binary_expr!(instance, func)
        },
        Some(ExprInstance::ENeqBody(ref mut instance)) if instance.p1.is_some() && instance.p2.is_some() => {
            binary_expr!(instance, func)
        },
        Some(ExprInstance::EAndBody(ref mut instance)) if instance.p1.is_some() && instance.p2.is_some() => {
            binary_expr!(instance, func)
        },
        Some(ExprInstance::EOrBody(ref mut instance)) if instance.p1.is_some() && instance.p2.is_some() => {
            binary_expr!(instance, func)
        },
        Some(ExprInstance::EMatchesBody(ref mut ematches)) if ematches.target.is_some() && ematches.pattern.is_some() => {
            if let Some(ref mut par) = ematches.target {
                func(par)?;
            }
            if let Some(ref mut par) = ematches.pattern {
                func(par)?;
            }
        },
        Some(ExprInstance::EListBody(ref mut _elist)) => {
            // for {
            //     pss <- ps.toVector
            //             .traverse(p => s1(p))
            //     newLocallyFree = locallyFree.until(env.shift)
            // } yield Expr(exprInstance = EListBody(EList(pss, newLocallyFree, connectiveUsed, rem)))
            unimplemented!("ExprInstance::EListBody is not implemented")
        },
        Some(ExprInstance::ETupleBody(ref mut _etuple)) => {
            // for {
            //     pss <- ps.toVector
            //             .traverse(p => s1(p))
            //     newLocallyFree = locallyFree.until(env.shift)
            // } yield Expr(exprInstance = ETupleBody(ETuple(pss, newLocallyFree, connectiveUsed)))
            unimplemented!("ExprInstance::ETupleBody is not implemented")
        },
        Some(ExprInstance::ESetBody(ref mut _eset)) => {
            // for {
            //     pss <- shs.sortedPars
            //             .traverse(p => s1(p))
            //   } yield Expr(
            //     exprInstance = ESetBody(
            //       ParSet(
            //         SortedParHashSet(pss.toSeq),
            //         connectiveUsed,
            //         locallyFree.map(_.until(env.shift)),
            //         remainder
            //       )
            //     )
            //   )
            unimplemented!("ExprInstance::ESetBody is not implemented")
        },
        Some(ExprInstance::EMapBody(ref mut _emap)) => {
            // for {
            //     kvps <- spm.sortedList.traverse {
            //              case (p1, p2) =>
            //                for {
            //                  pk1 <- s1(p1)
            //                  pk2 <- s1(p2)
            //                } yield (pk1, pk2)
            //            }
            //   } yield Expr(
            //     exprInstance = EMapBody(
            //       ParMap(kvps, connectiveUsed, locallyFree.map(_.until(env.shift)), remainder)
            //     )
            //   )
            unimplemented!("ExprInstance::EMapBody is not implemented")
        },

        Some(ExprInstance::EMethodBody(ref mut _emethod)) => {
            // for {
            //     subTarget    <- s1(target)
            //     subArguments <- arguments.toVector.traverse(p => s1(p))
            //   } yield Expr(
            //     exprInstance = EMethodBody(
            //       EMethod(
            //         mtd,
            //         subTarget,
            //         subArguments,
            //         locallyFree.until(env.shift),
            //         connectiveUsed
            //       )
            //     )
            //   )
            unimplemented!("ExprInstance::EMethodBody is not implemented")
        },

        _ => {
            warn!("Uncaptrued expression in substitue. {:#?}", &expression.expr_instance);
        }
    }
    
    Ok(())
}