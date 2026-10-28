; Blink an LED on GP0 from TMR0, on the PIC12C508.
; The window opens with this when the part is switched to the 12C508.
;
; The baseline core has no interrupts and no TMR0 overflow flag, so the loop
; watches TMR0's top bit go high and then fall back to 0 as the count wraps.
; The part starts at its last word, 0x1FF, where the factory's MOVLW leaves
; the oscillator calibration in W; the first instruction stores it.

        processor 12C508
        __config 0xFEA      ; INTRC, WDT off, no code protect, MCLR off
count   equ 0x07

        org 0
start   movwf OSCCAL        ; keep the factory calibration
        movlw 0xC7
        option              ; TMR0 from Fosc/4, prescaler 1:256
        movlw 0x3E
        tris GPIO           ; GP0 is an output
        clrf GPIO
        clrf count
top     btfss TMR0,7        ; wait for the top half of the count
        goto top
wrap    btfsc TMR0,7        ; then for it to wrap to 0
        goto wrap
        movlw 1
        xorwf GPIO,f        ; toggle GP0
        incf count,f
        goto top
