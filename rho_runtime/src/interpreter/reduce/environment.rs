
use std::rc::Rc;
use super::*;

// Environment Model of Evaluation
#[derive(Default)]
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

