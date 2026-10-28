; Count button presses on GP3 and show the low three bits on GP0, GP1, GP2,
; on the PIC12C508.
;
; The baseline core has no interrupts. Instead the part SLEEPs with
; wake-on-change on, and a change on GP3 wakes it with a reset. GPWUF in
; STATUS tells that reset from power-on, and RAM keeps the count across it.
; TRIS and OPTION go back to their reset values, so they are set again each
; time; the GPIO latch keeps the pins' last value.

        processor 12C508
        __config 0xFEA      ; INTRC, WDT off, no code protect, MCLR off
presses equ 0x07

        org 0
start   movwf OSCCAL        ; keep the factory calibration
        movlw 0x0F
        option              ; wake-on-change and pull-ups on
        movlw 0x38
        tris GPIO           ; GP0-GP2 out, GP3-GP5 in
        btfsc STATUS,GPWUF  ; woken by a change on GP3?
        goto woke
        clrf presses        ; no: power-on, nothing pressed yet
        goto show
woke    btfss GPIO,3        ; count the falling edges: GP3 now low
        incf presses,f
show    movf presses,w
        andlw 7
        movwf GPIO
        movf GPIO,w         ; read the pins: what a change is measured from
        sleep
