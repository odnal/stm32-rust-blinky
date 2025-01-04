# stm32 blinky example with cargo (baremetal)

## Getting Started

You need an STM32 device. I used openocd for the programmer. After installing openocd it comes with support for a variety of different STM32
devices and other mcu's.

### Installing OpenOCD
```bash
sudo apt install openocd
```

### 1. Build the project
```bash
cargo build
```

### 2. Prepare openocd and Flash the Program
Open two terminal windows.

#### Window 1: Start openocd with appropriate board config file.
```bash 
openocd -f board/stm32l4discovery.cfg

```

#### Window 2: Connect to openocd using telnet and flash device.
```bash 
telnet localhost 4444
> halt
> flash write_image erase target/thumbv7em-none-eabi/debug/blinky
> reset
```
