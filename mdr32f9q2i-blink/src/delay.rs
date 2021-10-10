use cortex_m::peripheral::syst::SystClkSource;
use cortex_m::peripheral::SYST;

pub const HFCLK_FREQ: u32 = 8_000_000;

/// System timer (SysTick) as a delay provider.
pub struct Delay {
    syst: SYST,
}

impl Delay {
    /// Configures the system timer (SysTick) as a delay provider.
    pub fn new(mut syst: SYST) -> Self {
        syst.set_clock_source(SystClkSource::Core);

        Delay { syst }
    }

    /// Releases the system timer (SysTick) resource.
    pub fn free(self) -> SYST {
        self.syst
    }

    pub fn delay_ms(&mut self, ms: u32) {
        self.delay_us(ms * 1_000);
    }

    pub fn delay_us(&mut self, us: u32) {
        // The SysTick Reload Value register supports values between 1 and 0x00FFFFFF.
        const MAX_RVR: u32 = 0x00FF_FFFF;

        let mut total_rvr = us * (HFCLK_FREQ / 1_000_000);

        while total_rvr != 0 {
            let current_rvr = if total_rvr <= MAX_RVR {
                total_rvr
            } else {
                MAX_RVR
            };

            self.syst.set_reload(current_rvr);
            self.syst.clear_current();
            self.syst.enable_counter();

            // Update the tracking variable while we are waiting...
            total_rvr -= current_rvr;

            while !self.syst.has_wrapped() {}

            self.syst.disable_counter();
        }
    }
}
