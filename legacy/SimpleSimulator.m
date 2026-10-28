#import "SimpleSimulator.h"
#import "romCode.h"

#include "PIC12F675/Headers/PIC12F675_cpuCore.h"
#include "PIC12F675/Headers/PIC12F675_cpuMemory.h"

#include "USB Pickit Interface.h"

#define numberOfStepsPerLoop 10000000
#define numberOfLoops        10

extern void Init(void);
extern int Execute(unsigned int no_of_CPU_cycles);

unsigned int p;//,lp;

@implementation SimpleSimulator

- (id) init{
    
    self = [super init];
    if (self) {
         [[NSNotificationCenter defaultCenter] addObserver:self selector:@selector(upDateStatus:)  name:@"status" object:nil ];
     }
    return self ;
}    

-(void)dealloc{ 
     
    [[NSNotificationCenter defaultCenter] removeObserver:self];    
    
    [super dealloc];
}


- (IBAction)ButtonGo:(id)sender
{

    Init(); //
    
    p = _PC.a & (ROM_no_of_bytes-1);
    if( p < 13){
        [ROMCode scrollRowToVisible: 26  ];
        [ROMCode scrollRowToVisible: 0  ];
    } else{
        if( p > (ROM_no_of_bytes-1)-13){
            [ROMCode scrollRowToVisible: (ROM_no_of_bytes-1)-26  ];
            [ROMCode scrollRowToVisible: (ROM_no_of_bytes-1)     ];
        } else{
            [ROMCode scrollRowToVisible: p+13 ];
            [ROMCode scrollRowToVisible: p-13 ];
            [ROMCode scrollRowToVisible: p+13  ];
        }
    }

//    lp = _PC.a & (ROM_no_of_bytes-1);
    
    [ROMCode  reloadData];
    [RAMCode  reloadData];
    [RegCode  reloadData];

}


-(void) executeManySteps {
    [ROMCode  reloadData];
    [RAMCode  reloadData];
    [RegCode  reloadData];

    unsigned int i,j,k;
    
    [[NSNotificationCenter defaultCenter] postNotificationName: @"status" object:[NSString stringWithFormat:@"Executing ... %3.2f %%",0.0 ]  ];
    
    
    k = (unsigned int) numberOfStepsPerLoop;
	
    for (i = 0;i<numberOfLoops;i++){

	j = Execute( k );
        
	if( j!= 0 ){  i = numberOfLoops; }

        [[NSNotificationCenter defaultCenter] postNotificationName: @"status" object:[NSString stringWithFormat:@"Executing ... %f %%",(float) (i*100.0/numberOfLoops) ]  ];
      
        
//	[ProgressBar setDoubleValue: i*100.0/numberOfLoops];
//	[ProgressBar displayIfNeeded];

	//lp = _PC.a & (ROM_no_of_bytes-1);
    }
    
    	[ROMCode  reloadData];
    	[RAMCode  reloadData];
    	[RegCode  reloadData];
    
    p = _PC.a & (ROM_no_of_bytes-1);
    
    if( p < 13){
        [ROMCode scrollRowToVisible: 26 ];  // should be the number of visable rows ...
        [ROMCode scrollRowToVisible: 0  ];
    } else{
        if( p > (ROM_no_of_bytes-1)-13){
            [ROMCode scrollRowToVisible: (ROM_no_of_bytes-1)-26  ];
            [ROMCode scrollRowToVisible: (ROM_no_of_bytes-1)     ];
        } else{
            [ROMCode scrollRowToVisible: p+13 ];
            [ROMCode scrollRowToVisible: p-13 ];
            [ROMCode scrollRowToVisible: p+13 ];
        }
    }
    
    [ROMCode displayIfNeeded ];
    [RAMCode displayIfNeeded ];
    [RegCode displayIfNeeded ];
    
   // lp = _PC.a & (ROM_no_of_bytes-1);

    [[NSNotificationCenter defaultCenter] postNotificationName: @"status" object:[NSString stringWithFormat:@"Executing ... %3.2f %%",100.0 ]  ];
    
}

