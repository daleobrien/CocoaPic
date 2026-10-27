#include <stdio.h>
#include <sys/time.h>

//#include <CoreServices/CoreServices.h>
#include "PIC12F675_cpuMemory.h"
#include "PIC12F675_cpuCore.h"

#define numberOfcycles 10000000
struct timeval now ;
struct timeval then ;

#define floattime(x)	( x.tv_sec + ( x.tv_usec / 1000000.0 ) )

long microseconds;

int a;

int main (int argc, const char * argv[]) {

    unsigned int rom[4096];
    unsigned int i,j;
       

    for (j=0;j<4095;j++){
	rom[j] = 4095-j;
    }

   gettimeofday(&then, NULL);

  for (i=0;i<numberOfcycles;i++){

      _WDT++;
      _TMR++;
      _PC.a++;
     // _PC.a &= 0x3ff;
      
      CPU_14[ rom[ _PC.a & 0x3ff] ]();
  
  }

     gettimeofday(&now, NULL);
    // insert code here...
    printf ( "MIPs: %0.6f\n", numberOfcycles/(floattime(now)-floattime(then))/1000000.0 ) ;

   return 0;
}
