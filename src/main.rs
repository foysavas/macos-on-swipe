mod accessibility;
mod gesture;

use log::{debug, error, info, warn};
use std::ffi::c_void;
use std::path::PathBuf;
use std::process::Command;
use std::thread;

use gesture::SwipeDirection;

// MultitouchSupport types
type MTDeviceRef = *mut c_void;

#[repr(C)]
#[derive(Clone, Copy)]
struct MTPoint {
    x: f32,
    y: f32,
}

#[repr(C)]
struct MTTouch {
    frame: i32,
    timestamp: f64,
    identifier: i32,
    state: i32,
    finger_id: i32,
    hand_id: i32,
    normalized: MTPoint,
    total_pressure: f32,
    _pad1: i32,
    angle: f32,
    major_axis: f32,
    minor_axis: f32,
    absolute: MTPoint,
    _pad2: i32,
    _pad3: i32,
    vel_x: f32,
    vel_y: f32,
    _pad4: i32,
    density: f32,
}

type MTFrameCallbackFunction = extern "C" fn(MTDeviceRef, *mut MTTouch, i32, f64, i32) -> i32;

#[link(name = "MultitouchSupport", kind = "framework")]
extern "C" {
    fn MTDeviceCreateList() -> *mut c_void;
    fn MTRegisterContactFrameCallback(device: MTDeviceRef, callback: MTFrameCallbackFunction);
    fn MTDeviceStart(device: MTDeviceRef, mode: i32);
}

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    fn CFArrayGetCount(array: *mut c_void) -> i64;
    fn CFArrayGetValueAtIndex(array: *mut c_void, idx: i64) -> *mut c_void;
    fn CFRunLoopRunInMode(mode: *const c_void, seconds: f64, returnAfterSourceHandled: bool) -> i32;
}

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    static kCFRunLoopDefaultMode: *const c_void;
}

const MIN_FINGERS: i32 = 3;
const SWIPE_THRESHOLD: f32 = 0.05;

static mut GESTURE_STATE: Option<GestureState> = None;
static mut SCRIPT_PATH: Option<PathBuf> = None;

struct GestureState {
    prev_positions: Vec<(i32, f32, f32)>,
    accumulated_dx: f32,
    accumulated_dy: f32,
    in_gesture: bool,
    max_fingers: i32,
}

impl GestureState {
    fn new() -> Self {
        Self {
            prev_positions: Vec::new(),
            accumulated_dx: 0.0,
            accumulated_dy: 0.0,
            in_gesture: false,
            max_fingers: 0,
        }
    }

    fn reset(&mut self) {
        self.prev_positions.clear();
        self.accumulated_dx = 0.0;
        self.accumulated_dy = 0.0;
        self.in_gesture = false;
        self.max_fingers = 0;
    }
}

fn get_script_path() -> Option<PathBuf> {
    let home = dirs::home_dir()?;
    let path = home.join(".config/macos-on-swipe/handle-swipe.sh");
    if path.exists() {
        Some(path)
    } else {
        None
    }
}

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    info!("macos-on-swipe starting...");

    // Check for script
    let script_path = get_script_path();
    match &script_path {
        Some(path) => info!("Script found: {}", path.display()),
        None => info!("No script at ~/.config/macos-on-swipe/handle-swipe.sh - swipes will only be logged"),
    }

    if !accessibility::is_trusted() {
        warn!("Accessibility permission not granted");
        accessibility::print_permission_instructions();
        info!("Prompting for accessibility permission...");
        accessibility::check_accessibility(true);

        if !accessibility::is_trusted() {
            error!("Cannot proceed without accessibility permissions");
            std::process::exit(1);
        }
    }

    info!("Accessibility permission granted");

    unsafe {
        GESTURE_STATE = Some(GestureState::new());
        SCRIPT_PATH = script_path;
    }

    // Initialize and start multitouch devices
    unsafe {
        let devices = MTDeviceCreateList();
        if devices.is_null() {
            error!("Failed to get multitouch device list. MultitouchSupport may not be available.");
            std::process::exit(1);
        }

        let count = CFArrayGetCount(devices);
        info!("Found {} multitouch device(s)", count);

        if count == 0 {
            error!("No multitouch devices found");
            std::process::exit(1);
        }

        for i in 0..count {
            let device = CFArrayGetValueAtIndex(devices, i) as MTDeviceRef;
            MTRegisterContactFrameCallback(device, touch_callback);
            MTDeviceStart(device, 0);
            info!("Started multitouch device {}", i);
        }
    }

    info!("Listening for {}-finger swipe gestures...", MIN_FINGERS);
    info!("Press Ctrl+C to exit");

    // Run the main loop
    unsafe {
        loop {
            let result = CFRunLoopRunInMode(kCFRunLoopDefaultMode, 0.1, false);
            if result == 1 {
                break;
            }
        }
    }
}

extern "C" fn touch_callback(
    _device: MTDeviceRef,
    data: *mut MTTouch,
    num_fingers: i32,
    _timestamp: f64,
    _frame: i32,
) -> i32 {
    unsafe {
        handle_touches(data, num_fingers);
    }
    0
}

unsafe fn handle_touches(data: *mut MTTouch, num_fingers: i32) {
    let state = match GESTURE_STATE.as_mut() {
        Some(s) => s,
        None => return,
    };

    let mut current: Vec<(i32, f32, f32)> = Vec::new();
    for i in 0..num_fingers {
        let touch = &*data.add(i as usize);
        current.push((touch.finger_id, touch.normalized.x, touch.normalized.y));
    }

    let touch_count = current.len() as i32;

    if touch_count >= MIN_FINGERS && !state.in_gesture {
        state.in_gesture = true;
        state.max_fingers = touch_count;
        state.accumulated_dx = 0.0;
        state.accumulated_dy = 0.0;
        state.prev_positions = current.clone();
        debug!("Gesture started: {} fingers", touch_count);
    }

    if state.in_gesture && touch_count > 0 {
        if touch_count > state.max_fingers {
            state.max_fingers = touch_count;
        }

        for (fid, cx, cy) in &current {
            for (pid, px, py) in &state.prev_positions {
                if fid == pid {
                    state.accumulated_dx += cx - px;
                    state.accumulated_dy += cy - py;
                    break;
                }
            }
        }

        state.prev_positions = current;
    }

    if state.in_gesture && touch_count == 0 {
        debug!("Gesture ended: max {} fingers, delta: ({:.3}, {:.3})", 
               state.max_fingers, state.accumulated_dx, state.accumulated_dy);

        if state.max_fingers >= MIN_FINGERS {
            let avg_dx = state.accumulated_dx / state.max_fingers as f32;
            let avg_dy = state.accumulated_dy / state.max_fingers as f32;

            if avg_dx.abs() >= SWIPE_THRESHOLD || avg_dy.abs() >= SWIPE_THRESHOLD {
                if let Some(dir) = SwipeDirection::from_deltas(avg_dx as f64, -avg_dy as f64) {
                    println!("swipe {}", dir);
                    
                    if let Some(script) = SCRIPT_PATH.as_ref() {
                        execute_script(script, dir);
                    }
                }
            }
        }

        state.reset();
    }
}

fn execute_script(script_path: &PathBuf, direction: SwipeDirection) {
    let direction_arg = direction.as_arg();
    debug!("Executing: {} {}", script_path.display(), direction_arg);

    match Command::new(script_path).arg(direction_arg).spawn() {
        Ok(mut child) => {
            thread::spawn(move || {
                let _ = child.wait();
            });
        }
        Err(e) => {
            error!("Failed to execute script '{}': {}", script_path.display(), e);
        }
    }
}
