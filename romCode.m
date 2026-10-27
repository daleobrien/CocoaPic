//
//  rom Table.m
//  CocoaPic
//
//  Created by Orla O'Brien on Wed May 22 2002.
//  Copyright (c) 2002 __MyCompanyName__. All rights reserved.
//

#import "romCode.h"

#include "PIC12F675/Headers/PIC12F675_cpuCore.h"
#include "PIC12F675/Headers/PIC12F675_cpuMemory.h"

NSMutableDictionary *romNames;
NSMutableDictionary *romComments;

extern 	NSMutableDictionary *ramNames;

extern char* getStringOfInstruction(int idx);
extern char* DecToHex(unsigned int no, unsigned int noOfPlaces);
extern char* getStringOfInstructionII(int idx);

int determineTypeOfInstuction( int idx){
 
    if(( idx >=   128 )&&( idx<=   255 ) ) { return(2); } else  //       fff ffff
    if(( idx >=   384 )&&( idx<=   511 ) ) { return(2); } else  //       fff ffff
    if(( idx >=   512 )&&( idx<=  4095 ) ) { return(3); } else  //      dfff ffff
    if(( idx >=  4096 )&&( idx<=  8191 ) ) { return(4); } else  //   bb bfff ffff
    if(( idx >=  8192 )&&( idx<= 12287 ) ) { return(5); } else  // kkk kkkk kkkk
    if(( idx >= 12288 )&&( idx<= 14336 ) ) { return(6); } else  //  kk kkkk kkkk
    if(( idx >= 14336 )&&( idx<= 15103 ) ) { return(7); } else  //     kkkk kkkk
    if(( idx >= 15103 )&&( idx<= 16383 ) ) { return(8); } else  //   k kkkk kkkk
	
    return(0); 						  	// no paramaters ...
}


// deterime if a ,f or ,F is specified
int extractDestinationFrom(NSString *string){

    // Check for a ","
    NSString *list 	= [ NSString stringWithString: [string lowercaseString] ];
    NSArray  *listItems = [ list componentsSeparatedByString:@"," ];

    // Look for a "f"
    if([listItems count]>1){

	NSScanner *theScanner  = [NSScanner scannerWithString: [listItems objectAtIndex:1 ] ];
	NSCharacterSet * chset = [NSCharacterSet characterSetWithCharactersInString: @"f"];

	if ( [theScanner scanCharactersFromSet:chset intoString:nil]==TRUE ){	return(0x080);}
    }
    
    //default to a "w"
    return(0x000);
}


unsigned int extractBitfieldFrom(NSString *string){

    unsigned int intValue[1];
    
    // Check for a ","
    NSString *list 	= [ NSString stringWithString: [string lowercaseString] ];
    NSArray  *listItems = [ list componentsSeparatedByString:@"," ];

    NSScanner *theScanner  = [NSScanner scannerWithString: [listItems objectAtIndex:0 ] ];
    [theScanner scanInt: intValue];

    return( intValue[0] );

}

// will pass either a INT of HEX
int convertToIntFrom(NSString *string){

    unsigned int intValue[1];

    NSString *list 	= [ NSString stringWithString: [string lowercaseString] ];
    NSArray  *listItems = [ list componentsSeparatedByString:@"0x" ];

    NSScanner *theScanner = [NSScanner scannerWithString: string ];

    if([listItems count]>1){ [theScanner scanHexInt:intValue];
    }else{		     [theScanner scanInt:   intValue];  }

    return( intValue[0] );
}

unsigned int extractFileFrombbbffff(NSString *string){

//    unsigned int intValue[1];

    // Check for a ","
    NSString *list 	= [ NSString stringWithString: [string lowercaseString] ];
    NSArray  *listItems = [ list componentsSeparatedByString:@"," ];

    if([listItems count]>1){
	return( convertToIntFrom([listItems objectAtIndex:1 ]) );
    }
  
    return( 0 );
}


