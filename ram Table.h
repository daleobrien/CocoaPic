//
//  ram Table.h
//  CocoaPic
//
//  Created by Orla O'Brien on Wed May 22 2002.
//  Copyright (c) 2002 __MyCompanyName__. All rights reserved.
//

#import <Foundation/Foundation.h>


@interface ram_Table : NSObject {


    IBOutlet NSTableView *romCODE;
}


- (int)numberOfRowsInTableView:(NSTableView *)aTableView;
- (id )tableView:(NSTableView *)aTableView
objectValueForTableColumn:(NSTableColumn *)aTableColumn
             row:(int)rowIndex;

@end
