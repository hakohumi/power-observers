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

    let mut hall_sensor = peripherals.hall_sensor;

    let my_power_sensor = Arc::new(Mutex::new(PowerSensor::init(0, 0, 0)));

    let my1 = Arc::clone(&my_power_sensor);
    let my2 = Arc::clone(&my_power_sensor);

    let mut read_timer = EspTimerService::new()?.timer(move || {
        let mut _my_power_sensor = my1.lock().unwrap();
        _my_power_sensor.counter += 1;

        if _my_power_sensor.counter > 9 {
            _my_power_sensor.counter = 0;

            _my_power_sensor.average_adc = _my_power_sensor.acc_adc_value / 10;
            println!(
                "_my_power_sensor.adc_value {} average adc {}",
                _my_power_sensor.acc_adc_value, _my_power_sensor.average_adc
            );
            _my_power_sensor.acc_adc_value = 0;
        }

        _my_power_sensor.add_adc_value(powered_adc1.read(&mut hall_sensor).unwrap() as u32);
        println!("adc value {}", _my_power_sensor.acc_adc_value);
        // adc_value += powered_adc1.read(&mut a2).unwrap() as u32;
    })?;

    let mut print_timer = EspTimerService::new()?.timer(move || {
        println!("timer test {:?}", my2.lock().unwrap().counter);
        println!("A2 sensor reading: {}mV", my2.lock().unwrap().average_adc);
        // TODO: ここでサーバに接続して、電力センサーからの値を送信する。
    })?;

    print_timer.every(Duration::from_secs(1))?;

    read_timer.every(Duration::from_millis(8))?;

    Ok((read_timer, print_timer))
}
