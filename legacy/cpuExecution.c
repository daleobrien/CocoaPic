#include <CoreServices/CoreServices.h>

#include "PIC12F675/Headers/PIC12F675_cpuCore.h"
#include "PIC12F675/Headers/PIC12F675_cpuMemory.h"

long numberOfCyclesTaken;

extern int ROM[ROM_no_of_bytes];
extern int breakPoints[ROM_no_of_bytes];

extern void func(void);

void Init(){
    unsigned int j;

    // ROM gets set to this for now

    for (j=0;j<(ROM_no_of_bytes);j++){ ROM[j]  =  rand() % 4096;    }

    ROM[0]= 5118;
    // RAM reset values
     _PC.a  = ROM_no_of_bytes-1;
    _WDT   =  0;
//    RTCC 	=  0;
//    STATUS 	=  0;
    _W 	 	= 10;
 //   _OPTION 	=  0;	//
 //   FSR    	=  0;	//
    
    WDT_prescaler = 0x80;
}



//255*1*10^(-6)
volatile unsigned int c=0;

int Execute(  unsigned int no_of_CPU_cycles  ){
    unsigned int i;

    numberOfCyclesTaken  += no_of_CPU_cycles;
    
    for(i=0; i< no_of_CPU_cycles;i++){

	CPU_14[ ROM[ _PC.a ] ](); // execute the function pointed to by the ROM instruction
	
	 _PC.a++;
	_WDT++;
//	RTCC++;

	_PC.a &= (ROM_no_of_bytes-1);
	_WDT  &= 0xff;

	if (_WDT & WDT_prescaler ) {

	    _PC.a = ROM_no_of_bytes - 1;
	    _WDT  = 0;
	//    RTCC       = 0;
	}

	if( breakPoints[_PC.a] != 0 ) {
	    return(i);  
	} 
    }
    
    return(0);
}

void setValueOfRam(unsigned int idx, unsigned int data){
   if(idx < RAM_no_of_bytes)
       f[idx][0].a  = data;
}

unsigned int  getValueOfRam(unsigned int idx){
    if(idx<RAM_no_of_bytes){
        return( f[idx][0].a &0xff );
    }else{
        return( 0 );  
    }
    
}



unsigned int  getValueOfReg(unsigned int idx){
    switch (idx){
        case 0:
            return(_Stack[0]& 0x1ff);
            break;
        case 1:
            return(_Stack[1]& 0x1ff); // THERE ARE MORE ...
            break;
        case 2:
            return(_W & 0xff);
            break;
        case 3:
            return(_PC.a & 0x1ff);
            break;
        case 4:
            return( _WDT );
            break;

        case 5:
 //           return(_OPTION);
	    return(0);
            break;
        default:
            return(0);
            break;
    }

}

char*  getNameOfReg(unsigned int idx){
    switch (idx){
        case 0: return("Stack 1"); 	break;
        case 1: return("Stack 2"); 	break;
        case 2: return("W"); 		break;
        case 3: return("PC Pointer"); 	break;
        case 4: return("WDT Timer"); 	break;
        case 5: return("OPTION"); 	break;
        default:return(""); 		break;
    }
}

char zoo[] =  "0x0000000" ;

char* DecToHex(unsigned int no, unsigned int noOfPlaces){
    unsigned int i,j;

    for( j=0 ;j<(noOfPlaces);j++)
    {
        i =  (no>>(4*(noOfPlaces-j-1) )) & 0x0f;

        if (i<10) zoo[2+j] = (char) ( i + 48); // 48 = "0"
        else      zoo[2+j] = (char) ( i + 55); // 97 = "a", 65 = "A"

    }
    zoo[2+noOfPlaces] = (char) 0;

    return( zoo );
}