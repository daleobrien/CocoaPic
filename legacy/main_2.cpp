
#include <iostream>

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
#define PC_Pointer	file[2]		// 11 bit
#define STATUS		file[3]
#define	FSR		file[4]
#define PORTA		file[5]
#define PORTB		file[6]
#define PORTC		file[7]

// Bit locations of STATUS
#define PA2 	7	// 	general purpose 	/ read/write bit
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



//void ( *gn[2])(void) ={  &ANDWF , &CALL };

// Reset the Mirco
// All instructions will assume that file[f] or what have you are of the correct length
void Init(int prog[512]){
    unsigned int j;
    // ROM gets set to this for now
    for (j=0;j<511;j++){ ROM[j] = prog[j];}

    // RAM reset values
    PC_Pointer = 0x00ff;
    STATUS 	= STATUS & 0x0018;
    _W 	 	= _W;
    _OPTION 	= _OPTION & 0x003f;
    FSR    	=  FSR 	  & 0x0080;
    PC_Pointer  = 0;			// should really point to above
}



void ADDWFd0(){
    int f;
    f =   ROM[ PC_Pointer ] & 0x001f;

    _W 	= (_W + file[f]) & 0x00ff;
    if(_W == 0) STATUS |= setbit[Z]; else STATUS &= clearbit[Z];

}

void ADDWFd1(){
    int f;

    f =   ROM[ PC_Pointer ] & 0x001f;

    file[f] =  (_W + file[f]) & 0x00ff;
    if(file[f]==0) STATUS |= setbit[Z]; else STATUS &= clearbit[Z];
}

void ANDLW(){
    _W = _W & ROM[ PC_Pointer ]; // and not needed
    if(_W!=0) 	STATUS |=  setbit[Z]; else STATUS &= clearbit[Z];		// Z bit
}

void CLRF(){
    int f;
    f =   ROM[ PC_Pointer ] & 0x001f;
    file[f] = 0;
    STATUS |=  setbit[Z];
}

void CLRW(){
    _W = 0;
    STATUS |= setbit[Z];
}

void COMFd0(){
    int f;

    f =   ROM[ PC_Pointer ] & 0x001f;

    _W = 0x00ff & (!file[f]);
    if(_W!=0) STATUS |= setbit[Z]; else STATUS &= clearbit[Z];		// Z bit
}


void COMFd1(){
    int f;

    f =   ROM[ PC_Pointer ] & 0x001f;

    file[f] = 0x00ff & (!file[f]);
    if(file[f]!= 0) STATUS |= setbit[Z]; else STATUS &= clearbit[Z];		// Z bit

}
void DECFd0(){			// Decrement F and store to either f or W,
    int f;
    f =   ROM[ PC_Pointer ] & 0x001f;
    _W = (file[f]-1)&0xff;

    if( _W != 0){
        STATUS &= clearbit[Z];		// Z bit clear, non zero result
    } else  {
        STATUS |= setbit[Z];		// Z bit set, zero result
    }
}

void DECFd1(){			// Decrement F and store to either f or W,

    int f;
    f =   ROM[ PC_Pointer ] & 0x001f;

    file[f] = file[f]-1;

    if( file[f]!=0){
        STATUS &= clearbit[Z];		// Z bit clear, non zero result
    } else {
        STATUS |= setbit[Z];		// Z bit set, zero result
    }
}


void DECFSZd0(){		// Decrement F and store to either f or W, if result is zero skip the next instruction

    int f;

    f =   ROM[ PC_Pointer ] & 0x001f;
    _W = (file[f]-1)&0xff;

    if( file[f]!=0){
        STATUS = STATUS & clearbit[Z];		// Z bit clear, non zero result
    } else {
        STATUS = STATUS | setbit[Z];		// Z bit set, zero result
        PC_Pointer = PC_Pointer+1;
    }
}

