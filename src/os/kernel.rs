use log::{error, info};
use crate::os::process::{ProcessRunnable, RunStrategy};
use crate::os::process_table::ProcessTable;
use crate::os::scheduler::Scheduler;

#[derive(Default)]
pub struct Kernel {
    scheduler: Scheduler,
    process_table: ProcessTable
}

impl Kernel {
    pub fn new_process(&mut self, name: Option<String>, run_strategy: RunStrategy, process_runnable: Box<ProcessRunnable>) {
        self.process_table.new_process(name, run_strategy, process_runnable);
    }
    
    pub fn tick(&mut self) {
        self.scheduler.next_tick();

        while let Some(process) = self.scheduler.get_next_process(&self.process_table) {
            match process.run() {
                Ok(_) => {
                    info!("[P] ✅ {}", process.name());
                }
                Err(err) => {
                    error!("[P] ❌ {} {}", process.name(), err);
                }
            }
        }
        
        self.scheduler.end_tick()
    }
}