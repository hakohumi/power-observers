use anyhow::{Ok, Result};

use std::sync::{Condvar, Mutex};
use std::thread;
use std::{sync::Arc, time::*};

mod eventloop;
use crate::eventloop::test_eventloop;

mod timer;
use timer::init_timer;

mod power_sensor;

pub fn run() -> Result<()> {
    println!("Hello, world!");

    for s in 0..5 {
        println!("Start program in {} secs", 5 - s);
        thread::sleep(Duration::from_secs(1));
    }

    let (eventloop, _subscription) = test_eventloop()?;
    let _timer = init_timer(eventloop)?;

    let mutex = Arc::new((Mutex::new(None), Condvar::new()));

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
        }
    };
    Ok(())
}
