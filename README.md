# A MWE for a bug in `cortex-m-rt`

_Disclaimer: I'm not sure, where the error is. I totally could be the case, that I'm using the `cortex-m-rt` crate wrong, so it's a layer 8 problem. That has to be confirmed._

I noticed a bug in the startup code of the `cortex-m-rt` crate when targeting a thumbv6m-controller.
This repository contains a minimal working example for bug reporting.

# Steps to reproduce
## Required tools and hardware
- rustc with the `thumbv6m-none-eabi` target installed
- `arm-none-eabi-gdb` and a debugger like `openocd` (this MWE will use openocd)
- a Cortex-M0 or Cortex-M0+ microcontroller (this MWE will use a EFM32TG11B120F128, but every other Cortex-M0(+) will work)
- a debug probe

## Commands
The repository contains a `.cargo/config`, that has already set up the environment in order to correctly compile, link and run the binary on the target via `arm-none-eabi-gdb`.
This means, that your usual `cargo run` will invoke the correct commands and you will be able to debug.
```bash
# start the debugger server (may be different for you)
$ openocd -f interface/jlink.cfg -c "transport select swd" -f target/efm32.cfg
# build and run the example
$ cargo run
# your now in the gdb-shell and should be stopped in the `Reset` function
(gdb) n
(gdb) s
# you should now see the `while sbss < ebss {`-line
# print the values of the `sbss` and `ebss` variables
(gdb) p sbss # will print `(*mut u32) 0x20000000 <cortex_m_rt_bug_mwe::GLOBAL_ARRAY>`
(gdb) p ebss # will print `(*mut u32) 0x20000400`
# print the address of `sbss`
(gdb) p &sbss # will print `(*mut *mut u32) 0x200003f0 <cortex_m_rt_bug_mwe::GLOBAL_ARRAY+1008>`
```
Note that the `sbss` variable is located inside of the .bss-range.
This range will cleared, i.e. set to 0.
This means, that the `sbss` variable will be reset to 0 before it reaches the value of `ebss`, i.e. before the loop ends.
This leads to a endless loop, which can be validated using:
```bash
(gdb) c # this will run infinite. Hit <ctrl+c> after some time
(gdb) n
... # send n until you see the `while sbss < ebss {`-line again
(gdb) p sbss # will most likely print a number less than the initial value
# end the debugging
(gdb) q
Quit anyway? (y or n) y
```
Et voilà, the bug is demonstrated.

But wait, there is one more thing!
If you try the same thing with optimizations enabled, the behavior will change:
1. try with `cargo run --release` and it will work
2. change the `Cargo.toml` and set the `opt-level` to 1:
   ```toml
   [profile.dev]
   opt-level = 1
   ```
   and run the commands above.
   You will get a HardFault!

# Compiler metadata
This behavior is visible on both `rustc 1.35.0 (3c235d560 2019-05-20)` and `rustc 1.37.0-nightly (03ee55bb1 2019-06-01)`.

The (reduced) disassemblies look very similar, have a look at `disassemblies/` folder.
