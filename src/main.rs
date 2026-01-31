//! German-style 4-way traffic light controller on Raspberry Pi Pico using Embassy + Rust
//!
//! Implements authentic German traffic light sequence with Red+Yellow preparation phase:
//! Red → Red+Yellow (1s) → Green (10s) → Yellow (3s) → Red → All Red safety (1s)
#![no_std]
#![no_main]
#![feature(impl_trait_in_assoc_type)]

use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // Initialize the Pico peripherals
    let p = embassy_rp::init(Default::default());

    // Define LEDs for each direction (Red, Yellow, Green)
    // Adjust GPIO numbers to match your wiring!
    let mut north_red    = Output::new(p.PIN_0, Level::Low);
    let mut north_yellow = Output::new(p.PIN_1, Level::Low);
    let mut north_green  = Output::new(p.PIN_2, Level::Low);

    let mut east_red     = Output::new(p.PIN_3, Level::Low);
    let mut east_yellow  = Output::new(p.PIN_4, Level::Low);
    let mut east_green   = Output::new(p.PIN_5, Level::Low);

    let mut south_red    = Output::new(p.PIN_6, Level::Low);
    let mut south_yellow = Output::new(p.PIN_7, Level::Low);
    let mut south_green  = Output::new(p.PIN_8, Level::Low);

    let mut west_red     = Output::new(p.PIN_9, Level::Low);
    let mut west_yellow  = Output::new(p.PIN_10, Level::Low);
    let mut west_green   = Output::new(p.PIN_11, Level::Low);

    // Initial state: All red
    all_red(
        &mut north_red, &mut north_yellow, &mut north_green,
        &mut south_red, &mut south_yellow, &mut south_green,
        &mut east_red, &mut east_yellow, &mut east_green,
        &mut west_red, &mut west_yellow, &mut west_green,
    ).await;

    // Main loop: German traffic light cycle
    loop {
        // === NORTH-SOUTH PHASE ===

        // Red+Yellow (German "get ready" phase, 1 sec)
        set_ns_red_yellow(
            &mut north_red, &mut north_yellow, &mut north_green,
            &mut south_red, &mut south_yellow, &mut south_green,
            &mut east_red, &mut east_yellow, &mut east_green,
            &mut west_red, &mut west_yellow, &mut west_green,
        ).await;
        Timer::after_secs(1).await;

        // North-South green (10 sec)
        set_ns_green(
            &mut north_red, &mut north_yellow, &mut north_green,
            &mut south_red, &mut south_yellow, &mut south_green,
            &mut east_red, &mut east_yellow, &mut east_green,
            &mut west_red, &mut west_yellow, &mut west_green,
        ).await;
        Timer::after_secs(10).await;

        // North-South yellow only (3 sec)
        set_ns_yellow(
            &mut north_red, &mut north_yellow, &mut north_green,
            &mut south_red, &mut south_yellow, &mut south_green,
            &mut east_red, &mut east_yellow, &mut east_green,
            &mut west_red, &mut west_yellow, &mut west_green,
        ).await;
        Timer::after_secs(3).await;

        // All red (safety clearance, 1 sec)
        all_red(
            &mut north_red, &mut north_yellow, &mut north_green,
            &mut south_red, &mut south_yellow, &mut south_green,
            &mut east_red, &mut east_yellow, &mut east_green,
            &mut west_red, &mut west_yellow, &mut west_green,
        ).await;
        Timer::after_secs(1).await;

        // === EAST-WEST PHASE ===

        // Red+Yellow (German "get ready" phase, 1 sec)
        set_ew_red_yellow(
            &mut east_red, &mut east_yellow, &mut east_green,
            &mut west_red, &mut west_yellow, &mut west_green,
            &mut north_red, &mut north_yellow, &mut north_green,
            &mut south_red, &mut south_yellow, &mut south_green,
        ).await;
        Timer::after_secs(1).await;

        // East-West green (10 sec)
        set_ew_green(
            &mut east_red, &mut east_yellow, &mut east_green,
            &mut west_red, &mut west_yellow, &mut west_green,
            &mut north_red, &mut north_yellow, &mut north_green,
            &mut south_red, &mut south_yellow, &mut south_green,
        ).await;
        Timer::after_secs(10).await;

        // East-West yellow only (3 sec)
        set_ew_yellow(
            &mut east_red, &mut east_yellow, &mut east_green,
            &mut west_red, &mut west_yellow, &mut west_green,
            &mut north_red, &mut north_yellow, &mut north_green,
            &mut south_red, &mut south_yellow, &mut south_green,
        ).await;
        Timer::after_secs(3).await;

        // All red (safety clearance, 1 sec)
        all_red(
            &mut north_red, &mut north_yellow, &mut north_green,
            &mut south_red, &mut south_yellow, &mut south_green,
            &mut east_red, &mut east_yellow, &mut east_green,
            &mut west_red, &mut west_yellow, &mut west_green,
        ).await;
        Timer::after_secs(1).await;
    }
}

