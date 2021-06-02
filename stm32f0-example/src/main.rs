#![no_std]
#![no_main]

use debugless_unwrap::DebuglessUnwrap;

use panic_halt as _;

#[cfg(feature = "println_debug")]
use rtt_target::{rprintln, rtt_init_print};

use stm32f0xx_hal::{
    gpio::{Output, Pin, PushPull},
    pac,
    prelude::*,
};

use cortex_m::{interrupt::free as disable_interrupts, Peripherals};

use panic_halt as _;

use cortex_m_rt::entry;

use stm32f0xx_hal::delay::Delay;

use cd74hc4067::*;

use picorand::{WyRand, RNG};

#[entry]
fn main() -> ! {
    #[cfg(feature = "println_debug")]
    {
        rtt_init_print!();

        rprintln!("Hello :)");
    }

    if let (Some(dp), Some(cp)) = (pac::Peripherals::take(), Peripherals::take()) {
        let mut flash = dp.FLASH;
        let mut rcc = dp.RCC.configure().freeze(&mut flash);

        let mut delay = Delay::new(cp.SYST, &rcc);

        let gpioa = dp.GPIOA.split(&mut rcc);

        let (pin_0, pin_1, pin_2, pin_3, pin_enable, mut _led) = disable_interrupts(|cs| {
            (
                gpioa.pa0.into_push_pull_output(cs).downgrade(),
                gpioa.pa1.into_push_pull_output(cs).downgrade(),
                gpioa.pa4.into_push_pull_output(cs).downgrade(),
                gpioa.pa8.into_push_pull_output(cs).downgrade(),
                gpioa.pa7.into_push_pull_output(cs).downgrade(),
                gpioa.pa5.into_open_drain_output(cs),
            )
        });

        let mut on_for = |duration: u32,
                          pin: u8,
                          mut hc: Cd74hc4067<
            Pin<Output<PushPull>>,
            Pin<Output<PushPull>>,
            DisabledState,
        >| {
            hc.set_channel_active(pin as u8).debugless_unwrap();
            let enabled = hc.enable().debugless_unwrap();

            delay.delay_ms(duration);

            enabled.disable().debugless_unwrap()
        };

        let mut disabled =
            cd74hc4067::Cd74hc4067::new(pin_0, pin_1, pin_2, pin_3, pin_enable).debugless_unwrap();

        let mut rng = RNG::<WyRand, u8>::new(0xDEADBEEF);

        let delay_time_ms: u32 = 2000;
        loop {
            let generated: u8 = rng.generate_range(0, 15);
            disabled = on_for(delay_time_ms, generated, disabled);
        }
    }
    loop {
        continue;
    }
}
