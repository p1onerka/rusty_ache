use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};

struct ScriptState {
    last_cycle_start: Instant,
    jumping: bool,
}

static STATE: OnceLock<Mutex<ScriptState>> = OnceLock::new();

pub fn main_obj_script() -> (i32, i32) {
    // this is just an example behavior for main object
    // if you run 'cargo run --bin main' you'll see the ship "falling" every 3 seconds
    // so in this funciton you can write custom main object script

    const COOLDOWN: Duration = Duration::from_secs(3);
    const JUMP_DURATION: Duration = Duration::from_millis(200);

    let state = STATE.get_or_init(|| {
        Mutex::new(ScriptState {
            last_cycle_start: Instant::now(),
            jumping: false,
        })
    });

    let mut st = state.lock().unwrap();
    let elapsed = st.last_cycle_start.elapsed();

    if st.jumping {
        // currently in jumping phase
        if elapsed >= JUMP_DURATION {
            // jump finished
            st.jumping = false;
            st.last_cycle_start = Instant::now(); // restart cycle after jump ends
            return (0, 0);
        } else {
            // still jumping
            return (0, -4); // height of jump
        }
    } else {
        // waiting for the 3-second cooldown
        if elapsed >= COOLDOWN {
            // start a new jump
            st.jumping = true;
            st.last_cycle_start = Instant::now(); // mark start of jump
            return (0, -4);
        } else {
            // still cooling down
            return (0, 0);
        }
    }
}
