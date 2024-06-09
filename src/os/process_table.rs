use crate::os::process::{Process, RunStrategy, ProcessRunnable};
use std::collections::HashMap;

#[derive(Default)]
pub struct ProcessTable {
    table: HashMap<u16, Process>,
}

impl ProcessTable {
    pub fn new_process(
        &mut self,
        name: Option<String>,
        run_strategy: RunStrategy,
        runnable: Box<ProcessRunnable>,
    ) {
        let process = Process::new(
            fastrand::u16(u16::MIN..u16::MAX),
            name,
            run_strategy,
            runnable,
        );
        self.table.insert(process.id(), process);
    }
    pub fn get_processes(&self) -> Vec<&Process> {
        self.table.values().collect::<Vec<&Process>>()
    }
}
