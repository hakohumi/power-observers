use embedded_svc::sys_time::SystemTime;
use embedded_svc::timer::{OnceTimer, PeriodicTimer, TimerService};

use esp_idf_svc::eventloop::EspBackgroundEventLoop;
use esp_idf_svc::systime::EspSystemTime;
use esp_idf_svc::timer::{EspTimer, EspTimerService};

use anyhow::{Ok, Result};
use embedded_hal::adc::OneShot;
use esp_idf_hal::adc;
use esp_idf_hal::prelude::Peripherals;
use std::sync::{Condvar, Mutex};
use std::thread;
use std::{sync::Arc, time::*};

mod eventloop;
use eventloop::test_eventloop;
use eventloop::EventLoopMessage;

pub fn run() -> Result<()> {
    println!("Hello, world!");

    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;
    let mut a2 = pins.gpio34.into_analog_atten_11db()?;
    let mut powered_adc1 = adc::PoweredAdc::new(
        peripherals.adc1,
        adc::config::Config::new().calibration(true),
    )?;

    let (eventloop, _subscription) = test_eventloop()?;
    let _timer = test_timer(eventloop)?;

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

fn test_timer(mut eventloop: EspBackgroundEventLoop) -> Result<EspTimer> {
    use embedded_svc::event_bus::Postbox;

    println!("About to schedule a one-shot timer for after 2 seconds");
    let mut once_timer = EspTimerService::new()?.timer(|| {
        println!("One-shot timer triggered");
    })?;

    once_timer.after(Duration::from_secs(2))?;

    thread::sleep(Duration::from_secs(3));

    println!("About to schedule a periodic timer every five seconds");
    let mut periodic_timer = EspTimerService::new()?.timer(move || {
        println!("Tick from periodic timer");

        let now = EspSystemTime {}.now();

        eventloop.post(&EventLoopMessage::new(now), None).unwrap();

        println!("eventloop test");
    })?;

    periodic_timer.every(Duration::from_secs(5))?;

    Ok(periodic_timer)
}
