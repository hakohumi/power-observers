use embedded_svc::timer::{PeriodicTimer, TimerService};

use anyhow::{Ok, Result};
use esp_idf_svc::timer::{EspTimer, EspTimerService};
use std::time::Duration;

use embedded_hal::adc::OneShot;
use esp_idf_hal::adc;
use esp_idf_hal::prelude::Peripherals;


pub fn init_timer() -> Result<(EspTimer, EspTimer)> {
    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;

    let mut a2 = pins.gpio34.into_analog_atten_11db()?;
    let mut powered_adc1 = adc::PoweredAdc::new(
        peripherals.adc1,
        adc::config::Config::new().calibration(true),
    )?;

    let mut _counter = 0;

    let mut read_timer = EspTimerService::new()?.timer(move || {
        _counter += 1;
        if _counter > 10 {
            _counter = 0;
        }

        println!("eventloop test {:?}", _counter);

        // TODO: ここでサーバに接続して、電力センサーからの値を送信する。
    })?;

    let mut print_timer = EspTimerService::new()?.timer(move || {
        println!(
            "A2 sensor reading: {}mV",
            powered_adc1.read(&mut a2).unwrap()
        );
    })?;

    print_timer.every(Duration::from_secs(1))?;

    read_timer.every(Duration::from_millis(8))?;

    Ok((read_timer, print_timer))
}
