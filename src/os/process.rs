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
    run_strategy: RunStrategy,
}

impl Process {
    pub fn new(
        id: u16,
        name: Option<String>,
        run_strategy: RunStrategy,
        runnable: Box<ProcessRunnable>,
    ) -> Self {
        Self {
            id,
            runnable,
            run_strategy,
            name: name.unwrap_or_else(|| id.to_string()),
        }
    }
    pub fn run(&self) -> anyhow::Result<()> {
        (self.runnable)(self)
    }
}

#[derive(PartialOrd, PartialEq, Copy, Clone)]
pub enum RunStrategy {
    Always,
    EveryNTicks(usize),
    EveryNSecond(usize),
    HasCpu(f64),
    HasBucketCpu(f64),
}

impl RunStrategy {
    pub fn priority(&self) -> usize {
        match self {
            RunStrategy::Always => 1,
            RunStrategy::EveryNTicks(_) => 2,
            RunStrategy::EveryNSecond(_) => 3,
            RunStrategy::HasCpu(_) => 4,
            RunStrategy::HasBucketCpu(_) => 5,
        }
    }
}