// Helper: North-South green, East-West red
async fn set_ns_green(
    nr: &mut Output<'_>, ny: &mut Output<'_>, ng: &mut Output<'_>,
    sr: &mut Output<'_>, sy: &mut Output<'_>, sg: &mut Output<'_>,
    er: &mut Output<'_>, ey: &mut Output<'_>, eg: &mut Output<'_>,
    wr: &mut Output<'_>, wy: &mut Output<'_>, wg: &mut Output<'_>,
) {
    nr.set_low();   ny.set_low();   ng.set_high();
    sr.set_low();   sy.set_low();   sg.set_high();
    er.set_high();  ey.set_low();   eg.set_low();
    wr.set_high();  wy.set_low();   wg.set_low();
}

// Helper: North-South red+yellow (German preparation phase), East-West red
async fn set_ns_red_yellow(
    nr: &mut Output<'_>, ny: &mut Output<'_>, ng: &mut Output<'_>,
    sr: &mut Output<'_>, sy: &mut Output<'_>, sg: &mut Output<'_>,
    er: &mut Output<'_>, ey: &mut Output<'_>, eg: &mut Output<'_>,
    wr: &mut Output<'_>, wy: &mut Output<'_>, wg: &mut Output<'_>,
) {
    nr.set_high();  ny.set_high();  ng.set_low();
    sr.set_high();  sy.set_high();  sg.set_low();
    er.set_high();  ey.set_low();   eg.set_low();
    wr.set_high();  wy.set_low();   wg.set_low();
}

// Helper: North-South yellow only, East-West red
async fn set_ns_yellow(
    nr: &mut Output<'_>, ny: &mut Output<'_>, ng: &mut Output<'_>,
    sr: &mut Output<'_>, sy: &mut Output<'_>, sg: &mut Output<'_>,
    er: &mut Output<'_>, ey: &mut Output<'_>, eg: &mut Output<'_>,
    wr: &mut Output<'_>, wy: &mut Output<'_>, wg: &mut Output<'_>,
) {
    nr.set_low();   ny.set_high();  ng.set_low();
    sr.set_low();   sy.set_high();  sg.set_low();
    er.set_high();  ey.set_low();   eg.set_low();
    wr.set_high();  wy.set_low();   wg.set_low();
}

// Helper: East-West green, North-South red
async fn set_ew_green(
    er: &mut Output<'_>, ey: &mut Output<'_>, eg: &mut Output<'_>,
    wr: &mut Output<'_>, wy: &mut Output<'_>, wg: &mut Output<'_>,
    nr: &mut Output<'_>, ny: &mut Output<'_>, ng: &mut Output<'_>,
    sr: &mut Output<'_>, sy: &mut Output<'_>, sg: &mut Output<'_>,
) {
    er.set_low();   ey.set_low();   eg.set_high();
    wr.set_low();   wy.set_low();   wg.set_high();
    nr.set_high();  ny.set_low();   ng.set_low();
    sr.set_high();  sy.set_low();   sg.set_low();
}

// Helper: East-West red+yellow (German preparation phase), North-South red
async fn set_ew_red_yellow(
    er: &mut Output<'_>, ey: &mut Output<'_>, eg: &mut Output<'_>,
    wr: &mut Output<'_>, wy: &mut Output<'_>, wg: &mut Output<'_>,
    nr: &mut Output<'_>, ny: &mut Output<'_>, ng: &mut Output<'_>,
    sr: &mut Output<'_>, sy: &mut Output<'_>, sg: &mut Output<'_>,
) {
    er.set_high();  ey.set_high();  eg.set_low();
    wr.set_high();  wy.set_high();  wg.set_low();
    nr.set_high();  ny.set_low();   ng.set_low();
    sr.set_high();  sy.set_low();   sg.set_low();
}

// Helper: East-West yellow only, North-South red
async fn set_ew_yellow(
    er: &mut Output<'_>, ey: &mut Output<'_>, eg: &mut Output<'_>,
    wr: &mut Output<'_>, wy: &mut Output<'_>, wg: &mut Output<'_>,
    nr: &mut Output<'_>, ny: &mut Output<'_>, ng: &mut Output<'_>,
    sr: &mut Output<'_>, sy: &mut Output<'_>, sg: &mut Output<'_>,
) {
    er.set_low();   ey.set_high();  eg.set_low();
    wr.set_low();   wy.set_high();  wg.set_low();
    nr.set_high();  ny.set_low();   ng.set_low();
    sr.set_high();  sy.set_low();   sg.set_low();
}

// Helper: All directions red (safety phase)
async fn all_red(
    nr: &mut Output<'_>, ny: &mut Output<'_>, ng: &mut Output<'_>,
    sr: &mut Output<'_>, sy: &mut Output<'_>, sg: &mut Output<'_>,
    er: &mut Output<'_>, ey: &mut Output<'_>, eg: &mut Output<'_>,
    wr: &mut Output<'_>, wy: &mut Output<'_>, wg: &mut Output<'_>,
) {
    nr.set_high(); ny.set_low(); ng.set_low();
    sr.set_high(); sy.set_low(); sg.set_low();
    er.set_high(); ey.set_low(); eg.set_low();
    wr.set_high(); wy.set_low(); wg.set_low();
}
