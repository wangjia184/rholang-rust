

use super::super::bnfc::*;
use super::super::errors::*;
use super::*;
use crate::model::*;

impl super::Normalizer {

    pub fn normalize_ground(&mut self, proc : &RawProc, input: &ProcVisitInputs) -> Result<ProcVisitOutputs, CompliationError> {

        let ground_ = unsafe { proc.u.pground_.ground_ };
        if ground_ == 0 as bnfc::Ground {
            return Err(CompliationError::NullPointer("pground_.ground_".to_string()));
        }

        let ground = unsafe { *ground_ };
        let expression = match ground.kind {
            Ground__is_GroundInt => self.normalize_ground_int(&ground),
            _ => {
                Err(CompliationError::UnrecognizedKind(ground.kind, "ground_.kind".to_string()))
            }
        }?;

        let par = input.par.clone_then_prepend_expr(expression, input.env.depth());
        Ok(
            ProcVisitOutputs {
                par : par, 
                known_free : (*input.known_free).clone() 
            }
        )
    }

    fn normalize_ground_int(&mut self, ground : &RawGround) -> Result<Expr, CompliationError> {
        let longliteral_ = unsafe { ground.u.groundint_.longliteral_ };
        if longliteral_ == 0 as bnfc::LongLiteral {
            return Err(CompliationError::NullPointer("groundint_.longliteral_".to_string()));
        }

        let literal_long = self.get_string(longliteral_).map_err(|err| {
            CompliationError::SourceUtf8Error(ground.line_number, ground.char_number, err)
        })?;

        let source_position = SourcePosition::new( ground.line_number, ground.char_number, literal_long.len());

        let long : i64 = match literal_long.parse::<i64>() {
            Err(_) => {
                self.syntax_errors.push(
                    (
                        SyntaxError::IntegerNumberError(literal_long),
                        Some(source_position),
                        None,
                    )
                );
                0 // set number to zero so that we can continue parse the AST
            },
            Ok(num) => num
        };

        Ok(
            Expr {
                expr_instance : Some(expr::ExprInstance::GInt(long))
            }
        )
    }
}