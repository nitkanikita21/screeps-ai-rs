use crate::os::process::{Process, ProcessFlag};
use crate::os::process_table::ProcessTable;
use log::{debug, info, warn};
use screeps::game;
use std::collections::HashMap;
use std::ops::Deref;

#[derive(Default)]
pub struct Scheduler {
    used_processes: Vec<u16>,
}

impl Scheduler {
    pub fn get_next_process<'a>(
        &mut self,
        process_table: &'a mut ProcessTable,
    ) -> Option<&'a mut Process> {
        let mut processes = process_table.get_processes();
        for x in &mut processes {
            x.process_mut().increment_wait_time()
        }
        let result = processes
            .into_iter()
            .max_by(|a, b| a.process().wait_time().cmp(&b.process().wait_time()))
            .unwrap();

        let is_limit_reached = if result.process().has_flag(ProcessFlag::CpuBucket) {
            game::cpu::get_used() < game::cpu::tick_limit()
        } else {
            game::cpu::get_used() < game::cpu::limit() as f64
        };
        
        if is_limit_reached
        {
            // info!("{} default", result.process().name());
            return Some(result.process_mut());
        };

        warn!("{} none", result.process().name());
        None
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
