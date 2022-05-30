pub struct PowerSensor {
    pub counter: u32,
    pub acc_adc_value: u32,
    pub average_adc: u32,
}

// TODO: リングバッファ方式の平均

impl PowerSensor {
    pub fn init(counter: u32, adc_value: u32, average_adc: u32) -> Self {
        let app = Self {
            counter,
            acc_adc_value: adc_value,
            average_adc,
        };
        app
    }

    pub fn add_adc_value(&mut self, adc_value: u32) {
        self.acc_adc_value += adc_value;
    }
}
