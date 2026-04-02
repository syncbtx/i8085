# i8085: A Cycle‑Accurate Intel 8085 Emulator

A **hardware‑parity, cycle‑accurate** emulator of the Intel 8085 microprocessor, written in Rust.  
It models every internal register, flag, pin, and bus cycle exactly as described in the Intel 8085 datasheet.

## Features

- **Strict hardware parity** – CPU state contains only real 8085 components (registers, flags, pins, machine cycle counter, T‑state ring counter, interrupt masks). No artificial fields.
- **Cycle accuracy** – Every T‑state of every instruction is explicitly modelled. Wait states, HALT, and interrupt timing follow the datasheet.
- **Compile‑time generated control ROM** – The instruction decode PLA is built as a constant table during compilation. Dispatch is a single jump table lookup.
- **Modular design** – Separate modules for ALU, Control Unit, Registers, Pins, Bus, Memory, and Motherboard.
- **Runs real 8085 binaries** – Includes a default bootloader test and can be extended to run any 8085 program.
