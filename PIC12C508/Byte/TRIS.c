/*
 *  TRIS.c
 *  ToolTest
 *
 *  Created by Orla O'Brien on Sun May 12 2002.
 *  Copyright (c) 2001 __MyCompanyName__. All rights reserved.
 *
 */

#include "TRIS.h"
#include "cpuMemory.h"

void TRIS0(void){ INDR_ADDR = _W; }
void TRIS1(void){ f[ 1] = _W; }
void TRIS2(void){ f[ 2] = _W; }
void TRIS3(void){ f[ 3] = _W; }
void TRIS4(void){ f[ 4] = _W; }

void TRIS5(void){ f[ 5] = _W; }
void TRIS6(void){ f[ 6] = _W; }
void TRIS7(void){ f[ 7] = _W; }