void DECFSZd1(){		// Decrement F and store to either f or W, if result is zero skip the next instruction

    int f;

    f =   ROM[ PC_Pointer ] & 0x001f;

    file[f] = (file[f] - 1) & 0xff;

    if( file[f]!=0){
        STATUS = STATUS & clearbit[Z];		// Z bit clear, non zero result
    } else
    {
        STATUS = STATUS | setbit[Z];		// Z bit set, zero result
        PC_Pointer = PC_Pointer+1;
    }
}

void  INCFd0(){			// Increment F and store to either f or W
    int f;

    f  =   ROM[ PC_Pointer ] & 0x001f;
    _W = (file[f]+1)&0xff;
    if( _W==0){
        STATUS |= setbit[Z];	// Z bit set,  zero result
    } else {
        STATUS &= clearbit[Z];	// Z bit clear, non zero result
    }
}

void  INCFd1(){			// Increment F and store to either f or W
    int f;

    f =   ROM[ PC_Pointer ] & 0x001f;
    file[f]=(file[f]+1)&0xff;

    if( file[f]==0){
        STATUS |= setbit[Z];	// Z bit set,  zero result
    } else {
        STATUS &= clearbit[Z];	// Z bit clear, non zero result
    }
}

void INCFSZd0(){		// Increment F and store to either f or W, if result is zero skip the next instruction
    int f;

    f =   ROM[ PC_Pointer ] & 0x001f;
    _W = (file[f]+1)&0xff;

    if( _W){
        STATUS = STATUS | setbit[Z];
        PC_Pointer = PC_Pointer+1;
    } else {
        STATUS = STATUS & clearbit[Z];
    }
}
void INCFSZd1(){		// Increment F and store to either f or W, if result is zero skip the next instruction
    int f;

    f =   ROM[ PC_Pointer ] & 0x001f;
    file[f] = (file[f]+1) & 0xff;

    if( file[f]==0){
        STATUS = STATUS | setbit[Z];
        PC_Pointer = PC_Pointer+1;
    } else {
        STATUS = STATUS & clearbit[Z];
    }
}

void IORWFd0(){			// OR
    int f;

    f =   ROM[ PC_Pointer ] & 0x001f;

    _W = file[f] | _W;
    if (_W==0) STATUS = STATUS | setbit[Z]; else STATUS &= clearbit[Z];		// Z bit

}
void IORWFd1(){			// OR
    int f;

    f =   ROM[ PC_Pointer ] & 0x001f;

    file[f] = file[f] | _W;
    if (file[f]==0) STATUS |= setbit[Z]; else STATUS &= clearbit[Z];		// Z bit

}

void MOVFd0(){		// Move F to (dest)
    int f;
    f =   ROM[ PC_Pointer ] & 0x001f;
    _W = file[f];
    if (_W==0) 	STATUS |= setbit[Z]; else STATUS &= clearbit[Z];		// Z bit

}
void MOVFd1(){		// Move F to (dest)
    int f;
    f =   ROM[ PC_Pointer ] & 0x001f;
    if (file[f]==0) STATUS |= setbit[Z]; else STATUS &= clearbit[Z];		// Z bit
}

void MOVWF(){			// Literal data is moved to W
    k = _W;
}

void NOP(){			// Do nothing, not really needed

}

void RLFd0(){			// Rotate Left through Carry bit <C>
    int f;
    f =   ROM[ PC_Pointer ] & 0x001f;
    if (file[f] & 0x0080)	STATUS |= setbit[C]; else STATUS &= clearbit[C];
    _W      = (file[f] << 1) & 0x00ff;
}

void RLFd1(){			// Rotate Left through Carry bit <C>
    int f;
    f =   ROM[ PC_Pointer ] & 0x001f;

    if (file[f] & 0x80) STATUS |= setbit[C]; else STATUS &= clearbit[C];
    file[f] = (file[f] << 1) & 0x00ff;// GET last carry bit too !!!
}

