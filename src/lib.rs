use anyhow::{Ok, Result};
use embedded_svc::event_bus::Postbox;

use std::sync::{Condvar, Mutex};
use std::thread;
use std::{sync::Arc, time::*};

mod timer;
use timer::init_timer;

mod power_sensor;

mod eventloop;
use eventloop::test_eventloop;

use crate::eventloop::EventLoopMessage;

pub fn run() -> Result<()> {
    println!("Hello, world!");

    for s in 0..5 {
        println!("Start program in {} secs", 5 - s);
        thread::sleep(Duration::from_secs(1));
    }

    let _timer = init_timer()?;

    let mutex = Arc::new((Mutex::new(None), Condvar::new()));

    let (mut event, _subscription) = test_eventloop().unwrap();

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
                .post(&EventLoopMessage::new(Duration::from_secs(1)), None)
                .unwrap();
        }
    };
    Ok(())
}
