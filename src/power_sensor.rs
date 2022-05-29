
pub struct PowerSensor {
    counter: Box<u32>,
}

impl PowerSensor {
    pub fn init(mut args: Box<u32>) -> Self {
        let app = Self { counter: args };
        app
    }

    pub fn get_counter(&self) {
        &self._counter;
    }

    pub fn add_count(&self) {
        let mut _counter = &self.counter;
        &self.counter += 1;
    }

    pub fn read(&self) {}
}