void RRFd0(){			// Rotate right through Carry bit <C>
    int f;
    f =   ROM[ PC_Pointer ] & 0x001f;

    if (file[f] & 0x0001)	STATUS |= setbit[C]; else 	STATUS &= clearbit[C];
    _W     = (file[f] >> 1) ;// GET last carry bit too !!!
}

void RRFd1(){			// Rotate right through Carry bit <C>
    int f;
    f =   ROM[ PC_Pointer ] & 0x001f;

    if (file[f] & 0x0001) STATUS |= setbit[C]; else STATUS  &= clearbit[C];
    file[f] >>=  1;
}

void SUBWFd0(){			// Subtract        f-W -> (dest)
    int f;
    f =   ROM[ PC_Pointer ] & 0x001f;

    if (file[f] == _W)	STATUS = STATUS | setbit[Z] ; else STATUS = STATUS & clearbit[Z];	// Zero Result bit
    if (file[f] >= _W)	STATUS = STATUS | setbit[C]|setbit[DC] ; else STATUS = STATUS & clearbit[C] & clearbit[DC]; // Carry bit & Borrow

    _W	= (file[f] - _W) & 0x00ff;

}

void SUBWFd1(){			// Subtract        f-W -> (dest)
    int f;
    f =   ROM[ PC_Pointer ] & 0x001f;

    if (file[f] == _W)	STATUS |= setbit[Z] ; else STATUS &= clearbit[Z];	// Zero Result bit
    if (file[f] >= _W)	STATUS = STATUS | setbit[C]|setbit[DC] ; else STATUS = STATUS & clearbit[C] & clearbit[DC]; // Carry bit & Borrow

    file[f]	= (file[f] - _W) & 0x00ff;
}


void SWAPFd0(){			// Swap nibbles,  ie before f=0xA5  the after (dest) =0x5A
    int f;
    f =   ROM[ PC_Pointer ] & 0x001f;
    _W = (file[f]>>4) | (file[f]<<4);

}
void SWAPFd1(){			// Swap nibbles,  ie before f=0xA5  the after (dest) =0x5A
    int f;
    f =   ROM[ PC_Pointer ] & 0x001f;
    file[f] = (file[f]>>4) | (file[f]<<4);
}

void XORWFd0(){

    _W =  (_W ^ file[ ROM[ PC_Pointer ] & 0x001f]) & 0xff;
    if ( _W == 0 ) STATUS &= clearbit[Z]; else STATUS |= setbit[Z]; // Z bit
}

void XORWFd1(){
    file[ ROM[ PC_Pointer ] & 0x001f] =  (_W ^ file[ ROM[ PC_Pointer ] & 0x001f]) & 0xff;
    if (  file[ ROM[ PC_Pointer ] & 0x001f] == 0 ) STATUS &= clearbit[Z]; else STATUS |= setbit[Z]; // Z bit

}

void BCFb0f0(){    file[( ROM[ PC_Pointer ] & 0x001f)] &= 0xFE;}
void BCFb1f0(){    file[( ROM[ PC_Pointer ] & 0x001f)] &= 0xFD;}
void BCFb2f0(){    file[( ROM[ PC_Pointer ] & 0x001f)] &= 0xFB;}
void BCFb3f0(){    file[( ROM[ PC_Pointer ] & 0x001f)] &= 0xF7;}
void BCFb4f0(){    file[( ROM[ PC_Pointer ] & 0x001f)] &= 0xEF;}
void BCFb5f0(){    file[( ROM[ PC_Pointer ] & 0x001f)] &= 0xDF;}
void BCFb6f0(){    file[( ROM[ PC_Pointer ] & 0x001f)] &= 0xBF;}
void BCFb7f0(){    file[( ROM[ PC_Pointer ] & 0x001f)] &= 0x7F;}
void BCFb0f1(){    file[( ROM[ PC_Pointer ] & 0x001f)] &= 0xFE;}
void BCFb1f1(){    file[( ROM[ PC_Pointer ] & 0x001f)] &= 0xFD;}
void BCFb2f1(){    file[( ROM[ PC_Pointer ] & 0x001f)] &= 0xFB;}
void BCFb3f1(){    file[( ROM[ PC_Pointer ] & 0x001f)] &= 0xF7;}
void BCFb4f1(){    file[( ROM[ PC_Pointer ] & 0x001f)] &= 0xEF;}
void BCFb5f1(){    file[( ROM[ PC_Pointer ] & 0x001f)] &= 0xDF;}
void BCFb6f1(){    file[( ROM[ PC_Pointer ] & 0x001f)] &= 0xBF;}
void BCFb7f1(){    file[( ROM[ PC_Pointer ] & 0x001f)] &= 0x7F;}

