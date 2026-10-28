; Blink an LED on GP0 from the TMR0 overflow flag.
; The same program the window opens with.

        __config 0x3F94     ; INTOSC, WDT off, MCLR off
count   equ 0x20

        org 0
start   movlw 7
        movwf CMCON         ; comparator off: GP0/GP1 digital
        bsf STATUS,RP0      ; bank 1
        clrf ANSEL          ; no analog inputs
        movlw 0x3E
        movwf TRISIO        ; GP0 is an output
        movlw 0x87
        movwf OPTION_REG    ; TMR0 from Fosc/4, prescaler 1:256
        bcf STATUS,RP0      ; bank 0
        clrf GPIO
        clrf count
loop    btfss INTCON,T0IF   ; wait for TMR0 to overflow
        goto loop
        bcf INTCON,T0IF
        movlw 1
        xorwf GPIO,f        ; toggle GP0
        incf count,f
        goto loop
