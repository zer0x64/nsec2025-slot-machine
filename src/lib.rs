use std::sync::{Arc, LazyLock};

use clock::Clock;
use strum::EnumCount;
use tauri::{async_runtime::Mutex, ipc::Channel, Manager};

mod cli;
mod clock;
mod models;
mod rngs;
mod slot;
mod timer;

#[cfg(feature = "gpio")]
mod gpio;

#[cfg(feature = "gpio")]
use gpio::{Gpio, GpioEventMessage};

use rngs::SlotLevel;
use slot::SlotMachine;

use models::{SpinResponse, StartResponse};

#[derive(Debug)]
#[cfg(feature = "gpio")]
struct AppState {
    slot_machine: SlotMachine,
    clock: Option<Clock>,
    gpio: Gpio,
}

#[derive(Debug)]
#[cfg(not(feature = "gpio"))]
struct AppState {
    slot_machine: SlotMachine,
    clock: Option<Clock>,
}

type SharedState = Arc<Mutex<AppState>>;

// Fake name
// A set of key generated from a random sequence
static AJBCKQOISJS: LazyLock<Mutex<[[u8; 64]; 161]>> = LazyLock::new(|| {
    let mut buffer = [[0u8; 64]; 161];
    let mut udnmklwbk: [u8; 64] = [
        0x4e, 0xc3, 0x8a, 0xa8, 0x24, 0x5a, 0xc3, 0xad, 0x8e, 0x37, 0x1c, 0x95, 0x0b, 0x46, 0x17,
        0xe7, 0xaf, 0x8a, 0x66, 0x98, 0x82, 0x48, 0x82, 0xd7, 0x1e, 0x44, 0x46, 0xd7, 0xca, 0x08,
        0x4e, 0x88, 0xcd, 0x04, 0x45, 0x9e, 0x78, 0xd7, 0xa9, 0x82, 0xee, 0xc9, 0x0a, 0x9a, 0x4b,
        0x13, 0x8f, 0x0c, 0x51, 0x0e, 0x44, 0xfb, 0x5e, 0x23, 0x8b, 0xfc, 0xda, 0xc2, 0x17, 0x0d,
        0x7d, 0x83, 0x17, 0x5b,
    ];

    for i in 0..161usize {
        for j in 0..64usize {
            udnmklwbk[j] ^= udnmklwbk[udnmklwbk.len() - 1 - j].rotate_left((i as u32) % 8)
        }

        buffer[i].copy_from_slice(&udnmklwbk);
    }

    Mutex::new(buffer)
});

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    #[cfg(feature = "dist")]
    std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");

    tracing_subscriber::fmt::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    #[cfg(feature = "gpio")]
    let shared_state = SharedState::new(Mutex::new(AppState {
        slot_machine: Default::default(),
        clock: None,
        gpio: gpio::Gpio::new(),
    }));

    #[cfg(not(feature = "gpio"))]
    let shared_state = SharedState::new(Mutex::new(AppState {
        slot_machine: Default::default(),
        clock: None,
    }));

    tauri::Builder::default()
        .manage(shared_state)
        .invoke_handler(tauri::generate_handler![
            init_clock,
            gpio_subscribe,
            start_level,
            stop_level,
            get_credits,
            get_flag,
            get_secret_flag,
            get_num_levels,
            spin
        ])
        .setup(|app| {
            let main_window = app.get_webview_window("main").unwrap();
            main_window.set_title("WonderSlot")?;

            #[cfg(feature = "kiosk")]
            main_window
                .set_decorations(false)
                .expect("Couldn't disable window decoration!");

            #[cfg(feature = "kiosk")]
            main_window
                .set_fullscreen(true)
                .expect("Couldn't start in fullscreen!");

            #[cfg(feature = "kiosk")]
            main_window.set_cursor_visible(false);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// Fake name
// Anti-debugging function
#[cfg(feature = "dist")]
#[allow(non_snake_case)]
async fn j9htd1() {
    // We call ptrace to see if a debugger is running
    // If it's -1, we get a 0xFF mask
    let s0E4WalQ = unsafe { libc::ptrace(libc::PTRACE_TRACEME, 0) as u64 };

    let s0E4WalQ = (((s0E4WalQ >> 63) as u8) ^ 0xFF) & 0x01;
    let s0E4WalQ = s0E4WalQ
        | (s0E4WalQ << 1)
        | (s0E4WalQ << 2)
        | (s0E4WalQ << 3)
        | (s0E4WalQ << 4)
        | (s0E4WalQ << 5)
        | (s0E4WalQ << 6)
        | (s0E4WalQ << 7);

    for (i, b) in AJBCKQOISJS
        .lock()
        .await
        .as_mut_slice()
        .iter_mut()
        .enumerate()
    {
        for j in 0..64 {
            b[j] ^= !((i.wrapping_mul(j)) as u8) & s0E4WalQ;
        }
    }
}

#[cfg(not(feature = "dist"))]
async fn j9htd1() {}

#[tauri::command]
async fn init_clock(
    clock_events: Channel<String>,
    state: tauri::State<'_, SharedState>,
) -> Result<(), String> {
    state.lock().await.clock = Some(Clock::new(clock_events));
    Ok(())
}

#[cfg(feature = "gpio")]
#[tauri::command]
async fn gpio_subscribe(
    gpio_events: Channel<GpioEventMessage>,
    state: tauri::State<'_, SharedState>,
) -> Result<(), String> {
    j9htd1().await;

    tracing::debug!("New gpio subscriber");
    state.lock().await.gpio.subscribe(gpio_events);

    Ok(())
}

#[cfg(not(feature = "gpio"))]
#[tauri::command]
async fn gpio_subscribe() {
    j9htd1().await;
    tracing::debug!("GPIO feature is not enabled, not starting thread");
}

#[tauri::command]
async fn get_num_levels() -> usize {
    SlotLevel::COUNT
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
async fn start_level(
    level: usize,
    timer_events: Channel<String>,
    state: tauri::State<'_, SharedState>,
) -> Result<StartResponse, String> {
    tracing::info!("Start level: {}", level);

    let level = SlotLevel::from_repr(level);
    if let Some(level) = level {
        let mut slot_machine = SlotMachine::new(
            &level,
            timer_events,
            state
                .lock()
                .await
                .clock
                .as_ref()
                .expect("shouldn't start a level without a clock"),
        );

        let slot_machine_inner = slot_machine
            .get_mut()
            .expect("slot machine was just created, it shouldn't be empty");

        let metadata = slot_machine_inner.get_metadata();
        let debug_info = slot_machine_inner.get_debug_info();

        {
            state.lock().await.slot_machine = slot_machine;
        }

        let reel_layout = metadata
            .reel_layout
            .into_iter()
            .map(i32::try_from)
            .filter_map(Result::ok)
            .collect();

        Ok(StartResponse {
            starting_credits: metadata.starting_credits as u64,
            required_credits: metadata.required_credits as u64,
            denomination: metadata.denomination,
            n_wheels: metadata.n_wheels as u32,
            reel_layout,
            debug_info,
        })
    } else {
        Err("That level doesn't exists!".to_string())
    }
}

#[tauri::command]
async fn stop_level(state: tauri::State<'_, SharedState>) -> Result<(), String> {
    state.lock().await.slot_machine.stop();
    Ok(())
}

#[tauri::command]
async fn get_credits(state: tauri::State<'_, SharedState>) -> Result<usize, String> {
    let mut state = state.lock().await;

    let slot_machine = match state.slot_machine.get_mut() {
        Some(s) => s,
        None => return Err("No level currently running!\n".to_string()),
    };

    let credits = slot_machine.credits;
    tracing::info!("Get credits: {}", credits);
    Ok(credits)
}

#[tauri::command]
async fn get_flag(state: tauri::State<'_, SharedState>) -> Result<String, String> {
    let mut state = state.lock().await;

    match state.slot_machine.get_mut() {
        Some(s) => s.get_flag().map_or(
            Err("Not enough credits to buy the flag!\n".to_string()),
            |f| Ok(f),
        ),
        None => Err("The level has timed out!\n".to_string()),
    }
}

#[tauri::command]
async fn get_secret_flag() -> String {
    slot_machine_procmacro::obfuscate_flag!(
        "FLAG-e9199b05fd8996d0bbd37133566cf80c6c81fa557286e8cc043cd58d089db6a2",
        b"\x5b\x06\x16\x9e\x7d\xc7\xec\x01\x79\x32\xd0\xf4\x15\xdc\xc9\xf4\x00\x0c\xe7\xc0\xab\x94\x28\x6c\x70\x40\x46\x03\xba\xdd\x46\x87\x6f\x8e\xeb\x09\xfe\x3c\xd8\x7c\x0c\x33\x4e\x39\xb9\x84\x73\x20\x9c\x61\x2d\x67\x62\x1b\x36\x3e\x7b\x19\x91\xa6\x6b\x69\x8d\x6c"
    );
}

#[tauri::command]
async fn spin(bet: usize, state: tauri::State<'_, SharedState>) -> Result<SpinResponse, String> {
    let mut state = state.lock().await;

    let slot_machine = match state.slot_machine.get_mut() {
        Some(s) => s,
        None => return Err("No level currently running!\n".to_string()),
    };

    if slot_machine.credits < bet {
        tracing::info!("Unable to spin: out of credits");
        return Err("Could not spin, out of credits\n".to_string());
    }

    // In slot machines, the bet is taken from the credits, even on a winning spin
    slot_machine.credits -= bet;

    let stops = slot_machine.spin();
    let payout = slot_machine.calculate_payout(bet, &stops);

    if payout > 0 {
        // You won!
        slot_machine.credits += payout
    }

    let credits = slot_machine.credits;

    let stops = stops
        .into_iter()
        .map(i32::try_from)
        .filter_map(Result::ok)
        .collect();

    let response = SpinResponse {
        stops,
        payout: payout as u64,
        credits: credits as u64,
    };

    tracing::info!("Spin. New balance: {}, reels: {:?}", credits, response);
    Ok(response)
}
