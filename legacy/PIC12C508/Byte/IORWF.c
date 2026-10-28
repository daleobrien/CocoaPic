/*
 *  IORWF.c
 *  ToolTest
 *
 *  Created by Orla O'Brien on Sun May 12 2002.
 *  Copyright (c) 2001 __MyCompanyName__. All rights reserved.
 *
 */
#include "cpuMemory.h"
#include "IORWF.h"

void IORWFd0f000(void) {_W = _W|INDR_ADDR; if(_W==0) STATUS |=  setbit[Z]; else STATUS &= clearbit[Z];}
void IORWFd0f001(void) {_W = _W|f[ 1]; if(_W==0) STATUS |=  setbit[Z]; else STATUS &= clearbit[Z];}
void IORWFd0f002(void) {_W = _W|f[ 2]; if(_W==0) STATUS |=  setbit[Z]; else STATUS &= clearbit[Z];}
void IORWFd0f003(void) {_W = _W|f[ 3]; if(_W==0) STATUS |=  setbit[Z]; else STATUS &= clearbit[Z];}
void IORWFd0f004(void) {_W = _W|f[ 4]; if(_W==0) STATUS |=  setbit[Z]; else STATUS &= clearbit[Z];}
void IORWFd0f005(void) {_W = _W|f[ 5]; if(_W==0) STATUS |=  setbit[Z]; else STATUS &= clearbit[Z];}
void IORWFd0f006(void) {_W = _W|f[ 6]; if(_W==0) STATUS |=  setbit[Z]; else STATUS &= clearbit[Z];}
void IORWFd0f007(void) {_W = _W|f[ 7]; if(_W==0) STATUS |=  setbit[Z]; else STATUS &= clearbit[Z];}
void IORWFd0f008(void) {_W = _W|f[ 8]; if(_W==0) STATUS |=  setbit[Z]; else STATUS &= clearbit[Z];}
void IORWFd0f009(void) {_W = _W|f[ 9]; if(_W==0) STATUS |=  setbit[Z]; else STATUS &= clearbit[Z];}
void IORWFd0f010(void) {_W = _W|f[10]; if(_W==0) STATUS |=  setbit[Z]; else STATUS &= clearbit[Z];}
void IORWFd0f011(void) {_W = _W|f[11]; if(_W==0) STATUS |=  setbit[Z]; else STATUS &= clearbit[Z];}
void IORWFd0f012(void) {_W = _W|f[12]; if(_W==0) STATUS |=  setbit[Z]; else STATUS &= clearbit[Z];}
void IORWFd0f013(void) {_W = _W|f[13]; if(_W==0) STATUS |=  setbit[Z]; else STATUS &= clearbit[Z];}
void IORWFd0f014(void) {_W = _W|f[14]; if(_W==0) STATUS |=  setbit[Z]; else STATUS &= clearbit[Z];}
void IORWFd0f015(void) {_W = _W|f[15]; if(_W==0) STATUS |=  setbit[Z]; else STATUS &= clearbit[Z];}
void IORWFd0f016(void) {_W = _W|f[16]; if(_W==0) STATUS |=  setbit[Z]; else STATUS &= clearbit[Z];}
void IORWFd0f017(void) {_W = _W|f[17]; if(_W==0) STATUS |=  setbit[Z]; else STATUS &= clearbit[Z];}
void IORWFd0f018(void) {_W = _W|f[18]; if(_W==0) STATUS |=  setbit[Z]; else STATUS &= clearbit[Z];}
void IORWFd0f019(void) {_W = _W|f[19]; if(_W==0) STATUS |=  setbit[Z]; else STATUS &= clearbit[Z];}
void IORWFd0f020(void) {_W = _W|f[20]; if(_W==0) STATUS |=  setbit[Z]; else STATUS &= clearbit[Z];}
void IORWFd0f021(void) {_W = _W|f[21]; if(_W==0) STATUS |=  setbit[Z]; else STATUS &= clearbit[Z];}
void IORWFd0f022(void) {_W = _W|f[22]; if(_W==0) STATUS |=  setbit[Z]; else STATUS &= clearbit[Z];}
void IORWFd0f023(void) {_W = _W|f[23]; if(_W==0) STATUS |=  setbit[Z]; else STATUS &= clearbit[Z];}
void IORWFd0f024(void) {_W = _W|f[24]; if(_W==0) STATUS |=  setbit[Z]; else STATUS &= clearbit[Z];}
void IORWFd0f025(void) {_W = _W|f[25]; if(_W==0) STATUS |=  setbit[Z]; else STATUS &= clearbit[Z];}
void IORWFd0f026(void) {_W = _W|f[26]; if(_W==0) STATUS |=  setbit[Z]; else STATUS &= clearbit[Z];}
void IORWFd0f027(void) {_W = _W|f[27]; if(_W==0) STATUS |=  setbit[Z]; else STATUS &= clearbit[Z];}
void IORWFd0f028(void) {_W = _W|f[28]; if(_W==0) STATUS |=  setbit[Z]; else STATUS &= clearbit[Z];}
void IORWFd0f029(void) {_W = _W|f[29]; if(_W==0) STATUS |=  setbit[Z]; else STATUS &= clearbit[Z];}
void IORWFd0f030(void) {_W = _W|f[30]; if(_W==0) STATUS |=  setbit[Z]; else STATUS &= clearbit[Z];}
void IORWFd0f031(void) {_W = _W|f[31]; if(_W==0) STATUS |=  setbit[Z]; else STATUS &= clearbit[Z];}

