use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::timer::timg::TimerGroup;
use esp_println::println;
use esp_hal::clock::CpuClock;

pub struct Board {
    pub status_led: Output<'static>,
}

impl Board {
    pub fn init() -> Self {
        let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
        let peripherals = esp_hal::init(config);

        println!("Peripherals config done");

        let timg0 = TimerGroup::new(peripherals.TIMG0); 
        esp_rtos::start(timg0.timer0, peripherals.FROM_CPU_INTR0);

        println!("Configured timer group");

        let output_cfg = OutputConfig::default();
        let status_led = Output::new(peripherals.GPIO2, Level::Low, output_cfg);
        
        println!("Board config done");

        Self {
            status_led,
        }
    }
}
