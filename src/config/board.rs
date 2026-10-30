use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::peripherals::Peripherals;
use esp_hal::timer::timg::TimerGroup;

pub struct Board {
    pub status_led: Output<'static>,
}

impl Board {
    pub fn init(peripherals: Peripherals) -> Self {
        let timg0 = TimerGroup::new(peripherals.TIMG0);
        esp_rtos::start(timg0.timer0, peripherals.FROM_CPU_INTR0);

        let output_cfg = OutputConfig::default();
        let status_led = Output::new(peripherals.GPIO2, Level::Low, output_cfg);
        
        Self {
            status_led,
        }
    }
}
