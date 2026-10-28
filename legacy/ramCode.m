#import "ramCode.h"
#import "romCode.h"

#include "PIC12F675/Headers/PIC12F675_cpuCore.h"
#include "PIC12F675/Headers/PIC12F675_cpuMemory.h"

extern unsigned int  getValueOfRam(unsigned int idx);
extern void setValueOfRam(unsigned int idx, unsigned int data);


NSMutableDictionary *ramNames;

//extern NSMutableDictionary *ramNames;

extern char* DecToHex(unsigned int no, unsigned int noOfPlaces);


@implementation ramCode

// these are used to highlight RAM location that have changed from last the cycle
static int fs[RAM_no_of_bytes] ;
static int ps[RAM_no_of_bytes] ;
static int pc[RAM_no_of_bytes] ;



// Set up the names of the default RAM locations
- (id)init{
    self = [super init];

    ramNames = [NSMutableDictionary dictionaryWithCapacity:255];
    [ramNames retain];

    [ramNames  setObject:[[NSString alloc]initWithCString: "INDF"   ] forKey: [NSString  stringWithFormat:@"%i",0] ];
    [ramNames  setObject:[[NSString alloc]initWithCString: "TMR0"   ] forKey: [NSString  stringWithFormat:@"%i",1] ];
    [ramNames  setObject:[[NSString alloc]initWithCString: "PCL"    ] forKey: [NSString  stringWithFormat:@"%i",2] ];
    [ramNames  setObject:[[NSString alloc]initWithCString: "STATUS" ] forKey: [NSString  stringWithFormat:@"%i",3] ];
    [ramNames  setObject:[[NSString alloc]initWithCString: "FSR"    ] forKey: [NSString  stringWithFormat:@"%i",4] ];
    [ramNames  setObject:[[NSString alloc]initWithCString: "OSCCAL" ] forKey: [NSString  stringWithFormat:@"%i",5] ];
    [ramNames  setObject:[[NSString alloc]initWithCString: "GPIO"   ] forKey: [NSString  stringWithFormat:@"%i",6] ];


	
    return(self);
}

-(void)dealloc{
    [super dealloc];

   [ramNames release];
    
}

- (void)tableView:(NSTableView *)aTableView setObjectValue:anObject forTableColumn:(NSTableColumn *)aTableColumn row:(int)rowIndex
{
 //   char string[9];
    //  id theRecord;
    //  NSParameterAssert(rowIndex >= 0 && rowIndex < [records count]);
    //  theRecord = [records objectAtIndex:rowIndex];
    //  [theRecord setObject:anObject forKey:[aTableColumn identifier]];

    NSString *identifier = [aTableColumn identifier];

    if ( [identifier isEqualToString:[[NSString alloc]initWithCString: "value"]]){
         
        setValueOfRam( rowIndex,  [anObject intValue]);// PCL , update ROM pointers..
	if(rowIndex == 2){
	    [[NSNotificationCenter defaultCenter] postNotificationName:  @"relinkFileNames" object:self];
	}
    }
    else if ( [identifier isEqualToString:[[NSString alloc]initWithCString: "name"]]){

	[ramNames  setObject:anObject forKey: [NSString  stringWithFormat:@"%i",rowIndex] ];
     //   [anObject getCString:string maxLength:9];
        
       // setNameOfRam(rowIndex, anObject);
	
	[[NSNotificationCenter defaultCenter] postNotificationName:  @"relinkFileNames" object:self];
      
    }
    return;
}


- (id)tableView:(NSTableView *)aTableView objectValueForTableColumn:(NSTableColumn *)aTableColumn  row:(int)rowIndex
{
    // create text that represnts the row number, RAM index
    NSMutableAttributedString *mutableString ;

    // Create text the represts the column header
    NSString *identifier = [aTableColumn identifier];

    //
    if ( [identifier isEqualToString:[[NSString alloc]initWithCString: "#"]]){

	[aTableColumn setEditable:NO];
	
        mutableString    = [[NSMutableAttributedString alloc] initWithString:[[NSString alloc]initWithCString: DecToHex(rowIndex,2)]];
	
    	[mutableString addAttribute:NSForegroundColorAttributeName value:[NSColor lightGrayColor] range:NSMakeRange(0,[mutableString length]) ];
//	[mutableString addAttribute:NSFontAttributeName value:[NSFont fontWithName:[NSString  stringWithCString:"Helvetica"] size:12.0] range:NSMakeRange(0,[mutableString length]) ];
	[mutableString addAttribute:NSFontAttributeName value:[NSFont systemFontOfSize:10.0] range:NSMakeRange(0,[mutableString length]) ];

	
        return (mutableString);
	
	//return([[NSString alloc]initWithCString: DecToHex(rowIndex,2)]);

    } else if ( [identifier isEqualToString:[[NSString alloc]initWithCString: "value"]]){

        // Ram value
        NSString *Value  = [[NSNumber numberWithInt:(unsigned int)getValueOfRam(rowIndex)] stringValue];
        mutableString    = [[NSMutableAttributedString alloc] initWithString:Value];

        if ( _PC.a !=  pc[rowIndex] ){ ps[rowIndex] = 0;  }
        pc[rowIndex] = _PC.a;

        // Chance the color if the value has changed since the last display update
        if ((fs[rowIndex] != getValueOfRam(rowIndex) ) || (ps[rowIndex] == 1)){

             fs[rowIndex]  = getValueOfRam(rowIndex) ;

            ps[rowIndex] = 1;
            [mutableString addAttribute:NSForegroundColorAttributeName value:[NSColor redColor] range:NSMakeRange(0,[Value length]) ];
        }

        return (mutableString);
    } // NAME
    else {
        
	[aTableColumn setEditable:YES];
	
	    
	NSString *p = [ramNames objectForKey: [NSString  stringWithFormat:@"%i",rowIndex] ];
	if(p){ return( p );}
	else { return( [[NSString alloc]initWithCString: "" ]);	}

	
    }

}


- (void)tableView:(NSTableView *)aTableView willDisplayCell:(id)aCell  forTableColumn:(NSTableColumn *)aTableColumn row:(int)rowIndex
{


    if (rowIndex % 2)
    {
	[aCell setDrawsBackground:YES];
	[aCell setBackgroundColor:[NSColor colorWithCalibratedRed:0.95 green:0.95 blue:1 alpha:1]];
    }else{
	[aCell setDrawsBackground:YES];
	[aCell setBackgroundColor:[NSColor whiteColor]];
    }
}


- (int)numberOfRowsInTableView:(NSTableView *)aTableView
{

    return(RAM_no_of_bytes);// for each memory location
}


@end
