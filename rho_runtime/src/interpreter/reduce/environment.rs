
use std::rc::Rc;
use super::*;


// Environment Model of Evaluation
#[derive(Default, Debug)]
pub struct Env<T = Par> where T : Clone {
    level : usize,
    shift : usize,

    // Rc is used here to avoid duplicated instance.
    // Be careful when we want to change the binding in a frame
    // Using Rc::make_mut() for Copy-on-Write
    bindings : Rc<Vec<Rc<T>>>
}

impl<T> Env<T> where T : Clone {
    // create a new frame by adding a new binding
    pub fn clone_then_put(&self, t : T) -> Self {
        let mut new_bindings = self.bindings.clone();
        let vector = Rc::make_mut(&mut new_bindings);
        vector.push(Rc::new(t));
        Self {
            level : self.level + 1,
            shift : self.shift,
            bindings : new_bindings,
        }
    }


    pub fn get(&self, k : usize) -> Option<Rc<T>> {
        let index = self.level + self.shift - k - 1;
        if index < self.bindings.len() {
            return Some(self.bindings[index as usize].clone());
        }
        None
    }

    pub fn clone_then_shift(&self, j : usize) -> Self {
        Self {
            level : self.level,
            shift : self.shift + j,
            bindings : self.bindings.clone(),
        }
    }
}






#[test]
fn env_frame_should_always_be_inserted_at_the_next_available_level_index() {
    let mut par1 = Par::default();
    par1.exprs.push(Expr {
        expr_instance : Some(expr::ExprInstance::GInt(1))
    });
    let mut par2 = Par::default();
    par2.exprs.push(Expr {
        expr_instance : Some(expr::ExprInstance::GInt(2))
    });
    let mut par3 = Par::default();
    par3.exprs.push(Expr {
        expr_instance : Some(expr::ExprInstance::GInt(3))
    });

    let env = Env::<Par>::default().clone_then_put(par1).clone_then_put(par2).clone_then_put(par3);

    match env {
        Env { level : 3, shift: 0, bindings } => {
            assert_eq!( bindings.len(), 3);
            match &bindings[0].exprs[0] {
                Expr { expr_instance: Some(expr::ExprInstance::GInt(i)) } => {
                    assert_eq!(*i, 1);
                },
                _ => {
                    panic!("{:#?}", &bindings[0].exprs[0]);
                }
            };
            match &bindings[1].exprs[0] {
                Expr { expr_instance: Some(expr::ExprInstance::GInt(i)) } => {
                    assert_eq!(*i, 2);
                },
                _ => {
                    panic!("{:#?}", &bindings[1].exprs[0]);
                }
            };
            match &bindings[2].exprs[0] {
                Expr { expr_instance: Some(expr::ExprInstance::GInt(i)) } => {
                    assert_eq!(*i, 3);
                },
                _ => {
                    panic!("{:#?}", &bindings[2].exprs[0]);
                }
            };
        },
        _ => {
            panic!("{:#?}", &env);
        }
    }
}