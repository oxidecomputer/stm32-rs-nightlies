# stm32h7
This crate provides an autogenerated API for access to STM32H7 peripherals.
The API is generated using [svd2rust] with patched svd files containing
extensive type-safe support. For more information please see the [main repo].

Refer to the [documentation] for full details.

[svd2rust]: https://github.com/japaric/svd2rust
[main repo]: https://github.com/stm32-rs/stm32-rs
[documentation]: https://docs.rs/stm32h7/latest/stm32h7/

## Usage
Each device supported by this crate is behind a feature gate so that you only
compile the device(s) you want. To use, in your Cargo.toml:

```toml
[dependencies.stm32h7]
version = "0.11.0"
features = ["stm32h743", "rt"]
```

The `rt` feature is optional and brings in support for `cortex-m-rt`.

In your code:

```rust
use stm32h7::stm32h743;

let mut peripherals = stm32h743::Peripherals::take().unwrap();
let gpioa = &peripherals.GPIOA;
gpioa.odr.modify(|_, w| w.odr0().set_bit());
```

For full details on the autogenerated API, please see:
https://docs.rs/svd2rust/0.17.0/svd2rust/#peripheral-api

## Supported Devices

| Module | Devices | Links |
|:------:|:-------:|:-----:|
| stm32h743 | STM32H743 | [RM0433](https://www.st.com/resource/en/reference_manual/dm00314099.pdf), [st.com](https://www.st.com/en/microcontrollers-microprocessors/stm32h743-753.html) |
| stm32h743v | STM32H743V | [RM0433](https://www.st.com/resource/en/reference_manual/dm00314099.pdf), [st.com](https://www.st.com/en/microcontrollers-microprocessors/stm32h743-753.html) |
| stm32h747cm4 | STM32H745 (CM4), STM32H747 (CM4), STM32H755 (CM4), STM32H757 (CM4) | [RM0399](https://www.st.com/resource/en/reference_manual/dm00176879.pdf), [st.com](https://www.st.com/en/microcontrollers-microprocessors/stm32h747-757.html) |
| stm32h747cm7 | STM32H745 (CM7), STM32H747 (CM7), STM32H755 (CM7), STM32H757 (CM7) | [RM0399](https://www.st.com/resource/en/reference_manual/dm00176879.pdf), [st.com](https://www.st.com/en/microcontrollers-microprocessors/stm32h747-757.html) |
| stm32h753 | STM32H753 | [RM0433](https://www.st.com/resource/en/reference_manual/dm00314099.pdf), [st.com](https://www.st.com/en/microcontrollers-microprocessors/stm32h743-753.html) |
| stm32h753v | STM32H753V | [RM0433](https://www.st.com/resource/en/reference_manual/dm00314099.pdf), [st.com](https://www.st.com/en/microcontrollers-microprocessors/stm32h743-753.html) |
| stm32h7b3 | STM32H7A3, STM32H7B3, STM32H7B0 | [RM0455](https://www.st.com/resource/en/reference_manual/dm00463927.pdf), [st.com](https://www.st.com/en/microcontrollers-microprocessors/stm32h7a3-7b3.html) |