void BSFb0f0(){    file[( ROM[ PC_Pointer ] & 0x001f)] |= 0x01; }
void BSFb1f0(){    file[( ROM[ PC_Pointer ] & 0x001f)] |= 0x02; }
void BSFb2f0(){    file[( ROM[ PC_Pointer ] & 0x001f)] |= 0x04; }
void BSFb3f0(){    file[( ROM[ PC_Pointer ] & 0x001f)] |= 0x08; }
void BSFb4f0(){    file[( ROM[ PC_Pointer ] & 0x001f)] |= 0x10; }
void BSFb5f0(){    file[( ROM[ PC_Pointer ] & 0x001f)] |= 0x20; }
void BSFb6f0(){    file[( ROM[ PC_Pointer ] & 0x001f)] |= 0x40; }
void BSFb7f0(){    file[( ROM[ PC_Pointer ] & 0x001f)] |= 0x80; }
void BSFb0f1(){    file[( ROM[ PC_Pointer ] & 0x001f)] |= 0x01; }
void BSFb1f1(){    file[( ROM[ PC_Pointer ] & 0x001f)] |= 0x02; }
void BSFb2f1(){    file[( ROM[ PC_Pointer ] & 0x001f)] |= 0x04; }
void BSFb3f1(){    file[( ROM[ PC_Pointer ] & 0x001f)] |= 0x08; }
void BSFb4f1(){    file[( ROM[ PC_Pointer ] & 0x001f)] |= 0x10; }
void BSFb5f1(){    file[( ROM[ PC_Pointer ] & 0x001f)] |= 0x20; }
void BSFb6f1(){    file[( ROM[ PC_Pointer ] & 0x001f)] |= 0x40; }
void BSFb7f1(){    file[( ROM[ PC_Pointer ] & 0x001f)] |= 0x80; }

void BTFSCb0f0(){ if ( file[( ROM[ PC_Pointer ] & 0x001f)] ^ 0x01  ) PC_Pointer++;}
void BTFSCb1f0(){ if ( file[( ROM[ PC_Pointer ] & 0x001f)] ^ 0x02  ) PC_Pointer++;}
void BTFSCb2f0(){ if ( file[( ROM[ PC_Pointer ] & 0x001f)] ^ 0x04  ) PC_Pointer++;}
void BTFSCb3f0(){ if ( file[( ROM[ PC_Pointer ] & 0x001f)] ^ 0x08  ) PC_Pointer++;}
void BTFSCb4f0(){ if ( file[( ROM[ PC_Pointer ] & 0x001f)] ^ 0x10  ) PC_Pointer++;}
void BTFSCb5f0(){ if ( file[( ROM[ PC_Pointer ] & 0x001f)] ^ 0x20  ) PC_Pointer++;}
void BTFSCb6f0(){ if ( file[( ROM[ PC_Pointer ] & 0x001f)] ^ 0x40  ) PC_Pointer++;}
void BTFSCb7f0(){ if ( file[( ROM[ PC_Pointer ] & 0x001f)] ^ 0x80  ) PC_Pointer++;}
void BTFSCb0f1(){ if ( file[( ROM[ PC_Pointer ] & 0x001f)] ^ 0x01  ) PC_Pointer++;}
void BTFSCb1f1(){ if ( file[( ROM[ PC_Pointer ] & 0x001f)] ^ 0x02  ) PC_Pointer++;}
void BTFSCb2f1(){ if ( file[( ROM[ PC_Pointer ] & 0x001f)] ^ 0x04  ) PC_Pointer++;}
void BTFSCb3f1(){ if ( file[( ROM[ PC_Pointer ] & 0x001f)] ^ 0x08  ) PC_Pointer++;}
void BTFSCb4f1(){ if ( file[( ROM[ PC_Pointer ] & 0x001f)] ^ 0x10  ) PC_Pointer++;}
void BTFSCb5f1(){ if ( file[( ROM[ PC_Pointer ] & 0x001f)] ^ 0x20  ) PC_Pointer++;}
void BTFSCb6f1(){ if ( file[( ROM[ PC_Pointer ] & 0x001f)] ^ 0x40  ) PC_Pointer++;}
void BTFSCb7f1(){ if ( file[( ROM[ PC_Pointer ] & 0x001f)] ^ 0x80  ) PC_Pointer++;}


