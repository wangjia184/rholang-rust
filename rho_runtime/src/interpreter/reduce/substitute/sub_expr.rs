use super::*;

impl Substitutable for Expr {

    fn substitute(self, reducer : &DebruijnInterpreter, depth : i32, env : &Env) -> Result<Self, ExecutionError> {
        
        let func1 = |par : Par| { par.substitute(reducer, depth, env) };

        let func2 = | left : Par, right : Par | -> Result<(Par, Par), ExecutionError> {
            Ok(( 
                left.substitute(reducer, depth, env)?,
                right.substitute(reducer, depth, env)?
            ))
        };

        dispatch( self, func1, func2)
    }


    fn substitute_no_sort(self, reducer : &DebruijnInterpreter, depth : i32, env : &Env) -> Result<Self, ExecutionError> {

        let func1 = |par : Par| { par.substitute_no_sort(reducer, depth, env) };

        let func2 = | left : Par, right : Par | -> Result<(Par, Par), ExecutionError> {
            Ok(( 
                left.substitute_no_sort(reducer, depth, env)?,
                right.substitute_no_sort(reducer, depth, env)?
            ))
        };

        dispatch( self, func1, func2)
    }
    
}

macro_rules! unary_expr{
    ($x:expr, $f:expr)=>{
        {
            // let par = enot.p.take().unwrap();
            // let par = func1(par)?;
            // enot.p = Some(par);
            $x.p = Some($f($x.p.take().unwrap())?);
        }
    }
}

macro_rules! binary_expr{
    ($x:expr, $f:expr)=>{
        {
            let (left, right) = $f( $x.p1.take().unwrap(), $x.p2.take().unwrap())?;
            $x.p1 = Some(left);
            $x.p2 = Some(right);
        }
    }
}

fn dispatch<F1, F2>(mut expression : Expr,  func1 : F1, func2 : F2 ) -> Result<Expr, ExecutionError> 
        where F1 : Fn(Par) -> Result<Par, ExecutionError>,
        F2 : Fn(Par, Par) -> Result<(Par, Par), ExecutionError>
{

    match expression.expr_instance {
        Some(ExprInstance::ENotBody(ref mut instance)) if instance.p.is_some() => {
            unary_expr!(instance, func1)
        },
        Some(ExprInstance::ENegBody(ref mut instance)) if instance.p.is_some() => {
            unary_expr!(instance, func1)
        },
        Some(ExprInstance::EMultBody(ref mut instance)) if instance.p1.is_some() && instance.p2.is_some() => {
            binary_expr!(instance, func2)
        },
        Some(ExprInstance::EDivBody(ref mut instance)) if instance.p1.is_some() && instance.p2.is_some() => {
            binary_expr!(instance, func2)
        },
        Some(ExprInstance::EModBody(ref mut instance)) if instance.p1.is_some() && instance.p2.is_some() => {
            binary_expr!(instance, func2)
        },
        Some(ExprInstance::EPercentPercentBody(ref mut instance)) if instance.p1.is_some() && instance.p2.is_some() => {
            binary_expr!(instance, func2)
        },
        Some(ExprInstance::EPlusBody(ref mut instance)) if instance.p1.is_some() && instance.p2.is_some() => {
            binary_expr!(instance, func2)
        },
        Some(ExprInstance::EMinusBody(ref mut instance)) if instance.p1.is_some() && instance.p2.is_some() => {
            binary_expr!(instance, func2)
        },
        Some(ExprInstance::EPlusPlusBody(ref mut instance)) if instance.p1.is_some() && instance.p2.is_some() => {
            binary_expr!(instance, func2)
        },
        Some(ExprInstance::EMinusMinusBody(ref mut instance)) if instance.p1.is_some() && instance.p2.is_some() => {
            binary_expr!(instance, func2)
        },
        Some(ExprInstance::ELtBody(ref mut instance)) if instance.p1.is_some() && instance.p2.is_some() => {
            binary_expr!(instance, func2)
        },
        Some(ExprInstance::ELteBody(ref mut instance)) if instance.p1.is_some() && instance.p2.is_some() => {
            binary_expr!(instance, func2)
        },
        Some(ExprInstance::EGtBody(ref mut instance)) if instance.p1.is_some() && instance.p2.is_some() => {
            binary_expr!(instance, func2)
        },
        Some(ExprInstance::EGteBody(ref mut instance)) if instance.p1.is_some() && instance.p2.is_some() => {
            binary_expr!(instance, func2)
        },
        Some(ExprInstance::EEqBody(ref mut instance)) if instance.p1.is_some() && instance.p2.is_some() => {
            binary_expr!(instance, func2)
        },
        Some(ExprInstance::ENeqBody(ref mut instance)) if instance.p1.is_some() && instance.p2.is_some() => {
            binary_expr!(instance, func2)
        },
        Some(ExprInstance::EAndBody(ref mut instance)) if instance.p1.is_some() && instance.p2.is_some() => {
            binary_expr!(instance, func2)
        },
        Some(ExprInstance::EOrBody(ref mut instance)) if instance.p1.is_some() && instance.p2.is_some() => {
            binary_expr!(instance, func2)
        },
        Some(ExprInstance::EMatchesBody(ref mut ematches)) if ematches.target.is_some() && ematches.pattern.is_some() => {
            let (left, right) = func2( ematches.target.take().unwrap(), ematches.pattern.take().unwrap())?;
            ematches.target = Some(left);
            ematches.pattern = Some(right);
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
    
    Ok(expression)
}