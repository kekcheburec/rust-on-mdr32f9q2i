#![no_main]
#![no_std]

mod delay;

#[panic_handler] // panicking behavior
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = mdr32f9q2l_pac::Peripherals::take().unwrap();
    let p_core = cortex_m::Peripherals::take().unwrap();

    p.MDR_RST_CLK.per_clock.write(|w| {
        w.pclk_portb().set_bit();
        w
    });

    p.MDR_PORTB.oe.write(|w| {
        w.oe_0().set_bit();
        w
    });

    p.MDR_PORTB.analog.write(|w| {
        w.analog_en_0().set_bit();
        w
    });

    p.MDR_PORTB.pwr.write(|w| {
        unsafe {
            w.pwr_0().bits(0b01);
        }
        w
    });

    let mut delay = delay::Delay::new(p_core.SYST);

    loop {
        p.MDR_PORTB.rxtx.write(|w| {
            w.rxtx_0().set_bit();
            w
        });

        delay.delay_ms(1000);

        p.MDR_PORTB.rxtx.write(|w| {
            w.rxtx_0().clear_bit();
            w
        });

        delay.delay_ms(1000);
    }
}
