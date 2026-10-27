/* regCode */

#import <Cocoa/Cocoa.h>

@interface regCode : NSObject
{
    IBOutlet NSTableView *regTable; // was textview ???
}


- (int)numberOfRowsInTableView:(NSTableView *)aTableView;
- (id )tableView:(NSTableView *)aTableView objectValueForTableColumn:(NSTableColumn *)aTableColumn row:(int)rowIndex;


@end
