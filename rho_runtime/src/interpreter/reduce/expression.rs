use std::sync::Arc;

use async_trait::async_trait;

use super::*;
use model::expr::ExprInstance;

struct ExprEvaluator {
    pub exp : Expr,
}

impl From<Expr> for ExprEvaluator {
    fn from(e: Expr) -> Self {
        ExprEvaluator { exp : e }
    }
}

impl From<Expr> for ThreadSafeEvaluator {
    fn from(e: Expr) -> Self {
        Box::new(ExprEvaluator { exp : e })
    }
}

#[async_trait]
impl AsyncEvaluator for ExprEvaluator {
    async fn evaluate(&self, _reducer : Arc<DebruijnInterpreter>) {
 
        
        
    }
}


impl DebruijnInterpreter {
    pub async fn evaluate_expression_to_par(exp : &Expr) {
        match &exp.expr_instance {
            Some(ExprInstance::EVarBody(EVar{ v : Some(var) })) => {
                unimplemented!("Some(expr::ExprInstance::EVarBody(EVar))");
            },
            Some(ExprInstance::EVarBody(EVar{ v : None })) => {
                panic!("EVar's v is None")
            },
            Some(ExprInstance::EMethodBody(EMethod {
                method_name,
                target : Some(target),
                arguments,
                ..
            })) => {
                unimplemented!("Some(ExprInstance::EMethodBody(EMethod))");
            },
            Some(ExprInstance::EMethodBody(EMethod { target : None, ..})) => {
                panic!("Invalid data, `expr_instance::EMethodBody::EMethod::target` is None");
            },
            None => {
                panic!("expr_instance is None")
            },
            _ => {

            }
        }
    }
}