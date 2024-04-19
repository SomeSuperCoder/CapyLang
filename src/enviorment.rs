use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::values::Value;

pub struct Enviorment {
    parent: Option<Rc<RefCell<Enviorment>>>,
    variables: HashMap<String, Value>
}

impl Enviorment {
    pub fn new(parent: Option<Rc<RefCell<Enviorment>>>) -> Rc<RefCell<Self>> {
        Rc::new(
            RefCell::new(
                Self {
                    parent,
                    variables: HashMap::new()
                }
            )
        )
    }

    pub fn declare(me: Rc<RefCell<Self>>, name: String, value: Value) -> Value {
        if me.borrow().variables.get(&name).is_some() {
            panic!("Cannot declare variable {}, as it already exists", name)
        }

        me.borrow_mut().variables.insert(name, value.clone());

        return value
    }

    pub fn assign(me: Rc<RefCell<Self>>, name: String, value: Value) -> Value {
        let env = Enviorment::resolve(Rc::clone(&me), name.clone());

        env.borrow_mut().variables.insert(name, value.clone());

        value
    }

    pub fn lookup(me: Rc<RefCell<Self>>, name: String) -> Value {
        let env = Enviorment::resolve(Rc::clone(&me), name.clone());
        
        let tmp = env.borrow();
        
        tmp.variables.get(&name).unwrap().clone()
    }

    pub fn resolve(me: Rc<RefCell<Self>>, name: String) -> Rc<RefCell<Enviorment>> {
        if me.borrow().variables.get(&name).is_some() {
            Rc::clone(&me)
        } else {
            if let Some(parent) = &mut me.borrow_mut().parent {
                Self::resolve(Rc::clone(&parent), name)
            } else {
                panic!("NameError: Cannot resolve {}, as it does not exist", name)
            }
        }
    }

    
}
