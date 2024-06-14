use crate::ai::fsm::state::State;
use ecow::EcoString;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

type RcState<A> = Rc<dyn State<Actor = A>>;
type WeakState<A> = Weak<dyn State<Actor = A>>;
type RcCondition<A> = Rc<dyn Fn(&A) -> bool>;

pub struct FSMachine<A> {
    actor: Weak<RefCell<A>>,
    current_state: WeakState<A>,
    states: Vec<RcState<A>>,
    conditions: HashMap<(EcoString, EcoString), RcCondition<A>>,
}

impl<A> FSMachine<A> {
    pub fn new(
        actor: Weak<RefCell<A>>,
        initial_state: RcState<A>,
        states: Vec<RcState<A>>,
    ) -> Self {
        Self {
            actor,
            current_state: Rc::downgrade(&initial_state),
            states,
            conditions: HashMap::new(),
        }
    }

    pub fn set_condition(
        &mut self,
        state_from: RcState<A>,
        state_to: RcState<A>,
        condition: RcCondition<A>,
    ) {
        self.conditions
            .insert((state_from.id(), state_to.id()), condition);
    }
    pub fn get_condition(&self, state_from: RcState<A>, state_to: RcState<A>) -> Option<bool> {
        self.conditions
            .get(&(state_from.id(), state_to.id()))
            .map(|o| o(&*self.actor.upgrade().unwrap().borrow()))
    }

    pub fn process(&mut self) {
        let rc_actor = self.actor.upgrade().unwrap();
        let rc_state = self.current_state.upgrade().unwrap();
        
        let ac = rc_actor.borrow();
        rc_state.run(&*ac);
        drop(ac);

        let ac = rc_actor.borrow();

        let Some(((_, to), _)) = self.conditions.iter().find(|((from, _), c)| {
            *from == rc_state.id()
                && c(&*ac)
        }) else {
            return;
        };
        drop(ac);
        let Some(to_state) = self.states.iter().find(| s | s.id() == *to) else {
            return;
        };

        self.current_state = Rc::downgrade(to_state);
        
    }
}
