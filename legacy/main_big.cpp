
#include <iostream>

#include <CoreServices/CoreServices.h>

#include "cpuCore.h"
#include "cpuMemory.h"



void Init(int prog[512]){
    unsigned int j;

    // ROM gets set to this for now
    for (j=0;j<511;j++){
        ROM[j] = prog[j];
    }
    // RAM reset values
    PC_Pointer = 0x00ff;
    STATUS 	= STATUS & 0x0018;
    _W 	 	= _W;
    _OPTION = _OPTION & 0x003f;
    FSR    	= FSR 	& 0x0080;
    PC_Pointer = 0;			// should really point to above
}




extern void Microseconds(UnsignedWide * microTickCount);

double myclock( void );

double myclock( void )
{
    long time;

    Microseconds( (UnsignedWide*) &time );
    return (double) time * 1e-6;
}


void ExecuteII(unsigned int no_of_CPU_cycles){
    unsigned int i;

  for(i=0; i< no_of_CPU_cycles;i++){

        if( PC_Pointer >= ROM_no_of_bytes){  PC_Pointer = 0;}else{PC_Pointer++;}

        CPU[ ROM[ PC_Pointer]&0xfff ]();
    }

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
    exec 	= 1000000; 
    execstp 	= 1000;
    
    
    Init(dummyRom);
      
    printf( "begin Simulation method 2, %d\n" ,exec*execstp);
    GetDateTime( &time0 );

    for (count = 0;count<exec; count++){ ExecuteII( execstp );}

    GetDateTime( &time );
    printf( "%f seconds ,%f MIPs\n",(float)(time-time0), ((float)(exec*execstp))/((float) (time-time0))/1e6);

    return 0;
}


