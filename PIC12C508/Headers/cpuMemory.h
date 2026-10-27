
extern long numberOfCyclesTaken;

extern const int clearbit[8];
extern const int   setbit[8];


// CPU Memory
#define ROM_no_of_bytes 1024
#define RAM_no_of_bytes 64

extern int   ROM [ROM_no_of_bytes];			// ROM space
extern int      f[RAM_no_of_bytes];			// RAM space, contains ports etc
//extern char fn[RAM_no_of_bytes][9];
                               // Debug stuff
extern int   breakPoints[ROM_no_of_bytes];

// CPU registors

#define INDR_ADDR	f[ f[4]&(ROM_no_of_bytes-1)]		// Not physically impemented
#define RTCC		f[ 1] 	//  8 bit
#define PC_Pointer	f[ 2]	// lower 8 bits of PC
#define STATUS		f[ 3]
#define	FSR		f[ 4]
#define PORTA		f[ 5]
#define PORTB		f[ 6]
#define PORTC		f[ 7]

// Bit locations of STATUS
#define PA2 	7	// 	general purpose read/write bit
#define PA1	6	//	general purpose 	/ page select on some CPUs
#define PA0	5	//	general purpose 	/ page select on some CPUs
#define TO	4	// 	time out bit		/ set to 1 on powerup,CLRWDT and SLEEP, cleared by WDT out
#define PD	3	// 	Powerdown bit		/ set to 1 on powerup,CLRWDT, clear by SLEEP, WDT has no effect
#define Z	2	// 	Zero bit			/ set to zero if ALU or Logic is zero
#define DC	1	//	Borrow			/
#define C	0	// 	Carry bit			/

extern int STACK1;
extern int STACK2;
extern int temp;
extern int _W;
extern int _TRISA;		// 4 bit	x bit of porta (f5) is an output if 0 else an output if 1
extern int _TRISB;		// 8 bit
extern int _TRISC;		// 8 bit
extern int _OPTION;		// 6 bit
extern int WDT_timer;
extern int WDT_prescaler;

// Bit locations of OPTION
#define RTS	5 //  RTCC source is internal instruction cycle clock (CLKOUT) if cleared to 0, Transition on RTCC pin if set to 1
#define RTE	4 //  RTCC signal edge, low to high transistion if cleared to 0, high to low transistion if set to 1
#define PSA	3 // Prescaler assignment, 0 RTCC, 1 WDT
#define PS2	2 //
#define PS1	1 //
#define PS0	0 // Prescaler scale


// Prescaler
// PSx	     		RTCC rate	     WDT rate
// 0 0 0		  2			  1
// 0 0 1		  4			  2
// 0 1 0		  8		 	  4
// 0 1 1		 16			  8
// 1 0 0 		 32			 16
// 1 0 1 		 64			 32
// 1 1 0		128			 64
// 1 1 1 		256			128
             