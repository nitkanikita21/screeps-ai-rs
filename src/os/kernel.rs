use log::{error, info, warn};
use screeps::{game, TextAlign, TextStyle};
use screeps::game::rooms;
use web_sys::console::{info, warn};
use crate::os::process::{ProcessFlag, ProcessRunnable};
use crate::os::process_table::{Priority, ProcessTable};
use crate::os::scheduler::Scheduler;

#[derive(Default)]
pub struct Kernel {
    scheduler: Scheduler,
    process_table: ProcessTable
}

const MAX_RUN_RPOCESSES: usize = 4;

impl Kernel {
    pub fn new_process(&mut self, name: Option<String>, priority: Priority, flags: Vec<ProcessFlag>, runnable: Box<ProcessRunnable>) {
        self.process_table.new_process(name, priority, flags, runnable);
    }

    pub fn tick(&mut self) {

        let mut i = 0usize;
        while let Some(process) = self.scheduler.get_next_process(&mut self.process_table) {
            if i > MAX_RUN_RPOCESSES { warn!("🚫 Instructions limit reached"); break; }
            match process.run() {
                Ok(_) => {
                    info!("[P] ✅ {}", process.name());
                }
                Err(err) => {
                    error!("[P] ❌ {} {}", process.name(), err.to_string());
                }
            }
            i += 1;
        }

        self.scheduler.end_tick();
        self.process_table.end_tick();
        self.draw_infos();
    }
    
    pub fn kill_process(&mut self, id: u16) {
        self.process_table.kill(id);
    }
    
    fn draw_infos(&self) {
        for visual in rooms().values().map(|r| r.visual()) {
            visual.text(
                0_f32,
                1_f32,
                format!("Used: {:.3}", game::cpu::get_used()),
                Some(TextStyle::default().align(TextAlign::Left).color("#368a37")),
            );
            visual.text(
                0_f32,
                2_f32,
                format!("Limit + Bucket: {}", game::cpu::tick_limit()),
                Some(TextStyle::default().align(TextAlign::Left)),
            );
            visual.text(
                0_f32,
                3_f32,
                format!("Limit: {}", game::cpu::limit()),
                Some(TextStyle::default().align(TextAlign::Left)),
            );
            visual.text(
                0_f32,
                4_f32,
                format!("Bucket: {}", game::cpu::bucket()),
                Some(TextStyle::default().align(TextAlign::Left)),
            );
            visual.text(
                0_f32,
                5_f32,
                format!("Tick limit: {}", game::cpu::tick_limit()),
                Some(TextStyle::default().align(TextAlign::Left)),
            );
        }
    }
}