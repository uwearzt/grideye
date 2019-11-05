# [Grid-EYE](https://crates.io/crates/grideye)

[![Apache licensed](https://img.shields.io/badge/license-Apache-blue.svg)](http://www.apache.org/licenses/LICENSE-2.0)
[![Build Status](https://travis-ci.org/uwearzt/grideye.svg?branch=master)](https://travis-ci.org/uwearzt/grideye)
[![Actions Status](https://github.com/uwearzt/grideye/workflows/push_pullreq/badge.svg)](https://github.com/uwearzt/grideye/actions)
[![crates.io](https://meritbadge.herokuapp.com/grideye)](https://crates.io/crates/grideye)

Rust driver for Grid-EYE / Panasonic AMG88(33)

## Documentation

 Read the detailed documentation [here](https://docs.rs/grideye/)

## Build

Rust nightly is needed to compile this crate.

### Raspberry Pi

```bash
cargo build --example raspberrypi
cargo run --example raspberrypi
```

### STM32

The STM32 is tested on a [1bitsy](http://1bitsy.org) board with a STM32F415RGT6 CPU.

```bash
cargo build --example stm32 --target thumbv7em-none-eabi
JLinkGDBServer -if SWD -device STM32F415RG
arm-none-eabi-gdb -x gdb.startup
```

## ToDo

- [ ] STM32 example
- [ ] add interrupts
- [ ] API documentation
- [ ] refactoring

## Done

- [x] GitHub actions
- [x] get the primary sensor readouts
- [x] add complete API
- [x] Travis CI

## License

[Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)

## Resources

- [Datasheet](https://industrial.panasonic.com/cdbs/www-data/pdf/ADI8000/ADI8000C59.pdf)
- [Evaluation Board](https://www.sparkfun.com/products/14607)
