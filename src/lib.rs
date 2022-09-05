use anyhow::{Ok, Result};
use embedded_svc::event_bus::Postbox;
use esp_idf_svc::netif::EspNetifStack;
use esp_idf_svc::nvs::EspDefaultNvs;
use esp_idf_svc::sysloop::EspSysLoopStack;
use log::info;

use std::sync::{Condvar, Mutex};
use std::thread;
use std::{sync::Arc, time::*};

mod timer;
use timer::init_timer;

mod power_sensor;
mod wifi;
use wifi::wifi;

mod eventloop;


pub fn run() -> Result<()> {
    println!("Hello, world!");

    for s in 0..5 {
        println!("Start program in {} secs", 5 - s);
        thread::sleep(Duration::from_secs(1));
    }
    let netif_stack = Arc::new(EspNetifStack::new()?);
    let sys_loop_stack = Arc::new(EspSysLoopStack::new()?);
    let default_nvs = Arc::new(EspDefaultNvs::new()?);

    let mut wifi = wifi(
        netif_stack.clone(),
        sys_loop_stack.clone(),
        default_nvs.clone(),
    )?;

    let _timer = init_timer()?;

    let mutex = Arc::new((Mutex::new(None), Condvar::new()));

    let (mut event, _subscription) = eventloop::test_eventloop().unwrap();

    let mut wait = mutex.0.lock().unwrap();

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
            event
                .post(&eventloop::EventLoopMessage::new(Duration::from_secs(1)), None)
                .unwrap();
        }
    };

    drop(wifi);
    info!("Wifi stopped");

    Ok(())
}
