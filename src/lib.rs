use std::fmt::format;
use std::thread::{sleep, spawn};
use std::time::Duration;
use std::{
    cell::RefCell,
    collections::{hash_map::Entry, HashMap, HashSet},
};

use crate::os::kernel::Kernel;
use crate::os::process::ProcessFlag;
use crate::os::process_table::Priority;
use js_sys::{JsString, Object, Reflect};
use log::*;
use screeps::game::rooms;
use screeps::{
    constants::{ErrorCode, Part, ResourceType},
    enums::StructureObject,
    find, game,
    local::ObjectId,
    objects::{Creep, Source, StructureController},
    prelude::*,
    TextAlign, TextStyle,
};
use wasm_bindgen::prelude::*;

mod ai;
mod logging;
mod os;
// this is one way to persist data between ticks within Rust's memory, as opposed to
// keeping state in memory on game objects - but will be lost on global resets!
thread_local! {
    // static CREEP_TARGETS: RefCell<HashMap<String, CreepTargetOld>> = RefCell::new(HashMap::new());
    static KERNEL: RefCell<Kernel> = RefCell::new(Kernel::default())
}

static INIT_LOGGING: std::sync::Once = std::sync::Once::new();
static INIT_PROCESS: std::sync::Once = std::sync::Once::new();

// this enum will represent a creep's lock on a specific target object, storing a js reference
// to the object id so that we can grab a fresh reference to the object each successive tick,
// since screeps game objects become 'stale' and shouldn't be used beyond the tick they were fetched

// add wasm_bindgen to any function you would like to expose for call from js
// to use a reserved name as a function name, use `js_name`:
#[wasm_bindgen(js_name = loop)]
pub fn game_loop() {
    INIT_LOGGING.call_once(|| {
        // show all output of Info level, adjust as needed
        logging::setup_logging(logging::Trace);
    });

    debug!("loop starting! CPU: {}", game::cpu::get_used());

    /*// mutably borrow the creep_targets refcell, which is holding our creep target locks
    // in the wasm heap
    CREEP_TARGETS.with(|creep_targets_refcell| {
        let mut creep_targets = creep_targets_refcell.borrow_mut();
        debug!("running creeps");
        for creep in game::creeps().values() {
            run_creep(&creep, &mut creep_targets);
        }
    });*/

    KERNEL.with_borrow_mut(|kernel| {
        INIT_PROCESS.call_once(|| {
            kernel.new_process(
                Some("ai".to_string()),
                Priority::new(4),
                Vec::new(),
                Box::new(|p| {
                    ai::test_ai();

                    Ok(())
                }),
            );

            kernel.new_process(
                Some("pixel".to_string()),
                Priority::new(1),
                vec![ProcessFlag::GeneratePixel],
                Box::new(|_| {
                    game::cpu::generate_pixel()
                        .map_err(|e| anyhow::Error::msg(format!("{:?}", e)))?;

                    Ok(())
                }),
            );
        });

        kernel.tick()
    });

    info!("done! cpu: {}", game::cpu::get_used());
    
}