// i.e "addw" -> "ADDWF 0x01,w"
int convertStringToBestMatchingInstruction( NSString *string ,int lastInstruction ){

    NSString *upperCaseString = [ NSString stringWithString: [string uppercaseString] ];
    NSString *trimmedString   = [ upperCaseString stringByTrimmingCharactersInSet:[NSCharacterSet whitespaceCharacterSet]];

    // copy users text to a local structure
    char oText[14];
    int len = [trimmedString cStringLength];

    if(len>14) len = 14;

    int i;
    for(i=0; i<len;i++)   { oText[i] = [trimmedString characterAtIndex:i ]; }

     // Scan through ROM looking for matchs, first try all the chars ...
    char* rText;
    int j;
    BOOL match;
    BOOL fChar;
    int bestIdx = 0;
    int bestROMCode = lastInstruction;

    for( j=0; j<16383; j++ ){
	
	rText 	= getStringOfInstructionII(j);
	match 	= TRUE;
	fChar   = FALSE;

	for(i=0;i<bestIdx;i++){
	    if( rText[i] != oText[i] ){ match = FALSE; }
	}
	
	do {
	  
	   if( rText[i] != oText[i] ){ match = FALSE; }

	    if( match && ( i >= bestIdx)  ){ // ... better match ?

		bestIdx     = i+1; // look only for better ones.
		bestROMCode = j;

		switch( determineTypeOfInstuction(j) ){
		    case 2: bestROMCode = (j&0x3f80) |     (0x07f & lastInstruction); break; //     fff ffff
		    case 3: bestROMCode = (j&0x3f00) |     (0x0ff & lastInstruction); break;//     dfff ffff
		    case 4: bestROMCode = (j&0x3c00) |     (0x3ff & lastInstruction); break;//  bb bfff ffff
		    case 5: bestROMCode = (j&0x3800) |     (0x7ff & lastInstruction); break;// kkk kkkk kkkk
		    case 6: bestROMCode = (j&0x3c00) |     (0x3ff & lastInstruction); break;//  kk kkkk kkkk
		    case 7: bestROMCode = (j&0x3f00) |     (0x0ff & lastInstruction); break;//     kkkk kkkk
		    case 8: bestROMCode = (j&0x3e00) |     (0x1ff & lastInstruction); break;//   k kkkk kkkk
		}
	    }
	    i++;    
	} while( (i<len) && match ); // exit this loop as soon as the first match is found
    }

    return(bestROMCode);
}


@implementation romCode

- (IBAction)ButtonGo:(id)sender{
    printf("button down\n");
}

- (id)init{
    
    self = [super init];
    


    romNames = [NSMutableDictionary dictionaryWithCapacity: ROM_no_of_bytes ];
    [romNames retain];

    romComments = [NSMutableDictionary dictionaryWithCapacity: ROM_no_of_bytes ];
    [romComments retain];
    

    [[NSNotificationCenter defaultCenter] addObserver:self selector:@selector(upDate)  name:@"relinkFileNames" object:nil];

	
    return(self);
}



