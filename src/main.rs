#![no_std]
#![no_main]

use {{ mcu }}_hal::{pac::Peripherals, prelude::*};
use esp_backtrace as _;
{% if mcu == "esp32c3" -%}
use riscv_rt::entry;
{%- else -%}
use xtensa_lx_rt::entry;
{%- endif %}

#[entry]
fn main() -> ! {
    let _peripherals = Peripherals::take().unwrap();

    loop {}
}
