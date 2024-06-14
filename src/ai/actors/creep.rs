use std::cell::{Cell, RefCell};
use std::rc::Rc;
use ecow::EcoString;
use getset::{CopyGetters, Getters};
use screeps::{Creep, SharedCreepProperties};
use crate::ai::fsm::machine::FSMachine;
use crate::ai::fsm::state::State;

#[derive()]
pub struct CreepActor {
    creep: Creep,
    fsm: RefCell<FSMachine<CreepActor>>
}

impl CreepActor {
    pub fn new(creep: Creep) -> Rc<RefCell<Self>> {
        Rc::new_cyclic(| actor | {
            let idle = Rc::new(IdleState);
            let mining = Rc::new(MiningState);
            let mut fsm = FSMachine::<CreepActor>::new(
                actor.clone(),
                idle.clone(),
                vec![idle.clone(), mining.clone()]
            );
            fsm.set_condition(idle.clone(), mining.clone(), Rc::new(| a | a.creep.name() == "TEST"));
            RefCell::new(Self {
                creep,
                fsm: RefCell::new(fsm)
            })
        })
    }
    pub fn process(&self) {
        self.fsm.borrow_mut().process()
    }
}


struct IdleState;
impl State for IdleState {
    type Actor = CreepActor;

    fn id(&self) -> EcoString {
        static ID: EcoString = EcoString::inline("idle");
        ID.clone()
    }

    fn run(&self, actor: &Self::Actor) {
        actor.creep.say("YES SIR", false).unwrap();
    }
}

struct MiningState;
impl State for MiningState {
    type Actor = CreepActor;

    fn id(&self) -> EcoString {
        static ID: EcoString = EcoString::inline("mine");
        ID.clone()
    }

    fn run(&self, actor: &Self::Actor) {
        actor.creep.say("MINING", false).unwrap();
    }
}