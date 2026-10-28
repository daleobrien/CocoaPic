#import "regCode.h"

#include "PIC12F675/Headers/PIC12F675_cpuMemory.h"

extern char*  getNameOfReg(unsigned int idx);
extern unsigned int  getValueOfReg(unsigned int idx);
@implementation regCode

static int fs[32] = {0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0};
static int ps[32] = {0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0};
static int pc[32] = {0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0};


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

- (id)tableView:(NSTableView *)aTableView objectValueForTableColumn:(NSTableColumn *)aTableColumn  row:(int)rowIndex
{
    NSMutableAttributedString *mutableString ;
    // Create text the represts the column header
    NSString *identifier = [aTableColumn identifier];
    
    [aTableColumn setEditable:YES];

    if ( [identifier isEqualToString:[[NSString alloc]initWithCString: "reg"]]){

        return ([[NSString alloc]initWithCString: getNameOfReg(rowIndex) ]);

    } 
    else {
        // Reg value
        NSString *regValue  = [[NSNumber numberWithInt:(unsigned int)getValueOfReg(rowIndex)] stringValue];
        mutableString       = [[NSMutableAttributedString alloc] initWithString:regValue];

        if ( _PC.a !=  pc[rowIndex] ){ ps[rowIndex] = 0;  }
        pc[rowIndex] = _PC.a;
        
        // Chance the color if the value has changed since the last display update
        if ((fs[rowIndex] != getValueOfReg(rowIndex) ) || (ps[rowIndex] == 1)){
             fs[rowIndex]  = getValueOfReg(rowIndex) ;

            ps[rowIndex] = 1;
            [mutableString addAttribute:NSForegroundColorAttributeName value:[NSColor redColor] range:NSMakeRange(0,[regValue length]) ];
        }
 
        return (mutableString);

    }

}


- (int)numberOfRowsInTableView:(NSTableView *)aTableView
{

    return(6);// for each memory location
}

@end
