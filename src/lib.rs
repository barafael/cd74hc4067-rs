//! A driver for generic GPIO driven CD74HC4067
//!
//! This driver was built using [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://docs.rs/embedded-hal/~0.2
//!
//! TODO # Examples

#![deny(unsafe_code)]
#![deny(missing_docs)]
#![cfg_attr(not(test), no_std)]

use core::marker::PhantomData;

/// Errors of this crate
// TODO Error<E>
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Error {
    /// Error setting a pin
    PinError,
}

struct EnabledState;
struct DisabledState;

use embedded_hal as hal;

use hal::digital::v2::OutputPin;

/// A structure representing the 4 input pins and pin_enable pin
pub struct CD74HC4067<A, B, C, D, E, State> {
    pin_0: A,
    pin_1: B,
    pin_2: C,
    pin_3: D,
    pin_enable: E,
    state: PhantomData<State>,
}

impl<A, B, C, D, E> CD74HC4067<A, B, C, D, E, DisabledState>
where
    A: OutputPin,
    B: OutputPin,
    C: OutputPin,
    D: OutputPin,
    E: OutputPin,
{
    /// Create a new CD74HC4067 structure by passing in 5 GPIOs implementing the
    /// `OutputPin` trait for `a`, `b`, `c`, `d`
    /// Mux is initially disabled, and all select pins are set low, selecting channel 0.
    pub fn new(
        mut pin_0: A,
        mut pin_1: B,
        mut pin_2: C,
        mut pin_3: D,
        mut pin_enable: E,
    ) -> Result<Self, Error> {
        // Disable the mux
        pin_enable.set_high().map_err(|_| Error::PinError)?;
        // Set to output 0
        pin_0.set_low().map_err(|_| Error::PinError)?;
        pin_1.set_low().map_err(|_| Error::PinError)?;
        pin_2.set_low().map_err(|_| Error::PinError)?;
        pin_3.set_low().map_err(|_| Error::PinError)?;

        Ok(Self {
            pin_0,
            pin_1,
            pin_2,
            pin_3,
            pin_enable,
            state: PhantomData::<DisabledState>,
        })
    }

    /// Release the 5 GPIOs previously occupied
    pub fn release(self) -> (A, B, C, D, E) {
        (
            self.pin_0,
            self.pin_1,
            self.pin_2,
            self.pin_3,
            self.pin_enable,
        )
    }

    /// Enable the mux display by pulling `pin_enable` low
    pub fn enable(mut self) -> Result<CD74HC4067<A, B, C, D, E, EnabledState>, Error> {
        self.pin_enable.set_low().map_err(|_| Error::PinError)?;
        Ok(CD74HC4067 {
            pin_0: self.pin_0,
            pin_1: self.pin_1,
            pin_2: self.pin_2,
            pin_3: self.pin_3,
            pin_enable: self.pin_enable,
            state: PhantomData::<EnabledState>,
        })
    }

    /// Enable output `n`. `n` must be between 0 and 15 inclusive.
    pub fn set_output_active(&mut self, n: u8) -> Result<(), Error> {
        assert!(n < 16);
        let is_bit_set = |b: u8| -> bool { n & (1 << b) != 0 };

        if is_bit_set(0) {
            self.pin_0.set_high().map_err(|_| Error::PinError)?;
        } else {
            self.pin_0.set_low().map_err(|_| Error::PinError)?;
        }
        if is_bit_set(1) {
            self.pin_1.set_high().map_err(|_| Error::PinError)?;
        } else {
            self.pin_1.set_low().map_err(|_| Error::PinError)?;
        }
        if is_bit_set(2) {
            self.pin_2.set_high().map_err(|_| Error::PinError)?;
        } else {
            self.pin_2.set_low().map_err(|_| Error::PinError)?;
        }
        if is_bit_set(3) {
            self.pin_3.set_high().map_err(|_| Error::PinError)?;
        } else {
            self.pin_3.set_low().map_err(|_| Error::PinError)?;
        }
        Ok(())
    }
}

impl<A, B, C, D, E> CD74HC4067<A, B, C, D, E, EnabledState>
where
    A: OutputPin,
    B: OutputPin,
    C: OutputPin,
    D: OutputPin,
    E: OutputPin,
{
    /// Disable the mux display by pulling `pin_enable` high
    pub fn disable(mut self) -> Result<CD74HC4067<A, B, C, D, E, DisabledState>, Error> {
        self.pin_enable.set_high().map_err(|_| Error::PinError)?;
        Ok(CD74HC4067 {
            pin_0: self.pin_0,
            pin_1: self.pin_1,
            pin_2: self.pin_2,
            pin_3: self.pin_3,
            pin_enable: self.pin_enable,
            state: PhantomData::<DisabledState>,
        })
    }
}

#[cfg(test)]
mod tests {
    use embedded_hal_mock::pin::{
        Mock as PinMock, State as PinState, Transaction as PinTransaction,
    };