- (IBAction)ButtonHalt:(id)sender
{

  //  [ProgressBar setDoubleValue: 100];
  //  [ProgressBar displayIfNeeded];

    // could set some var, which resets the Main loop counter ..., of flag therein
}

// Should introduce a thread to allow the action to be halted, e.t.c.

float timeTaken,timeTaken2;

static UnsignedWide startTime, endTime;

// Start the timer
void StartClock ( void ) {  Microseconds( &startTime ); } 

// Stop clock
float StopClock(void  ) { 
    Microseconds( &endTime );
    if ( endTime.hi == startTime.hi )	return( ( float ) ( endTime.lo - startTime.lo ));
    else				return(  -1.0f );
}

-(IBAction)ButtonStepMultiple:(id)sender
{
 //   [ProgressBar setDoubleValue: 0];
 //   [ProgressBar displayIfNeeded];

//  [[NSThread alloc] detachNewThreadSelector:@selector(executeManySteps:) toTarget:self withObject:nil];
  StartClock();

  [self executeManySteps];
  
  timeTaken = StopClock();
  
  [[NSNotificationCenter defaultCenter] postNotificationName: @"status" object:[NSString stringWithFormat:
      @"Simulation ran at %3.2f MIPs, for %i million cpu cycles", (numberOfStepsPerLoop*numberOfLoops )/timeTaken, (unsigned int)( (numberOfStepsPerLoop*numberOfLoops)/1000000.0)  ]  ];

 // printf("number of cycles -> %l \n",numberOfCyclesTaken);
  
  
 // [ProgressBar setDoubleValue: 100];
 // [ProgressBar displayIfNeeded];
  
}

    
    
- (void) upDateStatus:(NSNotification *)notification
{
    NSString *theText = [notification object];
   // printf("called here\n");
    [ StatusText setStringValue:theText ];

}

-(IBAction)ButtonRead:(id)sender
{
   // [ StatusText setStringValue:[NSString stringWithCString:"Looking For PicKit ..." ] ];
    
    
   
    // Read all memory from the PIC device, ... 
    if(  ! ReadFromPic()  ){
        
        p = _PC.a & (ROM_no_of_bytes-1);
        if( p < 13){
            [ROMCode scrollRowToVisible: 26  ];
            [ROMCode scrollRowToVisible: 0  ];
        } else{
            if( p > (ROM_no_of_bytes-1)-13){
                [ROMCode scrollRowToVisible: (ROM_no_of_bytes-1)-26  ];
                [ROMCode scrollRowToVisible: (ROM_no_of_bytes-1)     ];
            } else{
                [ROMCode scrollRowToVisible: p+13 ];
                [ROMCode scrollRowToVisible: p-13 ];
                [ROMCode scrollRowToVisible: p+13  ];
            }
        }
        
 //       lp = _PC.a & (ROM_no_of_bytes-1);
        
        [ROMCode  reloadData];
    }
    

}


-(IBAction)ButtonProgram:(id)sender
{
}

-(IBAction)ButtonStepSingle:(id)sender
{

//    [ProgressBar setDoubleValue: 100];
 //   [ProgressBar displayIfNeeded];
    
    Execute(1);
     
    [ROMCode  reloadData];
    [RAMCode  reloadData];
    [RegCode  reloadData];

    p = _PC.a & (ROM_no_of_bytes-1);
    if( p < 13){
        [ROMCode scrollRowToVisible: 26  ];
        [ROMCode scrollRowToVisible: 0  ];
    } else{
        if( p > (ROM_no_of_bytes-1)-13){
            [ROMCode scrollRowToVisible: (ROM_no_of_bytes-1)-26  ];
            [ROMCode scrollRowToVisible: (ROM_no_of_bytes-1)     ];
        } else{
                [ROMCode scrollRowToVisible: p+13 ];
                [ROMCode scrollRowToVisible: p-13 ];
                [ROMCode scrollRowToVisible: p+13 ];
            }
        }
    
    
 //   lp = _PC.a & (ROM_no_of_bytes-1);

}

/*
 -(IBAction)TextNumberOfSteps:(id)sender
{
    
  //  printf("Editing of NSTextFeild\n");
}
*/





@end
