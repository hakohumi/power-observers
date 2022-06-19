use embedded_svc::timer::{PeriodicTimer, TimerService};

use anyhow::{Ok, Result};
use esp_idf_svc::timer::{EspTimer, EspTimerService};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use embedded_hal::adc::OneShot;
use esp_idf_hal::adc;
use esp_idf_hal::prelude::Peripherals;

use crate::power_sensor::PowerSensor;

pub fn init_timer() -> Result<(EspTimer, EspTimer)> {
    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;

    let mut a2 = pins.gpio34.into_analog_atten_11db()?;
    let mut powered_adc1 = adc::PoweredAdc::new(
        peripherals.adc1,
        adc::config::Config::new().calibration(true),
    )?;

    let my_power_sensor = Arc::new(Mutex::new(PowerSensor::init()));

    let my1 = Arc::clone(&my_power_sensor);
    let my2 = Arc::clone(&my_power_sensor);

    let mut read_timer = EspTimerService::new()?.timer(move || {
        let mut _my_power_sensor = my1.lock().unwrap();
        let adc_read_value = powered_adc1.read(&mut a2).unwrap() as u32;
        println!("A2 sensor raw reading: {}mV", adc_read_value);
        _my_power_sensor.add_diff(adc_read_value);
    })?;

    let mut print_timer = EspTimerService::new()?.timer(move || {
        let mut _my_power_sensor = my2.lock().unwrap();
        println!(
            "A2 sensor reading: {}mV",
            _my_power_sensor.get_adc_average()
        );
        // TODO: ここでサーバに接続して、電力センサーからの値を送信する。
    })?;

    print_timer.every(Duration::from_secs(1))?;

    read_timer.every(Duration::from_millis(8))?;

    Ok((read_timer, print_timer))
}
