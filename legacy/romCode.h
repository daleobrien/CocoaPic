//
//  rom Table.h
//  CocoaPic 
//
//  Created by Orla O'Brien on Wed May 22 2002.
//  Copyright (c) 2002 __MyCompanyName__. All rights reserved.
//

#import <Cocoa/Cocoa.h>


@interface romCode : NSObject {

    IBOutlet NSTableView *romTable;
      
}

- (IBAction)ButtonGo:(id)sender;

- (void) upDate;

- (void) relinkFileNames;

- (void) activateNotifications:(BOOL)activate;

- (int)numberOfRowsInTableView:(NSTableView *)aTableView;

- (id  )tableView:(NSTableView *)aTableView objectValueForTableColumn:(NSTableColumn *)aTableColumn row:(int)rowIndex;
- (void)tableView:(NSTableView *)aTableView setObjectValue:anObject    forTableColumn:(NSTableColumn *)aTableColumn row:(int)rowIndex;

- (void)tableView:(NSTableView *)aTableView willDisplayCell:(id)aCell  forTableColumn:(NSTableColumn *)aTableColumn row:(int)rowIndex;

@end
