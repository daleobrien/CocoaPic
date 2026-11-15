# What is simulated

[Back to the README](../README.md)

## PIC12F675

The 14-bit midrange core, shared with the PIC16 series:

- all 35 instructions, with their cycle counts;
- banked registers and their mirrors, INDF/FSR, and PCL/PCLATH computed jumps;
- the 8-level stack;
- TMR0 with its prescaler and T0CKI;
- the watchdog with its postscaler, which resets the chip or wakes it from SLEEP;
- TMR1 on Fosc/4, with gating;
- interrupts from TMR0, INT, GPIO change, TMR1, the A/D and the EEPROM;
- SLEEP;
- GPIO with TRISIO and weak pull-ups, and ANSEL/CMCON analog pins reading 0;
- the data EEPROM with its 0x55/0xAA unlock;
- the A/D converter.

## PIC12C508

The 12-bit baseline core:

- all 33 instructions, including OPTION and TRIS;
- the start at 0x1FF, whose factory MOVLW leaves the oscillator calibration in W;
- INDF/FSR, and PCL computed jumps and CALL, which reach only the first 256 words;
- the 2-level stack, whose bottom falls off;
- TMR0 with its prescaler and T0CKI, and no overflow flag;
- the watchdog with its postscaler;
- SLEEP, with every wake a reset that keeps RAM and sets GPWUF, /TO and /PD to
  say why;
- wake-on-change and weak pull-ups on GP0, GP1 and GP3;
- OSCCAL.

`examples/blink-12c508.asm` and `examples/counter-12c508.asm` are the 12F675's
two examples rewritten for it.

## Not simulated

- the comparator output and the voltage reference;
- the external TMR1 oscillator;
- GP3 as /MCLR;
- the 12C508's wake-up delay;
- the original app's PICkit USB reader, which needs the hardware to port and test.
