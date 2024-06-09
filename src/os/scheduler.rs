use crate::os::process::{Process, RunStrategy};
use crate::os::process_table::ProcessTable;
use screeps::game;
use std::collections::HashMap;
use std::ops::Deref;

#[derive(Default)]
pub struct Scheduler {
    tick: u32,
    used_processes: Vec<u16>,
}

impl Scheduler {
    pub fn get_next_process<'a>(&mut self, process_table: &'a ProcessTable) -> Option<&'a Process> {
        let mut processes = process_table.get_processes();
        processes.sort_by_key(|p| p.run_strategy().priority());

        let finded = processes.into_iter().find(|x| match x.run_strategy() {
            RunStrategy::Always if !self.is_used(x.id()) => true,
            RunStrategy::EveryNTicks(t)
            if !self.is_used(x.id()) && self.tick % t as u32 == 0 =>
                {
                    true
                }
            RunStrategy::EveryNSecond(s)
            if !self.is_used(x.id()) && game::time() % s as u32 == 0 =>
                {
                    true
                }
            RunStrategy::HasCpu(cpu)
            if !self.is_used(x.id()) && self.get_free_cpu() >= cpu =>
                {
                    true
                }
            RunStrategy::HasBucketCpu(cpu)
            if !self.is_used(x.id())
                && game::cpu::bucket() as f64 + self.get_free_cpu() >= cpu =>
                {
                    true
                }
            _ => false,
        });
        
        finded.map(| p | {
            self.mark_used(p.id())
        });
        
        finded
    }

    pub fn next_tick(&mut self) {
        self.tick += 1;
    }
    pub fn end_tick(&mut self) {
        self.used_processes.clear();
    }

    fn is_used(&self, id: u16) -> bool {
        self.used_processes.contains(&id)
    }
    fn mark_used(&mut self, id: u16) {
        self.used_processes.push(id)
    }

    fn get_free_cpu(&self) -> f64 {
        game::cpu::limit() as f64 - game::cpu::get_used()
    }
}
