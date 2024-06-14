use crate::os::process::{ProcessFlag, Process, ProcessRunnable};
use std::collections::HashMap;
use getset::{Getters, MutGetters};

#[derive(PartialOrd, PartialEq, Copy, Clone)]
pub struct Priority(u16);

impl Priority {
    pub const fn new(p: u16) -> Self {
        Self(p)
    }
}

#[derive(MutGetters, Getters)]
pub struct ProcessInfo {
    #[getset(get_mut = "pub", get = "pub")]
    process: Process,
    #[getset(get = "pub")]
    priority: Priority
}

#[derive(Default)]
pub struct ProcessTable {
    table: HashMap<u16, ProcessInfo>,
    to_kill: Vec<u16>
}


impl ProcessTable {
    pub fn new_process(
        &mut self,
        name: Option<String>,
        priority: Priority,
        flags: Vec<ProcessFlag>,
        runnable: Box<ProcessRunnable>,
    ) {
        let process = Process::new(
            fastrand::u16(u16::MIN..u16::MAX),
            name,
            0,
            flags,
            runnable,
        );
        self.table.insert(process.id(), ProcessInfo {
            process, priority
        });
    }
    pub fn get_processes(&mut self) -> Vec<&mut ProcessInfo> {
        self.table.values_mut().collect::<Vec<&mut ProcessInfo>>()
    }
    pub fn kill(&mut self, id: u16) {
        self.to_kill.push(id);
    }
    
    pub fn end_tick(&mut self) {
        for x in &self.to_kill {
            self.table.remove(x);
        }
        self.to_kill.clear();
    }
}
