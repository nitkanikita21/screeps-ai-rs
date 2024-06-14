use getset::{CopyGetters, Getters};

pub type ProcessRunnable = dyn Fn(&Process) -> anyhow::Result<()>;

#[derive(CopyGetters, Getters)]
pub struct Process {
    #[getset(get_copy = "pub")]
    id: u16,

    #[getset(get = "pub")]
    name: String,
    runnable: Box<ProcessRunnable>,

    #[getset(get_copy = "pub")]
    wait_time: u16,

    #[getset(get = "pub")]
    flags: Vec<ProcessFlag>
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum ProcessFlag {
    CpuBucket,
    GeneratePixel
}


impl Process {
    pub fn new(
        id: u16,
        name: Option<String>,
        wait_time: u16,
        flags: Vec<ProcessFlag>,
        runnable: Box<ProcessRunnable>,
    ) -> Self {
        Self {
            id,
            runnable,
            wait_time,
            flags,
            name: name.unwrap_or_else(|| id.to_string()),
        }
    }
    pub fn run(&mut self) -> anyhow::Result<()> {
        self.wait_time = 0;
        (self.runnable)(self)
    }
    
    pub fn increment_wait_time(&mut self) {
        self.wait_time += 1;
    }
    
    pub fn has_flag(&self, flag: ProcessFlag) -> bool {
        self.flags.contains(&flag)
    }
}

