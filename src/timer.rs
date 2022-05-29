use embedded_svc::sys_time::SystemTime;
use embedded_svc::timer::{PeriodicTimer, TimerService};

use anyhow::{Ok, Result};
use esp_idf_svc::eventloop::EspBackgroundEventLoop;
use esp_idf_svc::systime::EspSystemTime;
use esp_idf_svc::timer::{EspTimer, EspTimerService};
use std::time::Duration;

use embedded_hal::adc::OneShot;
use esp_idf_hal::adc;
use esp_idf_hal::prelude::Peripherals;

use crate::eventloop::EventLoopMessage;

pub fn init_timer(mut eventloop: EspBackgroundEventLoop) -> Result<EspTimer> {
    use embedded_svc::event_bus::Postbox;

    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;

    let mut a2 = pins.gpio34.into_analog_atten_11db()?;
    let mut powered_adc1 = adc::PoweredAdc::new(
        peripherals.adc1,
        adc::config::Config::new().calibration(true),
    )?;

    let mut periodic_timer = EspTimerService::new()?.timer(move || {
        println!("Tick from periodic timer");

        let now = EspSystemTime {}.now();

        eventloop.post(&EventLoopMessage::new(now), None).unwrap();

        println!("eventloop test {:?}", now.to_owned());

        // TODO: ここでサーバに接続して、電力センサーからの値を送信する。

        println!(
            "A2 sensor reading: {}mV",
            powered_adc1.read(&mut a2).unwrap()
        );
    })?;

    periodic_timer.every(Duration::from_millis(8))?;

    Ok(periodic_timer)
}
