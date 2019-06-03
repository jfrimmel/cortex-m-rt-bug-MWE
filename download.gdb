target extended-remote :3333
load

break Reset
break HardFault
tbreak main
set print asm-demangle on
monitor arm semihosting enable

continue
