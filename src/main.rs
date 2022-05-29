use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

use anyhow::{Ok, Result};
use embedded_hal::adc::OneShot;
use esp_idf_hal::adc;
use esp_idf_hal::prelude::Peripherals;
use log;
use std::sync::{Condvar, Mutex};
use std::{sync::Arc, time::*};

fn main() -> Result<()> {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();

    println!("Hello, world!");

    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;
    let mut a2 = pins.gpio34.into_analog_atten_11db()?;
    let mut powered_adc1 = adc::PoweredAdc::new(
        peripherals.adc1,
        adc::config::Config::new().calibration(true),
    )?;

    let mutex = Arc::new((Mutex::new(None), Condvar::new()));

    let mut wait = mutex.0.lock().unwrap();
    let mut hall_sensor = peripherals.hall_sensor;

    #[allow(unused)]
    let cycles: u32 = loop {
        if let Some(cycles) = *wait {
            break cycles;
        } else {
            wait = mutex
                .1
                .wait_timeout(wait, Duration::from_secs(1))
                .unwrap()
                .0;

            log::info!(
                "Hall sensor reading: {}mV",
                powered_adc1.read(&mut hall_sensor).unwrap()
            );
            log::info!(
                "A2 sensor reading: {}mV",
                powered_adc1.read(&mut a2).unwrap()
            );
        }
    };
    Ok(())
}
