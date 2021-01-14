Rust embedded-hal driver for CD74HC4067
============

This library is an [embedded-hal][] compliant driver for a GPIO-driven CD74HC4067 chip.
It is completely hardware-agnostic, only requiring 5 output pins to operate.
Given a compliant implementation of embedded-hal for a target, this driver should work there out-of-the-box.

The chip is quite simple. It connects 1 of 16 analog IO lines to 1 signal line, depending on the 4 select pins.
The only thing to look out for is to disable the chip with the disable signal before changing the select pins to avoid activating an unwanted line for a short moment during switching.

Because of the simplicity of this chip, the driver is also nothing special. But that means it may serve as an example for an embedded-hal driver that uses type-state programming.
In this driver, type-state programming is used to ensure that:

* The chip starts out in disabled mode
* The select pins can only be changed while the chip is disabled
* Once the chip is enabled, it must be disabled before anything can be done to it

[embedded-hal]: https://github.com/japaric/embedded-hal.git

Testing embedded-hal with mocking
=============

The tests for the library also show a basic example of how to use embedded-hal-mock to test drivers when the hardware isn't there yet :) or on CI.
Note, the test coverage is [practically 100% line coverage](tarpaulin-report.html).

The only exception are the calls to PhantomData::<...>, as they are excluded from the compiled binary by design.

License
-------

[MIT License](LICENSE).
