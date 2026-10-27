//#include <iostream>

#include <CoreServices/CoreServices.h>

const int clearbit[8] = { 0x00FE , 0x00FD , 0x00FB , 0x00F7 , 0x00EF , 0x00DF , 0x00BF , 0x007F };
const int setbit[8]   = { 0x0001 , 0x0002 , 0x0004 , 0x0008 , 0x0010 , 0x0020 , 0x0040 , 0x0080 };

    // CPU Memory
#define ROM_no_of_bytes 512
#define RAM_no_of_bytes 32

    int	ROM [ROM_no_of_bytes];			// ROM space
    int file [RAM_no_of_bytes];			// RAM space, contains ports etc
                                  // Debug stuff
    int Break_points [100];

    // Decoded instrustion data
    int d;
    int k;
    int f;
    int b;

    // CPU registors
    #define INDR_ADDR	file[0]		// Not physically impemented
#define RTCC		file[1] 	//  8 bit
#define PC_Pointer	file[2]		// 11bit
#define STATUS		file[3]
#define	FSR		file[4]
#define PORTA		file[5]
#define PORTB		file[6]
#define PORTC		file[7]
    // Bit locations of STATUS
#define PA2 7	// 	general purpose read/write bit
#define PA1	6	//	general purpose 	/ page select on some CPUs
#define PA0	5	//	general purpose 	/ page select on some CPUs
#define TO	4	// 	time out bit		/ set to 1 on powerup,CLRWDT and SLEEP, cleared by WDT out
#define PD	3	// 	Powerdown bit		/ set to 1 on powerup,CLRWDT, clear by SLEEP, WDT has no effect
#define Z	2	// 	Zero bit			/ set to zero if ALU or Logic is zero
#define DC	1	//	Borrow			/
#define C	0	// 	Carry bit			/
int	STACK1;
int	STACK2;
int temp;
int _W;
int _TRISA;		// 4 bit	x bit of porta (f5) is an output if 0 else an output if 1
int _TRISB;		// 8 bit
int _TRISC;		// 8 bit
int _OPTION;	// 6 bit
int WDT_timer;
int WDT_prescaler;
                        // Bit locations of OPTION
#define RTS	5 //  RTCC source is internal instruction cycle clock (CLKOUT) if cleared to 0, Transition on RTCC pin if set to 1
#define RTE	4 //  RTCC signal edge, low to high transistion if cleared to 0, high to low transistion if set to 1
#define PSA	3 // Prescaler assignment, 0 RTCC, 1 WDT
#define PS2	2 //
#define PS1	1 //
#define PS0	0 // Prescaler scale
                        // Prescaler
                        // PSx	RTCC rate		WDT rate
                        // 0 0 0		  2			  1
                        // 0 0 1		  4			  2
                        // 0 1 0		  8		 	  4
                        // 0 1 1		 16			  8
                        // 1 0 0 		 32			 16
                        // 1 0 1 		 64			 32
                        // 1 1 0		128			 64
                        // 1 1 1 		256			128

                        // Instruction set
                        	// Byte orientated file registors operations
                         //   (11-6)   (5)    (4-0)			<- the 12 bits of ROM
                         // [OPcode]  [d]  [ f(FILE#) ]
                         //
                        	void ADDWF();
                                void ANDLW();
                                void CLRF();
                                void CLRW();
                                void COMF();
                                void DECF();
                                void DECFSZ();
                                void INCF();
                                void INCFSZ();
                                void IORWF();
                                void MOVF();
                                void MOVWF();
                                void NOP();
                                void RLF();
                                void RRF();
                                void SUBWF();
                                void SWAPF();
                                void XORWF();
                                    // Bit orientated file registors operations
                                    //   (11-8)      (7-5)          (4-0)			<- the 12 bits of ROM
                                    // [OPcode]  [b(bite#)]  [ f(FILE#) ]
                                    //
                                    void BCF();
                                    void BSF();
                                    void BTFSC();
                                    void BTFSS();
                                     	 // Literal and Control operations
                                       //   (11-8)      (7-0)      					<- the 12 bits of ROM
                                       // [OPcode]  [k(Literal) ]
                                       //
                                      	void ANDWF();
                                        void CALL();
                                        void CLRWDT();
                                        void GOTO();
                                        void IORLW();
                                        void MOVLW();
                                        void OPTION();
                                        void RETLW();
                                        void SLEEP();
                                        void TRIS();
                                        void XORLW();
                                        // Interrupts if any

                                        
                                        // All instructions will assume that file[f] or what have you are of the correct length