void BTFSSb0f0(){ if ( file[( ROM[ PC_Pointer ] & 0x001f)] | 0x01  ) PC_Pointer++;}
void BTFSSb1f0(){ if ( file[( ROM[ PC_Pointer ] & 0x001f)] | 0x02  ) PC_Pointer++;}
void BTFSSb2f0(){ if ( file[( ROM[ PC_Pointer ] & 0x001f)] | 0x04  ) PC_Pointer++;}
void BTFSSb3f0(){ if ( file[( ROM[ PC_Pointer ] & 0x001f)] | 0x08  ) PC_Pointer++;}
void BTFSSb4f0(){ if ( file[( ROM[ PC_Pointer ] & 0x001f)] | 0x10  ) PC_Pointer++;}
void BTFSSb5f0(){ if ( file[( ROM[ PC_Pointer ] & 0x001f)] | 0x20  ) PC_Pointer++;}
void BTFSSb6f0(){ if ( file[( ROM[ PC_Pointer ] & 0x001f)] | 0x40  ) PC_Pointer++;}
void BTFSSb7f0(){ if ( file[( ROM[ PC_Pointer ] & 0x001f)] | 0x80  ) PC_Pointer++;}
void BTFSSb0f1(){ if ( file[( ROM[ PC_Pointer ] & 0x001f)] | 0x01  ) PC_Pointer++;}
void BTFSSb1f1(){ if ( file[( ROM[ PC_Pointer ] & 0x001f)] | 0x02  ) PC_Pointer++;}
void BTFSSb2f1(){ if ( file[( ROM[ PC_Pointer ] & 0x001f)] | 0x04  ) PC_Pointer++;}
void BTFSSb3f1(){ if ( file[( ROM[ PC_Pointer ] & 0x001f)] | 0x08  ) PC_Pointer++;}
void BTFSSb4f1(){ if ( file[( ROM[ PC_Pointer ] & 0x001f)] | 0x10  ) PC_Pointer++;}
void BTFSSb5f1(){ if ( file[( ROM[ PC_Pointer ] & 0x001f)] | 0x20  ) PC_Pointer++;}
void BTFSSb6f1(){ if ( file[( ROM[ PC_Pointer ] & 0x001f)] | 0x40  ) PC_Pointer++;}
void BTFSSb7f1(){ if ( file[( ROM[ PC_Pointer ] & 0x001f)] | 0x80  ) PC_Pointer++;}

void ANDWFd0(){
    int f;

    f =   ROM[ PC_Pointer ] & 0x001f;

    if ( (_W & file[f]) == 0 )	STATUS &=  clearbit[Z]; else STATUS |= setbit[Z];		// Z bit
    _W = _W & file[f];
}

void ANDWFd1(){
    int f;
    f =   ROM[ PC_Pointer ] & 0x001f;

    if ( (_W & file[f]) == 0 )	STATUS &= clearbit[Z]; else STATUS |=  setbit[Z];		// Z bit
    file[f] = _W & file[f];	// Z bit
}

