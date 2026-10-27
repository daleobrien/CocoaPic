#include "cpuMemory.h"

const int clearbit[8] = { 0x00FE , 0x00FD , 0x00FB , 0x00F7 , 0x00EF , 0x00DF , 0x00BF , 0x007F };
const int   setbit[8] = { 0x0001 , 0x0002 , 0x0004 , 0x0008 , 0x0010 , 0x0020 , 0x0040 , 0x0080 };

// CPU Memory


int   ROM [ROM_no_of_bytes];			// ROM space
int      f[RAM_no_of_bytes];			// RAM space, contains ports etc


int   breakPoints[ROM_no_of_bytes];		// halting points



// Decoded instrustion data
int d;
int k;
int b;

int STACK1;
int STACK2;
int temp;
int _W;

int _OPTION;		// 6 bit
int WDT_timer;
int WDT_prescaler;






