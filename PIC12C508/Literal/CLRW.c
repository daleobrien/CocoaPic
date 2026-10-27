/*
 *  CLRW.c
 *  ToolTest
 *
 *  Created by Orla O'Brien on Sun May 12 2002.
 *  Copyright (c) 2001 __MyCompanyName__. All rights reserved.
 *
 */
#include "cpuMemory.h"
#include "CLRW.h"

void CLRWf000(void) { _W=0;  STATUS |=  setbit[Z];}