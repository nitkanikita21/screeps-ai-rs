use std::hash::Hash;
use ecow::EcoString;

pub trait State {
    type Actor;
    
    fn id(&self) -> EcoString;
    
    fn run(&self, actor: &Self::Actor);
}