void Init(int prog[512]){
    unsigned int j;

    // ROM gets set to this for now
    for (j=0;j<511;j++){ ROM[j] = prog[j]; }

    // RAM reset values
    PC_Pointer = 0x00ff;
    STATUS 	= 0;
    _W 	 	= 0;
    _OPTION 	= 0;
    FSR    	= 0;
    PC_Pointer = 0;			// should really point to above
}

                                        // The methods for each instruction
                                          
                                         	 void ADDWF(){
                                                    if (d==0){
                                                        _W 		= (_W + file[f]) & 0x00ff;
                                                        if(_W == 0) STATUS = STATUS || setbit[Z]; else STATUS = STATUS & clearbit[Z];
  		}else {
  			file[f] =  (_W + file[f]) & 0x00ff;	
 			if(file[f]==0) 	STATUS = STATUS | setbit[Z]; else STATUS = STATUS & clearbit[Z];
  		}
  	}
  	
 
  
     void ANDLW(){
    	_W = _W & k;
    	if(_W!=0) 		STATUS = STATUS | setbit[Z]; else STATUS = STATUS & clearbit[Z];		// Z bit
 	}
 	
   	
     void CLRF(){
 		file[f] = 0;
 		STATUS = STATUS | setbit[Z];
 	}
 
   
     void CLRW(){
 		_W = 0;
 		STATUS = STATUS | setbit[Z];
 	}
 
    
     void COMF(){
    	if (d==0){
    		_W = 0x00ff & (!file[f]);		
    		if(_W!=0) 		STATUS = STATUS | setbit[Z]; else STATUS = STATUS & clearbit[Z];		// Z bit
    	} else {
    		file[f] = 0x00ff & (!file[f]);
    		if(file[f]!=0)	STATUS = STATUS | setbit[Z]; else STATUS = STATUS & clearbit[Z];		// Z bit
    	}
 	}
  
   
     void DECF(){			// Decrement F and store to either f or W,
 		if( file[f]==0){
 			if(d==0) _W = 0x00ff; else file[f] = 0x00ff;
 			STATUS = STATUS & clearbit[Z];		// Z bit clear, non zero result
 		} else if( file[f]==1){
 		if(d==0) _W = 0; else file[f] = 0;
 			STATUS = STATUS | setbit[Z];		// Z bit set, zero result
 		} else{
 			if(d==0) _W = file[f]-1; else file[f] = file[f]-1;
 			STATUS = STATUS & clearbit[Z];		// Z bit clear, non zero result
 		}	
 	}
 
    
     void DECFSZ(){		// Decrement F and store to either f or W, if result is zero skip the next instruction
 		if( file[f]==0){
 			if(d==0) _W = 0x00ff; else file[f] = 0x00ff;
 			STATUS = STATUS & clearbit[Z];		// Z bit clear, non zero result
 		} else if( file[f]==1){
 			if(d==0) _W = 0; else file[f] = 0;
 			STATUS = STATUS | setbit[Z];		// Z bit set, zero result
 			PC_Pointer = PC_Pointer+1;
 	
 		} else{
 			if(d==0) _W = file[f]-1; else file[f] = file[f]-1;
 			STATUS = STATUS & clearbit[Z];		// Z bit clear, non zero result
 		}
 	
 		
 	}
     
   void INCF(){			// Increment F and store to either f or W
 		if( file[f]==0x00ff){
 			if(d==0) _W = 0x0000; else file[f] = 0x0000;
 			STATUS = STATUS | setbit[Z];	// Z bit set,  zero result
 		} else {
 			if(d==0) _W = file[f]+1; else file[f] = file[f]+1;
 			STATUS = STATUS & clearbit[Z];	// Z bit clear, non zero result
 		}	
 	}
 
    
     void INCFSZ(){		// Increment F and store to either f or W, if result is zero skip the next instruction
    	if( file[f]==0x00ff){
 			if(d==0) _W = 0x0000; else file[f] = 0x0000;
 			STATUS = STATUS | setbit[Z];
 			PC_Pointer = PC_Pointer+1;
 		} else {
 			if(d==0) _W = file[f]+1; else file[f] = file[f]+1;
 			STATUS = STATUS & clearbit[Z];		
 		}	
 	}
     
     void IORWF(){			// OR
    	if(d==0) {
    		_W = file[f] | _W;
    		if (_W==0) 		STATUS = STATUS | setbit[Z]; else STATUS = STATUS & clearbit[Z];		// Z bit
    	}
    	else {
    		file[f] = file[f] | _W;
    		if (file[f]==0) STATUS = STATUS | setbit[Z]; else STATUS = STATUS & clearbit[Z];		// Z bit
		}
 	}
 
      
     void MOVF(){		// Move F to (dest)
 		if(d==0) {
    		_W = file[f];
    		if (_W==0) 		STATUS = STATUS | setbit[Z]; else STATUS = STATUS & clearbit[Z];		// Z bit
    	}
    	else {
       		if (file[f]==0) STATUS = STATUS | setbit[Z]; else STATUS = STATUS & clearbit[Z];		// Z bit
		}
 	
 	}
	 
     void MOVWF(){			// Literal data is moved to W
    	k = _W;
 	}
 
 	 
     void NOP(){			// Do nothing, not really needed
 		
 	}
 
 	 
     void RLF(){			// Rotate Left through Carry bit <C>
       	if (file[f] & 0x0080)	STATUS = STATUS | setbit[C]; else 	STATUS = STATUS | clearbit[C];
 		if(d==0)	_W      = (file[f] << 1) & 0x00ff; 
    	else 		file[f] = (file[f] << 1) & 0x00ff;
    }
 
 	 
     void RRF(){			// Rotate right through Carry bit <C>
 		if (file[f] & 0x0001)	STATUS = STATUS | setbit[C]; else 	STATUS = STATUS | clearbit[C];
 		if(d==0)	_W      = (file[f] >> 1) & 0x00ff; 
    	else 		file[f] = (file[f] >> 1) & 0x00ff;
  	}
 	  
     void SUBWF(){			// Subtract        f-W -> (dest)
 		if (file[f] == _W)	STATUS = STATUS | setbit[Z]; else 	STATUS = STATUS | clearbit[Z];	// Zero Result bit
 		if (file[f] >= _W)	STATUS = STATUS | setbit[C]; else 	STATUS = STATUS | clearbit[C];	// Carry bit
 		if (file[f] >= _W)	STATUS = STATUS | setbit[DC]; else 	STATUS = STATUS | clearbit[DC];	// Borrow bit ?????
 		if (d==0)	_W		=	(file[f] - _W) & 0x00ff; 
		else 		file[f]	=	(file[f] - _W) & 0x00ff;
 	}
  
     void SWAPF(){			// Swap nibbles,  ie before f=0xA5  the after (dest) =0x5A
 		if (d==0) 	_W		 = ((file[f] & 0x00f0) >> 4 ) & ((file[f] & 0x000f) << 4 );
 		else		file[f]	 = ((file[f] & 0x00f0) >> 4 ) & ((file[f] & 0x000f) << 4 );
 	}
   
     void XORWF(){
 		_W = (_W^k);
 		if (_W==0) 			STATUS = STATUS | setbit[Z]; else 	STATUS = STATUS | clearbit[Z];	// Zero Result bit
 	}
 
   
     void BCF(){						// Bit Clear
		file[f] = file[f] & clearbit[b];
 	}
 	
  
     void BSF(){						// Bit set
		file[f] = file[f] | setbit[b];
 	}
 	
   	
     void BTFSC(){						// Bit test, skip next instruction if bit is clear
 		if ( file[f] & setbit[b]  == 0 ) PC_Pointer++;  WDT_timer++; RTCC++;
 	}
 	
    	
  	 void BTFSS(){						// Bit test, skip next instruction if bit is set
  		if ( file[f] & setbit[b]  == setbit[b] ) PC_Pointer++;  WDT_timer++; RTCC++;
 	}
 	
   	
  	 void ANDWF(){
		if ( (_W & file[f]) == 0 )	STATUS = STATUS & clearbit[Z]; else STATUS = STATUS | setbit[Z];		// Z bit
		if(d==0) _W = _W & file[f];	 else 	file[f] = _W & file[f];	// Z bit
 	}
  
    		
 	 void CALL(){						// Call Subroutine
 		STACK2 = STACK1;
 		STACK1 = PC_Pointer + 1;
 		PC_Pointer = k + PA0<<9 + PA1<<10;
 		STATUS = STATUS | clearbit[PA2];
 	}
     		
	 void CLRWDT(){
 		WDT_timer = 0;
 		WDT_prescaler = 0; // must check something here first
 		STATUS = STATUS & setbit[TO] & setbit[PD];
 	}
      		
	 void GOTO(){						// Change the PC pointer
 		PC_Pointer = k + PA0<<9 + PA1<<10;
 	}

	 
	 void IORLW(){					// w OR Literial
 		_W = _W | k;
		if ( _W == 0 )	STATUS = STATUS & setbit[Z]; else STATUS = STATUS | clearbit[Z];		// Z bit
 	}
 	 
	 void MOVLW(){					// Move Literial to w
		_W = k & 0x00ff;
 	}
 	  	
	 void OPTION(){				// Load the Option regisitor	
 		_OPTION = _W & 0x003f;	//  6 bits only
 	}
 	
 	  
	 void RETLW(){					// Return from Subroutine Call
		_W = k & 0x00ff;
		PC_Pointer = STACK1;
		STACK1 = STACK2;	// STACK is unchanged
		
 	}
 	  
	 void SLEEP(){
 	}
 	
 	 
	 void TRIS(){					// IO control of each port, really not sure on this one
 		if(f==5) _TRISA = _W & 0x000f;
 		if(f==6) _TRISB = _W & 0x00ff;
 		if(f==7) _TRISC = _W & 0x00ff;
 	}
 	
 	 
	 void XORLW(){
 		_W = _W^k;
 		if ( _W == 0 )	STATUS = STATUS & setbit[Z]; else STATUS = STATUS | clearbit[Z];		// Z bit
 	}
 	



