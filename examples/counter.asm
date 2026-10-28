; Count button presses on GP3 and show the low three bits on GP0, GP1, GP2.
; Interrupt-on-change wakes the part from SLEEP for each edge.
        __config 0x3F94
presses equ 0x20
last    equ 0x21

        org 0
        goto start
        org 4
isr     bcf INTCON,GPIF
        retfie

start   movlw 7
        movwf CMCON         ; comparator off
        bsf STATUS,RP0
        clrf ANSEL          ; all digital
        movlw 0x38
        movwf TRISIO        ; GP0-GP2 out, GP3-GP5 in
        movlw 0x08
        movwf IOC           ; interrupt on change of GP3
        bcf STATUS,RP0
        clrf presses
        clrf GPIO
        movf GPIO,w         ; latch the pins for the change compare
        movwf last
        movlw 0x88
        movwf INTCON        ; GIE | GPIE

wait    sleep
        nop
        movf GPIO,w
        movwf last
        btfss last,3        ; count the falling edges: GP3 now low
        incf presses,f
        movf presses,w
        andlw 7
        movwf GPIO
        goto wait