-(void) awakeFromNib {

    
    NSTableColumn *aColumn1 = [[NSTableColumn alloc] initWithIdentifier:[[NSString alloc]initWithCString: "*" ] ];
    [aColumn1 setDataCell: [[NSTextFieldCell alloc] initTextCell:[NSString stringWithUTF8String:""]] ];
    [aColumn1 setResizable:FALSE];
    [aColumn1 setWidth:23.0];

    [aColumn1 setHeaderCell:[[NSTableHeaderCell alloc] initTextCell:[NSString stringWithUTF8String:"✘"]]];
    [[aColumn1 headerCell] setAlignment:NSCenterTextAlignment];

   NSTableColumn *aColumn2 = [[NSTableColumn alloc] initWithIdentifier:[[NSString alloc]initWithCString: "pc" ] ];
   [aColumn2 setDataCell: [[NSTextFieldCell alloc] initTextCell:[[NSString alloc]initWithCString:""]] ];
   [aColumn2 setWidth:60.0];
   [aColumn2 setHeaderCell:[[NSTableHeaderCell alloc] initTextCell:[[NSString alloc]initWithCString:"addr"]]];
  

   NSTableColumn *aColumn3 = [[NSTableColumn alloc] initWithIdentifier:[[NSString alloc]initWithCString: "Instruction" ] ];
   [aColumn3 setDataCell: [[NSTextFieldCell alloc] initTextCell:[[NSString alloc]initWithCString:""]] ];
   [aColumn3 setHeaderCell:[[NSTableHeaderCell alloc] initTextCell:[[NSString alloc]initWithCString:"Instruction"]]];

   NSTableColumn *aColumn4 = [[NSTableColumn alloc] initWithIdentifier:[[NSString alloc]initWithCString: "Data" ] ];
   [aColumn4 setDataCell: [[NSTextFieldCell alloc] initTextCell:[[NSString alloc]initWithCString:""]] ];
   [aColumn4 setHeaderCell:[[NSTableHeaderCell alloc] initTextCell:[[NSString alloc]initWithCString:"Data"]]];
   
   NSTableColumn *aColumn5 = [[NSTableColumn alloc] initWithIdentifier:[[NSString alloc]initWithCString: "Comment" ] ];
   [aColumn5 setDataCell: [[NSTextFieldCell alloc] initTextCell:[[NSString alloc]initWithCString:""]] ];
   [aColumn5 setHeaderCell:[[NSTableHeaderCell alloc] initTextCell:[[NSString alloc]initWithCString:"Notes"]]];

    [ [ aColumn1 dataCell]  setEditable:FALSE];
    [ [ aColumn2 dataCell]  setEditable:TRUE];
    [ [ aColumn3 dataCell]  setEditable:TRUE];
    [ [ aColumn4 dataCell]  setEditable:TRUE];
    [ [ aColumn5 dataCell]  setEditable:TRUE];
	
    [romTable  addTableColumn:aColumn1];
    [romTable  addTableColumn:aColumn2];
    [romTable  addTableColumn:aColumn3];
    [romTable  addTableColumn:aColumn4];
    [romTable  addTableColumn:aColumn5];

    [romTable setTarget:self];
    [romTable setDoubleAction:@selector(doCommand:)];
    [romTable setDrawsGrid:FALSE];
    [romTable setIntercellSpacing:NSMakeSize(0,0)];

    [romTable setRowHeight:17.0];
   // [romTable sizeLastColumnToFit];  // moves them all 
  }

// Toggle Break Point,
// add/remove graphic 
// 
- (void) doCommand:(int)tag{

      breakPoints[[romTable selectedRow]] ^= 1;

      [romTable deselectRow:[romTable selectedRow]]; 
      [self upDate];
   
}


-(void)dealloc{
    
    [super dealloc];
    [romNames release];
    [romComments release];

}


-(void) upDate {
   
    [ romTable reloadData ];

    printf("update ... \n");
   // [ romTable setBackgroundColor:(NSColor *)blackColor];
    [ romTable setNeedsDisplay:TRUE];
    
}


