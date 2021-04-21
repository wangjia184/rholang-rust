use smallvec::SmallVec;

use super::*;

pub type EnvVector<T> = SmallVec<[T; 6]>;

// Environment Model of Evaluation
#[derive(Default, Debug, Clone)]
pub struct Env<T = Par> where T : Clone + std::marker::Send {
    level : usize,
    pub shift : usize,

    // need be optimized
    // Arc is used here to avoid duplicated instance.
    // Be careful when we want to change the binding in a frame
    // Using Arc::make_mut() for Copy-on-Write
    bindings : EnvVector<T>
}

impl<T> Env<T> where T : Clone + std::marker::Send {


    pub fn put(&mut self, t : T) {
        self.bindings.push(t);
        self.level += 1;
    }

    pub fn create(vector : EnvVector<T>) -> Self{
        Self {
            level : vector.len(),
            shift : 0,
            bindings : vector,
        } 
    }


    pub fn get<'a>(&'a self, k : i32) -> Option<&'a T> {
        let index = self.level + self.shift - k as usize - 1;
        if index < self.bindings.len() {
            return Some(&self.bindings[index as usize]);
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

    let mut env = Env::<Par>::default();
    env.put(par1);
    env.put(par2);
    env.put(par3);

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