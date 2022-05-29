use embedded_svc::sys_time::SystemTime;
use embedded_svc::timer::{OnceTimer, PeriodicTimer, TimerService};

use anyhow::{Ok, Result};
use esp_idf_svc::eventloop::EspBackgroundEventLoop;
use esp_idf_svc::systime::EspSystemTime;
use esp_idf_svc::timer::{EspTimer, EspTimerService};
use std::thread;
use std::time::Duration;

use crate::eventloop::EventLoopMessage;

pub fn init_timer(mut eventloop: EspBackgroundEventLoop) -> Result<EspTimer> {
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
