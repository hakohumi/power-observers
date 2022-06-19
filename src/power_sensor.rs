const WINDOW_NUM: usize = 100;

pub struct PowerSensor {
    pub counter: u32,
    pub acc_adc_value: u32,
    pub average_adc: u32,
    pub flg_average_calc: bool,
    pub old_value: u32,
    pub window: [u32; WINDOW_NUM],
}
impl PowerSensor {
    pub fn init() -> Self {
        let app = Self {
            counter: 0,
            acc_adc_value: 0,
            average_adc: 0,
            flg_average_calc: false,
            old_value: 0,
            window: [0; WINDOW_NUM], // https://www.yukimura-physics.com/entry/elemag32
        };
        app
    }

    pub fn add_adc_value(&mut self, adc_value: u32) {
        self.acc_adc_value += adc_value;
        self.counter += 1;
    }

    pub fn add_diff(&mut self, adc_value: u32) {
        self.acc_adc_value += (self.old_value - adc_value).pow(2) as u32;
        self.counter += 1;
    }

    pub fn get_adc_average(&mut self) -> u32 {
        self.average_adc = self.acc_adc_value as u32 / self.counter;
        self.init_state();
        self.average_adc
    }

    fn init_state(&mut self) {
        self.acc_adc_value = 0;
        self.counter = 0;
        self.flg_average_calc = false;
    }
}
