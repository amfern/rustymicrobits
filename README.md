# goal
I wan't to build some micro controller in rust and the write a linux kernel driver in rust for that microcontroller.
And i want to use the rt_prempt kernel patch to run the driver in real time... :)

## build
will produce target/thumbv7m-none-eabi/debug/rustymicrobits

``` shell
cargo check
cargo build
```

## flash


``` shell
cargo dfu

```


## notes



https://cdn.shopifycdn.net/s/files/1/0604/5905/7341/files/Holybro_Kakute_F7_AIO_V1.5_Manual.pdf?v=1679587701
file:///home/ilya/Downloads/rm0385-stm32f75xxx-and-stm32f74xxx-advanced-armbased-32bit-mcus-stmicroelectronics.pdf

USART3 pinout
https://github.com/iNavFlight/inav/blob/8ac5703a558a4b1367cf2b64b90355019525e61f/src/main/target/KAKUTEF7/target.h#L59-L60
https://github.com/stm32-rs/stm32f7xx-hal/blob/eaa235c4067a96e60c45e482d44cb97cacf4ca33/src/serial.rs#L102-L108


1. convert the elf file into .dfu format, look how i was doing it in the xtend source code
2. use dfu-tool to flash it dfu-tool write --verbose -d 0483:df11 obj/fcos_v3.4.0-a1_HUNTER_V0_DIGITAL_KAKUTEF7.dfu
   a. ./elf2dfuse target/thumbv7m-none-eabi/debug/rustymicrobits  target/thumbv7m-none-eabi/debug/rustymicrobits.dfu
   b. fwupdtool install-blob --plugins dfu --no-reboot-check target/thumbv7m-none-eabi/debug/rustymicrobits.dfu

3. need to install dfu-tool first

use for concurrency
https://github.com/embassy-rs/embassy



https://docs.kernel.org/rust/index.html


Maybe buy a different hardware, an actual hardware suitable for development https://embassy.dev/book/index.html#_nrf_kits 
