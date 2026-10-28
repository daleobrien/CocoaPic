; A busy loop for the benchmark: no SLEEP and no waiting on a flag, just a
; spread of the instruction mix a real program runs. Each pass fills 16 GPRs
; (0x30..0x3F) through FSR/INDF, sums them with carries, shifts, and calls a table lookup.
        __config 0x3F94     ; INTOSC, WDT off, MCLR off
sum     equ 0x20
sumhi   equ 0x21
n       equ 0x22
tmp     equ 0x23

        org 0
start   movlw 7
        movwf CMCON         ; comparator off
        bsf STATUS,RP0
        clrf ANSEL
        clrf TRISIO         ; all outputs
        movlw 0x88
        movwf OPTION_REG    ; TMR0 from Fosc/4, no prescaler
        bcf STATUS,RP0

pass    movlw 0x30          ; fill 0x30..0x3F with n*3 + 1
        movwf FSR
        movlw 16
        movwf n
fill    movf n,w
        movwf tmp
        addwf tmp,w
        addwf tmp,w
        addlw 1
        movwf INDF
        incf FSR,f
        decfsz n,f
        goto fill

        clrf sum            ; sum them into sumhi:sum
        clrf sumhi
        movlw 0x30
        movwf FSR
        movlw 16
        movwf n
add     movf INDF,w
        addwf sum,f
        btfsc STATUS,C
        incf sumhi,f
        incf FSR,f
        decfsz n,f
        goto add

        bcf STATUS,C        ; mix: rotate, swap, xor, look up
        rlf sum,f
        rrf sumhi,f
        swapf sum,w
        xorwf sumhi,w
        andlw 3
        call table
        movwf GPIO
        comf GPIO,f
        goto pass

table   addwf PCL,f
        retlw 0x01
        retlw 0x02
        retlw 0x04
        retlw 0x08
