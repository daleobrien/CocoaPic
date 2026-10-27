//
//  ram Table.m
//  CocoaPic
//
//  Created by Orla O'Brien on Wed May 22 2002.
//  Copyright (c) 2002 __MyCompanyName__. All rights reserved.
//

#import "ram Table.h"


@implementation ram_Table


- (id)tableView:(NSTableView *)aTableView objectValueForTableColumn:(NSTableColumn *)aTableColumn  row:(int)rowIndex
{

    // Create text the represts the column header
    NSString *identifier = [aTableColumn identifier];

    // create text that represnts the row number, i.e. the PC counter
    NSString *pcCounter  = [[NSNumber numberWithInt:rowIndex] stringValue];

    [aTableColumn setEditable:NO];

    if ( [identifier isEqualToString:[[NSString alloc]initWithCString: "pc"]]){

        return (pcCounter);

    } else {

        return ([[NSString alloc]initWithCString: "ram"]);
    }

}


- (int)numberOfRowsInTableView:(NSTableView *)aTableView
{

    return(32);// for each memory location
}

@end
