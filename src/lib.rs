use anyhow::{Ok, Result};

use embedded_hal::adc::OneShot;
use esp_idf_hal::adc;
use esp_idf_hal::prelude::Peripherals;
use std::sync::{Condvar, Mutex};
use std::thread;
use std::{sync::Arc, time::*};

mod eventloop;
use crate::eventloop::test_eventloop;

mod timer;
use timer::init_timer;

pub fn run() -> Result<()> {
    println!("Hello, world!");

    for s in 0..3 {
        log::info!("Start program in {} secs", 3 - s);
        thread::sleep(Duration::from_secs(1));
    }

    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;
    let mut a2 = pins.gpio34.into_analog_atten_11db()?;
    let mut powered_adc1 = adc::PoweredAdc::new(
        peripherals.adc1,
        adc::config::Config::new().calibration(true),
    )?;

    let (eventloop, _subscription) = test_eventloop()?;
    let _timer = init_timer(eventloop)?;

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

            println!(
                "Hall sensor reading: {}mV",
                powered_adc1.read(&mut hall_sensor).unwrap()
            );
            println!(
                "A2 sensor reading: {}mV",
                powered_adc1.read(&mut a2).unwrap()
            );

            // TODO: ここでサーバに接続して、電力センサーからの値を送信する。
        }
    };
    Ok(())
}