- (void)tableView:(NSTableView *)aTableView setObjectValue:anObject forTableColumn:(NSTableColumn *)aTableColumn row:(int)rowIndex{

    NSString 	*identifier 	= [aTableColumn identifier];
 
    // Instruction Column
    if ( [identifier isEqualToString:[[NSString alloc]initWithCString: "Instruction" ]] ){

	ROM[ rowIndex ] = convertStringToBestMatchingInstruction( anObject,ROM[ rowIndex ] );	

    // Data Column
    } else if ( [identifier isEqualToString:[[NSString alloc]initWithCString: "Data"]]){

	int g;
	switch( determineTypeOfInstuction( ROM[ rowIndex ] ) ){

	    case 1: // fff
	    {

		// need to check for "porta" in the entered text as well ...
		g = (0x07 & convertToIntFrom(anObject) );
		if(g < 5) g = 5;
		ROM[rowIndex]  = ( ROM[rowIndex] & 0xff8) | g;
	    }
	    break;
		
	    case 2: // fff ffff
	    {
		NSArray  *user = [ anObject componentsSeparatedByString:@"," ];
		NSArray *ram = [ramNames allKeysForObject:[user objectAtIndex:0]];

		ROM[rowIndex] = ( ROM[rowIndex] & 0x3f80);
		
		if([ram count] >0){
		    ROM[rowIndex] |=  (0x7f & [[ram objectAtIndex:0] intValue] );
		}else{ 
		    ROM[rowIndex] |=  (0x7f & convertToIntFrom(anObject) );
		}
		
	    }
	    break;
		
	    case 3: // dfff ffff
	    {

		NSArray  *user = [ anObject componentsSeparatedByString:@"," ];
		NSArray *ram   = [ramNames allKeysForObject:[user objectAtIndex:0]];

		ROM[rowIndex] = ( ROM[rowIndex] & 0x3f00) | extractDestinationFrom(anObject);
		
		if([ram count] >0){
		    ROM[rowIndex] |=  (0x7f & [[ram objectAtIndex:0] intValue] )  ;
		}else{ 
		    ROM[rowIndex] |=  (0x7f & convertToIntFrom(anObject) )   ;
		}
		
	    }
		break;

	    case 4: // bb bfff ffff
	    {
		
	    
		NSArray  *user = [ anObject componentsSeparatedByString:@"," ];
		NSArray *ram   = [ramNames allKeysForObject:[user objectAtIndex:0]];

		ROM[rowIndex] &= 0x3c00; // just keep the opp code

		if( [user count] >1){ ROM[rowIndex] |=  (( 0x7&[[user objectAtIndex:1] intValue])<<7); }
		
		if([ram count] >0){
		    ROM[rowIndex] |=  (0x007f & [[ram objectAtIndex:0] intValue] ); // Point to the file with the matching name
		}else{
		    ROM[rowIndex] |=  (0x007f & convertToIntFrom(anObject) ); // Point to the file with the matching name
		}
		printf("in  %x \n", ROM[rowIndex]);
	    }
		break;
		
	    case 5: // kkk kkkk kkkk

	    {
		NSArray  *user = [ anObject componentsSeparatedByString:@"," ];
		NSArray *rom   = [romNames allKeysForObject:[user objectAtIndex:0]];

		if([rom count] >0){
		    ROM[rowIndex]  = ( ROM[rowIndex] & 0x3800) | (0x7ff & [[rom objectAtIndex:0] intValue] );
		}else{
		    ROM[rowIndex]  = ( ROM[rowIndex] & 0x3800) | (0x7ff & convertToIntFrom(anObject) );
		}
		
	    }
		
		break;
		
	    case 6: // kk kkkk kkkk
	    {
		NSArray  *user = [ anObject componentsSeparatedByString:@"," ];
		NSArray *rom   = [romNames allKeysForObject:[user objectAtIndex:0]];

		if([rom count] >0){
		    ROM[rowIndex]  = ( ROM[rowIndex] & 0x3c00) | (0x3ff & [[rom objectAtIndex:0] intValue] );
		}else{
		    ROM[rowIndex]  = ( ROM[rowIndex] & 0x3c00) | (0x3ff & convertToIntFrom(anObject) );
		}

	    }
		//ROM[rowIndex]  = ( ROM[rowIndex] & 0x3c00) | (0x3ff & convertToIntFrom(anObject) );
		break;

	    case 7: // kkkk kkkk
	    {
		NSArray  *user = [ anObject componentsSeparatedByString:@"," ];
		NSArray *rom   = [romNames allKeysForObject:[user objectAtIndex:0]];

		if([rom count] >0){
		    ROM[rowIndex]  = ( ROM[rowIndex] & 0x3f00) | (0x0ff & [[rom objectAtIndex:0] intValue] );
		}else{
		    ROM[rowIndex]  = ( ROM[rowIndex] & 0x3f00) | (0x0ff & convertToIntFrom(anObject) );
		}

	    }
		
		//ROM[rowIndex]  = ( ROM[rowIndex] & 0x3f00) | (0x0ff & convertToIntFrom(anObject) );
		break;

	    case 8: // k kkkk kkkk
	    {
		NSArray  *user = [ anObject componentsSeparatedByString:@"," ];
		NSArray *rom   = [romNames allKeysForObject:[user objectAtIndex:0]];

		if([rom count] >0){
		    ROM[rowIndex]  = ( ROM[rowIndex] & 0x3e00) | (0x1ff & [[rom objectAtIndex:0] intValue] );
		}else{
		    ROM[rowIndex]  = ( ROM[rowIndex] & 0x3e00) | (0x1ff & convertToIntFrom(anObject) );
		}

	    }
		
//		ROM[rowIndex]  = ( ROM[rowIndex] & 0x3e00) | (0x1ff & convertToIntFrom(anObject) );
		break;
	}

    // ps, was label, ... label
    } else if ( [identifier isEqualToString:[[NSString alloc]initWithCString: "pc"]]){

	if( [(NSString*)anObject length] > 0 ){

	    if( ! [(NSString*)anObject isEqual: [[NSString alloc]initWithCString: DecToHex(rowIndex,3) ]]  ){
		
		[romNames  setObject:anObject forKey: [NSString  stringWithFormat:@"%i",rowIndex] ];

	    }else{

		if( [romNames objectForKey:[NSString  stringWithFormat:@"%i",rowIndex] ] ){
		    [romNames removeObjectForKey:[NSString  stringWithFormat:@"%i",rowIndex] ];
		}
	    }
	}
	
	// new to redisplay the 
	//NSLog(@"%@\n",romNames);

	// update any labels in goto statements etc...
	[romTable setNeedsDisplay ];

    }else if ( [identifier isEqualToString:[[NSString alloc]initWithCString: "Comment"]]){

	if( [(NSString*)anObject length] > 0 ){

	    [romComments  setObject:anObject forKey: [NSString  stringWithFormat:@"%i",rowIndex] ];

	}
    }
}

