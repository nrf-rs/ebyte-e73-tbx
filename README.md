# Board Support Package for the Ebyte E73-TBA and E73-TBB

This crate is a Board Support Package (BSP). It wraps the HAL crate
(`nrf52832-hal` or `nrf52810-hal`) for the on-board MCU, and provides high level
wrappers for the onboard features.

## Usage

Select the variant of your board via Cargo features:

|   Board   |   MCU    | Feature |
|-----------|----------|---------|
| [E73-TBA] | nRF52810 |  `tba`  |
| [E73-TBB] | nRF52832 |  `tbb`  |

For example:

```toml
[dependencies.ebyte-e73-tbx-bsp]
version = "x.y.z"
features = ["tba"]
```

You will require the `thumbv7em-none-eabihf` target installed. To build one of
these examples:

```console
$ rustup target add thumbv7em-none-eabihf
$ git clone https://github.com/nrf-rs/nrf-bsp-template.git
$ cd nrf-bsp-template
$ cargo build --target=thumbv7em-none-eabihf --example blinky
```

To use in your own application, add as a dependency and call the
`Board::take()` function.

[E73-TBA]: http://ebyte.com/en/product-view-news.aspx?id=888
[E73-TBB]: http://ebyte.com/en/product-view-news.aspx?id=889

## Minimum Supported Rust Version

This crate is guaranteed to build on stable Rust 1.43 and higher.

## Licence

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
