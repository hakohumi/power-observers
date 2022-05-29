use embedded_svc::mqtt::client::utils::ConnState;
use embedded_svc::mqtt::client::{MessageImpl, QoS};
use embedded_svc::sys_time::SystemTime;
use embedded_svc::timer::{OnceTimer, PeriodicTimer, TimerService};
use esp_idf_svc::eventloop::{
    EspBackgroundEventLoop, EspBackgroundSubscription, EspEventFetchData, EspEventPostData,
    EspTypedEventDeserializer, EspTypedEventSerializer, EspTypedEventSource,
};
use esp_idf_svc::mqtt::client::EspMqttClient;
use esp_idf_svc::systime::EspSystemTime;
use esp_idf_svc::timer::{EspTimer, EspTimerService};
use esp_idf_sys::{self as _, c_types, EspError}; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

use anyhow::{Ok, Result};
use embedded_hal::adc::OneShot;
use esp_idf_hal::adc;
use esp_idf_hal::prelude::Peripherals;
use log::*;
use std::sync::{Condvar, Mutex};
use std::thread;
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

            // println!(
            //     "Hall sensor reading: {}mV",
            //     powered_adc1.read(&mut hall_sensor).unwrap()
            // );
            // println!(
            //     "A2 sensor reading: {}mV",
            //     powered_adc1.read(&mut a2).unwrap()
            // );

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
        // client
        //     .publish(
        //         "rust-esp32-std-demo",
        //         QoS::AtMostOnce,
        //         false,
        //         format!("Now is {:?}", now).as_bytes(),
        //     )
        //     .unwrap();
    })?;

    periodic_timer.every(Duration::from_secs(5))?;

    Ok(periodic_timer)
}

#[derive(Copy, Clone, Debug)]
struct EventLoopMessage(Duration);

impl EventLoopMessage {
    pub fn new(duration: Duration) -> Self {
        Self(duration)
    }
}

impl EspTypedEventSource for EventLoopMessage {
    fn source() -> *const c_types::c_char {
        b"DEMO-SERVICE\0".as_ptr() as *const _
    }
}

impl EspTypedEventSerializer<EventLoopMessage> for EventLoopMessage {
    fn serialize<R>(
        event: &EventLoopMessage,
        f: impl for<'a> FnOnce(&'a EspEventPostData) -> R,
    ) -> R {
        f(&unsafe { EspEventPostData::new(Self::source(), Self::event_id(), event) })
    }
}

impl EspTypedEventDeserializer<EventLoopMessage> for EventLoopMessage {
    fn deserialize<R>(
        data: &EspEventFetchData,
        f: &mut impl for<'a> FnMut(&'a EventLoopMessage) -> R,
    ) -> R {
        f(unsafe { data.as_payload() })
    }
}
fn test_eventloop() -> Result<(EspBackgroundEventLoop, EspBackgroundSubscription)> {
    use embedded_svc::event_bus::EventBus;

    println!("About to start a background event loop");
    let mut eventloop = EspBackgroundEventLoop::new(&Default::default())?;

    println!("About to subscribe to the background event loop");
    let subscription = eventloop.subscribe(|message: &EventLoopMessage| {
        println!("Got message from the event loop: {:?}", message.0);
    })?;

    Ok((eventloop, subscription))
}