- (void)keyDown:(NSEvent *)theEvent{
    printf("keydown\n");
}

- (void)mouseDrown:(NSEvent *)theEvent{
     printf("mouse down\n");
}

- (BOOL)tableView:(NSTableView *)aTableView shouldEditTableColumn:(NSTableColumn *)aTableColumn row:(int)rowIndex{

    NSString *identifier = [aTableColumn identifier];

    // Check the to see if the apple key is pressed ?
    // but how ?
  
    //return(FALSE);
    
    if ( [identifier isEqualToString:@"*" ] ){
	return(FALSE);
    }
    return(TRUE);
	
}

- (id)tableView:(NSTableView *)aTableView objectValueForTableColumn:(NSTableColumn *)aTableColumn  row:(int)rowIndex
{

    // Create text the represts the column header

    NSString 			*identifier = [aTableColumn identifier];
    NSMutableAttributedString 	*mutableString = [NSMutableAttributedString alloc];


    // PC Column
    if ( [identifier isEqualToString:@"pc" ] ){

	[aTableColumn setEditable:YES];

	NSString *p = [romNames objectForKey: [NSString  stringWithFormat:@"%i",rowIndex] ];

	if(p==nil){
	     p = [[NSString alloc]initWithString: [[NSString alloc]initWithCString: DecToHex(rowIndex,3) ] ];
	    [mutableString initWithString:p];
	    [mutableString addAttribute:NSForegroundColorAttributeName value:[NSColor lightGrayColor] range:NSMakeRange(0,[mutableString length]) ];
	    [mutableString addAttribute:NSFontAttributeName value:[NSFont systemFontOfSize:9.0] range:NSMakeRange(0,[mutableString length]) ];

	
       }else{
	   [mutableString initWithString:p];
       }

	
	
    // Instruction Column
    } else if ( [identifier isEqualToString:@"Instruction" ] ){

	NSString *ina = [[NSString alloc]initWithCString: getStringOfInstruction(rowIndex) ];
	NSString *ins = [ina stringByTrimmingCharactersInSet:[NSCharacterSet whitespaceCharacterSet]];
        NSArray  *listItems  = [ins componentsSeparatedByString:@" "];

	[ mutableString  initWithString:[listItems objectAtIndex:0]];

    // Data Column
    } else if ( [identifier isEqualToString:@"Data"]){

	NSString *p;
	
	 switch( determineTypeOfInstuction(ROM[ rowIndex ]) ){

	     case 1: // fff
		 p = [ramNames objectForKey: [NSString  stringWithFormat:@"%i",ROM[rowIndex] & 0x7] ];
		 if( p==nil){ p = [NSString stringWithFormat: @"%x", ROM[rowIndex] & 0x7 ];	}
		 [mutableString   initWithString:p];
		 break;
		 
	     case 2: // fff ffff
		 // check ffff=0, then set to pointer address, i.e. [name]
		 p = [ramNames objectForKey: [NSString  stringWithFormat:@"%i",(ROM[rowIndex]&0x1f) ] ];
		 if( p==nil){ p = [NSString stringWithFormat: @"0x%x", ROM[rowIndex] & 0x7F  ];	}
		 [mutableString   initWithString:p];
		 break;

	     case 3: // d ffffff  -> <name>,d or f,d
		 if(ROM[rowIndex]&0x80){
		     p = [ramNames objectForKey: [NSString  stringWithFormat:@"%i",(ROM[rowIndex]&0x7f) ] ];
		     if( p==nil){ p = [NSString stringWithFormat: @"0x%x", ROM[rowIndex] & 0x7F  ];	}
		     [ mutableString  initWithString:[p stringByAppendingString:[NSString  stringWithFormat:@",f" ] ]];
		    
		 }else{
		     p = [ramNames objectForKey: [NSString  stringWithFormat:@"%i",(ROM[rowIndex]&0x7f) ] ];
		     if( p==nil){ p = [NSString stringWithFormat: @"0x%x", ROM[rowIndex] & 0x1F  ];	}
		      [ mutableString  initWithString:[p stringByAppendingString:[NSString  stringWithFormat:@",w" ] ]];
		  
		 }
		 break;
		     
	     case 4: // ii iibb bfff ffff
		 
		 p = [ramNames objectForKey: [NSString  stringWithFormat:@"%i",(ROM[rowIndex]&0x7f) ] ];
		 if( p==nil){ p = [NSString stringWithFormat: @"0x%x", ROM[rowIndex] & 0x7f  ];	}

		// apend string with ,bbb
		[ mutableString  initWithString:[p stringByAppendingString:[NSString  stringWithFormat:@",%i",(ROM[rowIndex]&0x380)>>7 ] ]];
		 
		 break;
		 
	     case 5: // kkk kkkk kkkk
		 p = [romNames objectForKey: [NSString  stringWithFormat:@"%i",(ROM[rowIndex] & 0x7ff) ] ];
		 if( p==nil){ p = [NSString stringWithFormat: @"0x%x", ROM[rowIndex]& 0x7ff  ];	}
		 [mutableString   initWithString:p];
		 break;

	     case 6: // kk kkkk kkkk
		 p = [romNames objectForKey: [NSString  stringWithFormat:@"%i",(ROM[rowIndex] & 0x3ff) ] ];
		 if( p==nil){ p = [NSString stringWithFormat: @"0x%x", ROM[rowIndex]& 0x3ff  ];	}
	         [mutableString   initWithString:p];
		 break;

	     case 7: //  kkkk kkkk
		 p = [romNames objectForKey: [NSString  stringWithFormat:@"%i",(ROM[rowIndex] & 0x0ff) ] ];
		 if( p==nil){ p = [NSString stringWithFormat: @"0x%x", ROM[rowIndex]& 0x0ff  ];	}
		     [mutableString   initWithString:p];
		 break;

	     case 8: //  k kkkk kkkk
		 p = [romNames objectForKey: [NSString  stringWithFormat:@"%i",(ROM[rowIndex] & 0x1ff) ] ];
		 if( p==nil){ p = [NSString stringWithFormat: @"0x%x", ROM[rowIndex]& 0x1ff  ];	}
		     [mutableString   initWithString:p];
		 break;

	     default:
		 [mutableString   initWithString:@""];
		 break;
	 }
    } else if( [identifier isEqualToString:@"Comment"]){
	
	NSString *p = [romComments objectForKey: [NSString  stringWithFormat:@"%i",rowIndex] ];
	if(p==nil){  p = [[NSString alloc]initWithCString: "" ];}

	[mutableString   initWithString:p];
	
    

    } else if ( [identifier isEqualToString:@"*"]){

	if((breakPoints[rowIndex] != 0)){
	    [mutableString   initWithString:[NSString stringWithUTF8String:"✘"]]; // "⦿◉⦾✘"
	    [mutableString setAlignment:NSCenterTextAlignment range:NSMakeRange(0,[mutableString length]) ];
	}else{
	    [mutableString   initWithString:[[NSString alloc]initWithCString:""]];
	}

	
	
    }else{
	
	[aTableColumn setEditable:YES];
	[mutableString   initWithString:[[NSString alloc]initWithCString:""]];
    }


    // Highlight the PC Counter row ...
    if (rowIndex == ( (ROM_no_of_bytes-1)& _PC.a)){
	[mutableString addAttribute:NSForegroundColorAttributeName value:[NSColor redColor] range:NSMakeRange(0,[mutableString length]) ];
    }

     
    return (mutableString);
 
}



