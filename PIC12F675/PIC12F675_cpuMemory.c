// Automatically generated on Sunday, 2 November 2003
#include "Headers/PIC12F675_cpuMemory.h"
#include "Headers/PIC12F675_cpuCore.h"



union p8  _PC;

int   ROM [ROM_no_of_bytes];			// ROM space
int   breakPoints[ROM_no_of_bytes];		// halting points

union u8 f[128][2];

unsigned char _W;

unsigned int _Stack[7];
unsigned int _StackPointer;

unsigned char _TMR;
unsigned char _WDT;

// IS this needed ?
unsigned char WDT_prescaler;

int   ROM [ROM_no_of_bytes];			// ROM space
//int      f[RAM_no_of_bytes];			// RAM space, contains ports etc
int   breakPoints[ROM_no_of_bytes];		// halting points


char g[15];
char* getStringOfInstruction(int idx){
    int i,j;
    j =  ROM[idx];
    
    for (i=0;i<15;i++) {
        g[i] = CPU_14s[j][i];
    }
    return( (char*) g );
}

// not tested at all ...
char* getStringOfInstructionII(int idx){

    int i;

    for (i=0;i<15;i++) {
        g[i] = CPU_14s[idx][i];
    }

    return( (char*) g );
}