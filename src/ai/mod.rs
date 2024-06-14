use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use screeps::{game, SharedCreepProperties};
use screeps::game::creeps;
use crate::ai::actors::creep::CreepActor;

pub mod actor;
pub mod actors;
pub mod fsm;

thread_local! {
    static CREEPS: RefCell<HashMap<String, Rc<RefCell<CreepActor>>>> = RefCell::new(HashMap::new())
}

pub fn test_ai() {
    CREEPS.with_borrow_mut(| creeps | {
        for cr in game::creeps().values() {
            creeps.insert(cr.name(), CreepActor::new(cr));
        }

        for (_, actor) in creeps {
            let mut x = actor.borrow_mut();
            x.process()
        }
    });
}