void Execute(long no_of_CPU_cycles){
    int i;
    //int Nibble;
    int data;
    for(i=0; i< no_of_CPU_cycles;i++){
       // PC_Pointer = ( PC_Pointer < ROM_no_of_bytes) ?PC_Pointer:PC_Pointer++;  WDT_timer++; RTCC++;
        PC_Pointer++;  WDT_timer++; RTCC++;

        // Case statement approach
        data = ROM[ PC_Pointer&(ROM_no_of_bytes-1) ];
        // Display some data for debug reasons somehow here
        if (data < 0x0800){					// Use bitset test instead, should be alittle faster
            if (data < 0x0400){
                if (data < 0x0200){
                    if (data < 0x0100){		// 0x00xx ? 0x01xx
                        if (data < 0x0080){		// 0000 0xxx xxxx
                            if (data < 0x0040){		// 0000 00xx xxxx
                                if (data < 0x0020){		// 0000 000x xxxx
                                    if (data < 0x0008){		// 0000 0000 0xxx
                                                         // case statemate here might be faster still
                                        switch (data){
                                            case 0x0002:
                                                OPTION();
                                                break;
                                            case 0x0003:
                                                SLEEP();
                                                break;
                                            case 0x0004:
                                                CLRWDT();
                                                break;
                                            case 5:
                                                f = 5;
                                                TRIS();
                                                break;
                                            case 6:
                                                f = 6;
                                                TRIS();
                                                break;
                                            case 7:
                                                f = 6;
                                                TRIS();
                                                break;
                                            default:	// Nop or invaliod instruction
                                                break;
                                        }
                                    } else {				// 0000 000x xxxx
                                                // must be invalid instruction
                                    }
                                } else {				// 0000 001f ffff
                                    f =  data & 0x001f;
                                    MOVWF();
                                }
                            } else {				// 0000 01xx xxxx
                                if (data < 0x00C0){		// 0000 010x xxxx
                                                     // 0000 0100 0000 is valid anything else isn't, ie 0000 0101 0000
                                    if (data == 0x0040){
                                        CLRW();
                                    } else {
                                        // invalid instruction
                                    }
                                } else {				// 0000 011f ffff
                                    CLRF();
                                }
                            }
                        } else {				// 0000 1xdf ffff ? 0000 11xdf ffff
                            f =  data & 0x001f;
                            d = (data & 0x0020) >> 5;
                            if (data < 0x00C0){		// 0000 10df ffff
                                SUBWF();
                            } else {				// 0000 11df ffff
                                DECF();
                            }
                        }
                    } else {				// 0x01xx
                        f =  data & 0x001f;
                        d = (data & 0x0020) >> 5;
                        if (data < 0x0180){		// 0000 1xxx xxxx
                            if (data < 0x0140){		// 0001 00df ffff		// 16 instructions below and including this
                                IORWF();
                            } else {			// 0001 01df ffff
                                ANDWF();
                            }
                        } else {				// 0001 1xxx xxxx
                            if (data < 0x01C0){		// 0001 10df ffff
                                XORWF();
                            } else {				// 0001 11df ffff
                                ADDWF();
                            }
                        }
                    }
                } else {
                    f =  data & 0x001f;
                    d = (data & 0x0020) >> 5;
                    if (data < 0x0300){											// 0x02xx
                        if (data < 0x0280){		// 0010 0xdf ffff ? 0010 1xdf ffff
                            if (data < 0x0240){		// 0010 00df ffff
                                MOVF();
                            } else {				// 0010 01df ffff
                                COMF();
                            }
                        } else {				// 0010 1xxx xxxx
                            if (data < 0x02C0){		// 0010 10df ffff
                                INCF();
                            } else {				// 0010 11df ffff
                                DECFSZ();
                            }
                        }
                    } else {				// 0x03xx
                        if (data < 0x0380){		// 0011 0xdf ffff ? 0011 1xdf ffff
                            if (data < 0x0340){		// 0011 00df ffff
                                RRF();
                            } else {				// 0011 01df ffff
                                RLF();
                            }
                        } else {				// 0011 1xdf ffff
                            if (data < 0x03C0){		// 0011 10df ffff
                                SWAPF();
                            } else {				// 0011 11df ffff
                                INCFSZ();
                            }
                        }
                    }
                }
            } else{
                b = (data & 0x00e0) >> 5;
                f =  data & 0x001f;
                if (data < 0x0600){
                    if (data < 0x0500){		// 0x04xx  0100 bbbf ffff
                        BCF();
                    } else {				// 0x05xx   0101 bbbf ffff
                        BSF();
                    }
                } else {
                    if (data < 0x0700){		// 0x06xx   0000 0110 bbbf ffff
                        BTFSC();
                    } else {				// 0x07xx   0000 0111 bbbf ffff
                        BTFSS();
                    }
                }
            }
        }else{
            k = data & 0x00ff;			// all Literial comands so need literal data
            if (data < 0x0C00){
                if (data < 0x0A00){
                    if (data < 0x0900){		// 0000 1000 kkkk kkkk
                        RETLW();
                    } else {				// 0000 1001 kkkk kkkk
                        CALL();
                    }
                } else {					// 0000 101k kkkk kkkk
                    k = data & 0x01ff;
                    GOTO();
                }
            } else{
                if (data < 0x0E00){
                    if (data < 0x0D00){		// 0000 1100 kkkk kkkk
                        MOVLW();
                    } else {				// 0000 1101 kkkk kkkk
                        IORLW();
                    }
                } else {
                    if (data < 0x0F00){		// 0000 1110 kkkk kkkk
                        ANDLW();
                    } else {				// 0000 1111 kkkk kkkk
                        XORLW();
                    }
                }
            }
        }
    }
}

extern void Microseconds(UnsignedWide * microTickCount);

double myclock( void );

double myclock( void )
{
    long time;

    Microseconds( (UnsignedWide*) &time );
    return (double) time * 1e-6;
}


int main (int argc, const char * argv[])
{

    int dummyRom[512];
    unsigned int count;
    unsigned int exec;
    unsigned int execstp;
    long unsigned int time;
    long unsigned int time0;

    for (count=0;count<511;count++){
        dummyRom[count] = count;
    }
    Init(dummyRom);
    exec 	= 1;
    execstp 	= 100000000;

    printf( "Simulationing using method II for %f  Million Instructions\n" ,exec*execstp/1e6);
    GetDateTime( &time0 );

    for (count = 0;count<exec; count++){ Execute( execstp ); }

    GetDateTime( &time );
    printf( "%f seconds -> %f MIPs\n",(float)(time-time0), ((float)(exec*execstp))/((float) (time-time0))/1e6);

    return 0;
}

