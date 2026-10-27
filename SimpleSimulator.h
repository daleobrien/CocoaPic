/* SimpleSimulator */

#import <Cocoa/Cocoa.h>

@interface SimpleSimulator : NSObject
{
  //  IBOutlet NSProgressIndicator *ProgressBar;
    IBOutlet NSTableView *RAMCode;  // was id now xxx*
    IBOutlet NSTableView *ROMCode; // was id
    IBOutlet NSTableView *RegCode; // was id
    IBOutlet NSTextField *StatusText;
}

- (void) executeManySteps;

- (IBAction) ButtonGo:(id)sender;
- (IBAction) ButtonHalt:(id)sender;
- (IBAction) ButtonStepMultiple:(id)sender;
- (IBAction) ButtonStepSingle:(id)sender;
- (IBAction) ButtonRead:(id)sender;
- (IBAction) ButtonProgram:(id)sender;


@end