//

- (void)tableView:(NSTableView *)aTableView willDisplayCell:(id)aCell  forTableColumn:(NSTableColumn *)aTableColumn row:(int)rowIndex
{

    if ( 1|| ![[aTableColumn identifier] isEqualToString:@"*"] )
    {


	// Colour each row as alternating colors
	if (rowIndex % 2)
	{
	    [aCell setDrawsBackground:YES];
	    [aCell setBackgroundColor:[NSColor colorWithCalibratedRed:0.95 green:0.95 blue:1 alpha:1]];
	}else{
	    [aCell setDrawsBackground:YES];
	    [aCell setBackgroundColor:[NSColor whiteColor]];
	}

	// Colour Breakpoint rows
	//if(breakPoints[rowIndex] != 0){
	  //  [aCell setDrawsBackground:YES];
	    //[aCell setBackgroundColor:[NSColor colorWithCalibratedRed:1 green:0.80 blue:0.80 alpha:1]];
	//}

    }else{ 
	//[aCell setControlTint:NSClearControlTint];
    }
    
}

- (int)numberOfRowsInTableView:(NSTableView *)aTableView
{
    return( ROM_no_of_bytes );  // for each memory location
}


-(void) relinkFileNames{   }

-(void) activateNotifications:(BOOL)activate{
    if(activate){ [[NSNotificationCenter defaultCenter] addObserver:self selector:@selector(upDate:)  name:@"relinkFileNames" object:nil];}
    else{         [[NSNotificationCenter defaultCenter] removeObserver:self]; }
}

@end