void IORWFd1f000(void) {INDR_ADDR = _W|INDR_ADDR; if(INDR_ADDR==0) STATUS |=  setbit[Z]; else STATUS &= clearbit[Z];}
void IORWFd1f001(void) {f[ 1] = _W|f[ 1]; if(f[ 1]==0) STATUS |=  setbit[Z]; else STATUS &= clearbit[Z];}
void IORWFd1f002(void) {f[ 2] = _W|f[ 2]; if(f[ 2]==0) STATUS |=  setbit[Z]; else STATUS &= clearbit[Z];}
void IORWFd1f003(void) {f[ 3] = _W|f[ 3]; if(f[ 3]==0) STATUS |=  setbit[Z]; else STATUS &= clearbit[Z];}
void IORWFd1f004(void) {f[ 4] = _W|f[ 4]; if(f[ 4]==0) STATUS |=  setbit[Z]; else STATUS &= clearbit[Z];}
void IORWFd1f005(void) {f[ 5] = _W|f[ 5]; if(f[ 5]==0) STATUS |=  setbit[Z]; else STATUS &= clearbit[Z];}
void IORWFd1f006(void) {f[ 6] = _W|f[ 6]; if(f[ 6]==0) STATUS |=  setbit[Z]; else STATUS &= clearbit[Z];}
void IORWFd1f007(void) {f[ 7] = _W|f[ 7]; if(f[ 7]==0) STATUS |=  setbit[Z]; else STATUS &= clearbit[Z];}
void IORWFd1f008(void) {f[ 8] = _W|f[ 8]; if(f[ 8]==0) STATUS |=  setbit[Z]; else STATUS &= clearbit[Z];}
void IORWFd1f009(void) {f[ 9] = _W|f[ 9]; if(f[ 9]==0) STATUS |=  setbit[Z]; else STATUS &= clearbit[Z];}
void IORWFd1f010(void) {f[10] = _W|f[10]; if(f[10]==0) STATUS |=  setbit[Z]; else STATUS &= clearbit[Z];}
void IORWFd1f011(void) {f[11] = _W|f[11]; if(f[11]==0) STATUS |=  setbit[Z]; else STATUS &= clearbit[Z];}
void IORWFd1f012(void) {f[12] = _W|f[12]; if(f[12]==0) STATUS |=  setbit[Z]; else STATUS &= clearbit[Z];}
void IORWFd1f013(void) {f[13] = _W|f[13]; if(f[13]==0) STATUS |=  setbit[Z]; else STATUS &= clearbit[Z];}
void IORWFd1f014(void) {f[14] = _W|f[14]; if(f[14]==0) STATUS |=  setbit[Z]; else STATUS &= clearbit[Z];}
void IORWFd1f015(void) {f[15] = _W|f[15]; if(f[15]==0) STATUS |=  setbit[Z]; else STATUS &= clearbit[Z];}
void IORWFd1f016(void) {f[16] = _W|f[16]; if(f[16]==0) STATUS |=  setbit[Z]; else STATUS &= clearbit[Z];}
void IORWFd1f017(void) {f[17] = _W|f[17]; if(f[17]==0) STATUS |=  setbit[Z]; else STATUS &= clearbit[Z];}
void IORWFd1f018(void) {f[18] = _W|f[18]; if(f[18]==0) STATUS |=  setbit[Z]; else STATUS &= clearbit[Z];}
void IORWFd1f019(void) {f[19] = _W|f[19]; if(f[19]==0) STATUS |=  setbit[Z]; else STATUS &= clearbit[Z];}
void IORWFd1f020(void) {f[20] = _W|f[20]; if(f[20]==0) STATUS |=  setbit[Z]; else STATUS &= clearbit[Z];}
void IORWFd1f021(void) {f[21] = _W|f[21]; if(f[21]==0) STATUS |=  setbit[Z]; else STATUS &= clearbit[Z];}
void IORWFd1f022(void) {f[22] = _W|f[22]; if(f[22]==0) STATUS |=  setbit[Z]; else STATUS &= clearbit[Z];}
void IORWFd1f023(void) {f[23] = _W|f[23]; if(f[23]==0) STATUS |=  setbit[Z]; else STATUS &= clearbit[Z];}
void IORWFd1f024(void) {f[24] = _W|f[24]; if(f[24]==0) STATUS |=  setbit[Z]; else STATUS &= clearbit[Z];}
void IORWFd1f025(void) {f[25] = _W|f[25]; if(f[25]==0) STATUS |=  setbit[Z]; else STATUS &= clearbit[Z];}
void IORWFd1f026(void) {f[26] = _W|f[26]; if(f[26]==0) STATUS |=  setbit[Z]; else STATUS &= clearbit[Z];}
void IORWFd1f027(void) {f[27] = _W|f[27]; if(f[27]==0) STATUS |=  setbit[Z]; else STATUS &= clearbit[Z];}
void IORWFd1f028(void) {f[28] = _W|f[28]; if(f[28]==0) STATUS |=  setbit[Z]; else STATUS &= clearbit[Z];}
void IORWFd1f029(void) {f[29] = _W|f[29]; if(f[29]==0) STATUS |=  setbit[Z]; else STATUS &= clearbit[Z];}
void IORWFd1f030(void) {f[30] = _W|f[30]; if(f[30]==0) STATUS |=  setbit[Z]; else STATUS &= clearbit[Z];}
void IORWFd1f031(void) {f[31] = _W|f[31]; if(f[31]==0) STATUS |=  setbit[Z]; else STATUS &= clearbit[Z];}