void CALL(){						// Call Subroutine
                  //  k =  zdata & 0x01ff;
    STACK2 = STACK1;
    STACK1 = PC_Pointer + 1;
    PC_Pointer = ( ROM[ PC_Pointer ] & 0x01ff) | (( STATUS<<4)&0x0600);
    STATUS = STATUS | clearbit[PA2];
}

void CLRWDT(){

    WDT_timer     = 0;
    WDT_prescaler = 0; // must check something here first
    STATUS = STATUS & setbit[TO] & setbit[PD];
}

void GOTO(){
    PC_Pointer = ( ROM[ PC_Pointer ] & 0x01ff) | (( STATUS<<4)&0x0600); // pick up bits PA0,PA1
}

void IORLW(){					// w OR Literial
    _W =(_W |  ROM[ PC_Pointer ]) & 0x01ff;
    if ( _W == 0 ) STATUS  &= setbit[Z]; else STATUS |= clearbit[Z];		// Z bit
}

void MOVLW(){					// Move Literial to w
    _W =  ROM[ PC_Pointer ] & 0x01ff;
}

void OPTION(){			// Load the Option regisitor
    _OPTION = _W & 0x003f;	//  6 bits only
}

void RETLW(){					// Return from Subroutine Call

    _W =  ROM[ PC_Pointer ] & 0x01ff;  	// k & 0x00ff;
    PC_Pointer = STACK1;
    STACK1     = STACK2;	// STACK is unchanged
}



void XORLW(){
    //   k =  zdata & 0x01ff;
    _W = (_W^ ROM[ PC_Pointer ])& 0x01ff ;// W^k
    if ( _W == 0 )	STATUS = STATUS & setbit[Z]; else STATUS = STATUS | clearbit[Z];		// Z bit
}



void INT000(void){

    switch (ROM[ PC_Pointer ]){
        case 2:
            _OPTION = _W & 0x003f;	//  6 bits only
            break;
        case 3:
            //SLEEP();
            break;
        case 4:
            WDT_timer     = 0;
            WDT_prescaler = 0; // must check something here first
            STATUS = STATUS | setbit[TO] | setbit[PD];
            //CLRWDT();
            break;
        case 5:
            _TRISA = _W & 0x000f;
            break;
        case 6:
            _TRISB = _W & 0x00ff;
            break;
        case 7:
            _TRISC = _W & 0x00ff;
            break;
        default:	// Nop or invaliod instruction
            break;
    }
}
void x(void){ }


