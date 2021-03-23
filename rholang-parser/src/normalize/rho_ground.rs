

use super::super::bnfc;
use super::*;


impl super::Normalizer {

    pub fn normalize_ground(&mut self, proc : &RawProc, input: &ProcVisitInputs) -> Result<ProcVisitOutputs, CompiliationError> {

        let ground_ = unsafe { proc.u.pground_.ground_ };
        if ground_ == 0 as bnfc::Ground {
            return Err(CompiliationError::new_null_pointer("pground_.ground_"));
        }

        let ground = unsafe { *ground_ };
        let expression = match ground.kind {
            bnfc::Ground__is_GroundInt => self.normalize_ground_int(&ground),
            _ => {
                Err(CompiliationError::new_unrecognized_data("ground_.kind", ground.kind, proc.line_number, proc.char_number))
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

    fn normalize_ground_int(&mut self, ground : &RawGround) -> Result<Expr, CompiliationError> {
        let longliteral_ = unsafe { ground.u.groundint_.longliteral_ };
        if longliteral_ == 0 as bnfc::LongLiteral {
            return Err(CompiliationError::new_null_pointer("groundint_.longliteral_"));
        }

        let literal_long = self.get_string(longliteral_).map_err(|err| {
            CompiliationError::new_utf8_error(&err, ground.line_number, ground.char_number)
        })?;

        let source_position = SourcePosition::new( ground.line_number, ground.char_number, literal_long.len());

        let long : i64 = match literal_long.parse::<i64>() {
            Err(_) => {
                self.syntax_errors.push(SyntaxError::new_integer_number_error(&literal_long, source_position));
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