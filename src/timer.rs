use embedded_svc::sys_time::SystemTime;
use embedded_svc::timer::{PeriodicTimer, TimerService};

use anyhow::{Ok, Result};
use esp_idf_svc::eventloop::EspBackgroundEventLoop;
use esp_idf_svc::systime::EspSystemTime;
use esp_idf_svc::timer::{EspTimer, EspTimerService};
use std::time::Duration;

use crate::eventloop::EventLoopMessage;

pub fn init_timer(mut eventloop: EspBackgroundEventLoop) -> Result<EspTimer> {
    use embedded_svc::event_bus::Postbox;

    let mut periodic_timer = EspTimerService::new()?.timer(move || {
        println!("Tick from periodic timer");

        let now = EspSystemTime {}.now();

        eventloop.post(&EventLoopMessage::new(now), None).unwrap();

        println!("eventloop test");
    })?;

    periodic_timer.every(Duration::from_secs(5))?;

    Ok(periodic_timer)
}