void (*const CPU[])(void) = {
    INT000	,x	,MOVWF	,MOVWF	,CLRW	,x	,CLRF	,CLRF	,SUBWFd0,SUBWFd0,SUBWFd1,SUBWFd1,DECFd0	,DECFd0	,DECFd1	,DECFd1	,
    IORWFd0	,IORWFd0,IORWFd1,IORWFd1,ANDWFd0,ANDWFd0,ANDWFd1,ANDWFd1,XORWFd0,XORWFd0,XORWFd1,XORWFd1,ADDWFd0,ADDWFd0,ADDWFd1,ADDWFd1,
    MOVFd0	,MOVFd0	,MOVFd1	,MOVFd1	,COMFd0	,COMFd0	,COMFd1	,COMFd1	,INCFd0	,INCFd0	,INCFd1	,INCFd1	,DECFSZd0,DECFSZd0,DECFSZd0,DECFSZd0,
    RRFd0	,RRFd0	,RRFd1	,RRFd1	,RLFd0	,RLFd0	,RLFd1	,RLFd1	,SWAPFd0,SWAPFd0,SWAPFd1,SWAPFd1,INCFSZd0,INCFSZd0,INCFSZd1,INCFSZd1,
    BCFb0f0	,BCFb1f0,BCFb2f0,BCFb3f0,BCFb4f0,BCFb5f0,BCFb6f0,BCFb7f0,BCFb0f1,BCFb1f1,BCFb2f1,BCFb3f1,BCFb4f1,BCFb5f1,BCFb6f1,BCFb7f1,
    BSFb0f0	,BSFb1f0,BSFb2f0,BSFb3f0,BSFb4f0,BSFb5f0,BSFb6f0,BSFb7f0,BSFb0f1,BSFb1f1,BSFb2f1,BSFb3f1,BSFb4f1,BSFb5f1,BSFb6f1,BSFb7f1,
    BTFSCb0f0	,BTFSCb1f0,BTFSCb2f0,BTFSCb3f0,BTFSCb4f0,BTFSCb5f0,BTFSCb6f0,BTFSCb7f0,
    BTFSCb0f1	,BTFSCb1f1,BTFSCb2f1,BTFSCb3f1,BTFSCb4f1,BTFSCb5f1,BTFSCb6f1,BTFSCb7f1,
    BTFSSb0f0	,BTFSSb1f0,BTFSSb2f0,BTFSSb3f0,BTFSSb4f0,BTFSSb5f0,BTFSSb6f0,BTFSSb7f0,
    BTFSSb0f1	,BTFSSb1f1,BTFSSb2f1,BTFSSb3f1,BTFSSb4f1,BTFSSb5f1,BTFSSb6f1,BTFSSb7f1,
    RETLW	,RETLW	,RETLW	,RETLW	,RETLW	,RETLW	,RETLW	,RETLW ,RETLW	,RETLW	,RETLW	,RETLW	,RETLW	,RETLW	,RETLW	,RETLW,
    CALL	,CALL	,CALL	,CALL	,CALL	,CALL	,CALL	,CALL	,CALL	,CALL	,CALL	,CALL	,CALL	,CALL	,CALL	,CALL	,
    GOTO	,GOTO	,GOTO	,GOTO	,GOTO	,GOTO	,GOTO	,GOTO	,GOTO	,GOTO	,GOTO	,GOTO	,GOTO	,GOTO	,GOTO	,GOTO	,
    GOTO	,GOTO	,GOTO	,GOTO	,GOTO	,GOTO	,GOTO	,GOTO	,GOTO	,GOTO	,GOTO	,GOTO	,GOTO	,GOTO	,GOTO	,GOTO	,
    MOVLW	,MOVLW	,MOVLW	,MOVLW	,MOVLW	,MOVLW	,MOVLW	,MOVLW	,MOVLW	,MOVLW	,MOVLW	,MOVLW	,MOVLW	,MOVLW	,MOVLW	,MOVLW,
    IORLW	,IORLW	,IORLW	,IORLW	,IORLW	,IORLW	,IORLW	,IORLW	,IORLW	,IORLW	,IORLW	,IORLW	,IORLW	,IORLW	,IORLW	,IORLW,
    ANDLW	,ANDLW	,ANDLW	,ANDLW	,ANDLW	,ANDLW	,ANDLW	,ANDLW	,ANDLW	,ANDLW	,ANDLW	,ANDLW	,ANDLW	,ANDLW	,ANDLW	,ANDLW,
    XORLW	,XORLW	,XORLW	,XORLW	,XORLW	,XORLW	,XORLW	,XORLW	,XORLW	,XORLW	,XORLW	,XORLW	,XORLW	,XORLW	,XORLW	,XORLW  };

//extern Microseconds( (UnsignedWide*) &time );
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

    for (count=1;count<512;count++){
        dummyRom[count] = count;
    }
    exec 	= 10000;
    execstp 	= 10000;

    printf( "Simulationing using method II for %f  Million Instructions\n" ,exec*execstp/1e6);
    GetDateTime( &time0 );

    for (count = 0;count<exec; count++){ ExecuteII( execstp ); }

    GetDateTime( &time );
    printf( "%f seconds -> %f MIPs\n",(float)(time-time0), ((float)(exec*execstp))/((float) (time-time0))/1e6);

    return 0;
}