    use super::*;

    #[test]
    fn test_make_mux() {
        let pin_0 = PinMock::new(&[PinTransaction::set(PinState::Low)]);
        let pin_1 = PinMock::new(&[PinTransaction::set(PinState::Low)]);
        let pin_2 = PinMock::new(&[PinTransaction::set(PinState::Low)]);
        let pin_3 = PinMock::new(&[PinTransaction::set(PinState::Low)]);

        let expectations = [PinTransaction::set(PinState::High)];

        let pin_enable = PinMock::new(&expectations);

        let mux = CD74HC4067::new(pin_0, pin_1, pin_2, pin_3, pin_enable).unwrap();
        let (mut pin_0, mut pin_1, mut pin_2, mut pin_3, mut pin_enable) = mux.release();
        pin_enable.done();
        pin_0.done();
        pin_1.done();
        pin_2.done();
        pin_3.done();
    }

    #[test]
    fn test_enable() {
        let pin_0 = PinMock::new(&[PinTransaction::set(PinState::Low)]);
        let pin_1 = PinMock::new(&[PinTransaction::set(PinState::Low)]);
        let pin_2 = PinMock::new(&[PinTransaction::set(PinState::Low)]);
        let pin_3 = PinMock::new(&[PinTransaction::set(PinState::Low)]);

        let expectations = [
            PinTransaction::set(PinState::High),
            PinTransaction::set(PinState::Low),
            PinTransaction::set(PinState::High),
        ];

        let pin_enable = PinMock::new(&expectations);

        let mux = CD74HC4067::new(pin_0, pin_1, pin_2, pin_3, pin_enable).unwrap();
        let enabled_mux = mux.enable().unwrap();
        let mux = enabled_mux.disable().unwrap();

        let (mut pin_0, mut pin_1, mut pin_2, mut pin_3, mut pin_enable) = mux.release();
        pin_enable.done();
        pin_0.done();
        pin_1.done();
        pin_2.done();
        pin_3.done();
    }

    #[test]
    fn test_set_output_to_9() {
        let pin_0 = PinMock::new(&[PinTransaction::set(PinState::High)]);
        let pin_1 = PinMock::new(&[PinTransaction::set(PinState::Low)]);
        let pin_2 = PinMock::new(&[PinTransaction::set(PinState::Low)]);
        let pin_3 = PinMock::new(&[PinTransaction::set(PinState::High)]);

        let pin_enable = PinMock::new(&[]);

        let mut mux = CD74HC4067 {
            pin_0,
            pin_1,
            pin_2,
            pin_3,
            pin_enable,
            state: PhantomData::<DisabledState>,
        };

        mux.set_output_active(9).unwrap();

        let (mut pin_0, mut pin_1, mut pin_2, mut pin_3, mut pin_enable) = mux.release();

        pin_0.done();
        pin_1.done();
        pin_2.done();
        pin_3.done();
        pin_enable.done();
    }

    #[test]
    fn test_set_output_to_6() {
        let pin_0 = PinMock::new(&[PinTransaction::set(PinState::Low)]);
        let pin_1 = PinMock::new(&[PinTransaction::set(PinState::High)]);
        let pin_2 = PinMock::new(&[PinTransaction::set(PinState::High)]);
        let pin_3 = PinMock::new(&[PinTransaction::set(PinState::Low)]);

        let pin_enable = PinMock::new(&[]);

        let mut mux = CD74HC4067 {
            pin_0,
            pin_1,
            pin_2,
            pin_3,
            pin_enable,
            state: PhantomData::<DisabledState>,
        };

        mux.set_output_active(6).unwrap();

        let (mut pin_0, mut pin_1, mut pin_2, mut pin_3, mut pin_enable) = mux.release();

        pin_0.done();
        pin_1.done();
        pin_2.done();
        pin_3.done();
        pin_enable.done();
    }

    #[test]
    fn test_set_output_to_10() {
        let pin_0 = PinMock::new(&[PinTransaction::set(PinState::Low)]);
        let pin_1 = PinMock::new(&[PinTransaction::set(PinState::High)]);
        let pin_2 = PinMock::new(&[PinTransaction::set(PinState::Low)]);
        let pin_3 = PinMock::new(&[PinTransaction::set(PinState::High)]);

        let pin_enable = PinMock::new(&[]);

        let mut mux = CD74HC4067 {
            pin_0,
            pin_1,
            pin_2,
            pin_3,
            pin_enable,
            state: PhantomData::<DisabledState>,
        };

        mux.set_output_active(10).unwrap();

        let (mut pin_0, mut pin_1, mut pin_2, mut pin_3, mut pin_enable) = mux.release();

        pin_0.done();
        pin_1.done();
        pin_2.done();
        pin_3.done();
        pin_enable.done();
    }
}
