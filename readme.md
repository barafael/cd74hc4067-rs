Rust embedded-hal driver for CD74HC4067
============

This library is an [embedded-hal][] compliant driver for a GPIO-driven CD74HC4067 chip.
It is completely hardware-agnostic, only requiring 5 output pins to operate.
Given a compliant implementation of embedded-hal for a target, this driver should work there out-of-the-box.

# Blog post

To read more on this nice little driver, blog post [here](https://barafael.github.io/A-Platform-Agnostic-Driver-for-the-CD74HC4067/).

[embedded-hal]: https://github.com/japaric/embedded-hal.git

Testing embedded-hal with mocking
=============

The tests for the library also show a basic example of how to use embedded-hal-mock to test drivers when the hardware isn't there yet :) or on CI.
Note, the test coverage is [practically 100% line coverage](coverage.pdf).

The only exception are the calls to PhantomData::<...>, as they are excluded from the compiled binary by design.

License
-------

[MIT License](LICENSE).
