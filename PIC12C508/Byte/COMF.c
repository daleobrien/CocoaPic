/*
 *  COMF.c
 *  ToolTest
 *
 *  Created by Orla O'Brien on Sun May 12 2002.
 *  Copyright (c) 2001 __MyCompanyName__. All rights reserved.
 *
 */
#include "cpuMemory.h"
#include "COMF.h"

void COMFd0f000(void) {_W = 0x00ff & (!INDR_ADDR); if(_W !=0) STATUS |= setbit[Z]; else STATUS &= clearbit[Z];}
void COMFd0f001(void) {_W = 0x00ff & (!f[ 1]); if(_W !=0) STATUS |= setbit[Z]; else STATUS &= clearbit[Z];}
void COMFd0f002(void) {_W = 0x00ff & (!f[ 2]); if(_W !=0) STATUS |= setbit[Z]; else STATUS &= clearbit[Z];}
void COMFd0f003(void) {_W = 0x00ff & (!f[ 3]); if(_W !=0) STATUS |= setbit[Z]; else STATUS &= clearbit[Z];}
void COMFd0f004(void) {_W = 0x00ff & (!f[ 4]); if(_W !=0) STATUS |= setbit[Z]; else STATUS &= clearbit[Z];}
void COMFd0f005(void) {_W = 0x00ff & (!f[ 5]); if(_W !=0) STATUS |= setbit[Z]; else STATUS &= clearbit[Z];}
void COMFd0f006(void) {_W = 0x00ff & (!f[ 6]); if(_W !=0) STATUS |= setbit[Z]; else STATUS &= clearbit[Z];}
void COMFd0f007(void) {_W = 0x00ff & (!f[ 7]); if(_W !=0) STATUS |= setbit[Z]; else STATUS &= clearbit[Z];}
void COMFd0f008(void) {_W = 0x00ff & (!f[ 8]); if(_W !=0) STATUS |= setbit[Z]; else STATUS &= clearbit[Z];}
void COMFd0f009(void) {_W = 0x00ff & (!f[ 9]); if(_W !=0) STATUS |= setbit[Z]; else STATUS &= clearbit[Z];}
void COMFd0f010(void) {_W = 0x00ff & (!f[10]); if(_W !=0) STATUS |= setbit[Z]; else STATUS &= clearbit[Z];}
void COMFd0f011(void) {_W = 0x00ff & (!f[11]); if(_W !=0) STATUS |= setbit[Z]; else STATUS &= clearbit[Z];}
void COMFd0f012(void) {_W = 0x00ff & (!f[12]); if(_W !=0) STATUS |= setbit[Z]; else STATUS &= clearbit[Z];}
void COMFd0f013(void) {_W = 0x00ff & (!f[13]); if(_W !=0) STATUS |= setbit[Z]; else STATUS &= clearbit[Z];}
void COMFd0f014(void) {_W = 0x00ff & (!f[14]); if(_W !=0) STATUS |= setbit[Z]; else STATUS &= clearbit[Z];}
void COMFd0f015(void) {_W = 0x00ff & (!f[15]); if(_W !=0) STATUS |= setbit[Z]; else STATUS &= clearbit[Z];}
void COMFd0f016(void) {_W = 0x00ff & (!f[16]); if(_W !=0) STATUS |= setbit[Z]; else STATUS &= clearbit[Z];}
void COMFd0f017(void) {_W = 0x00ff & (!f[17]); if(_W !=0) STATUS |= setbit[Z]; else STATUS &= clearbit[Z];}
void COMFd0f018(void) {_W = 0x00ff & (!f[18]); if(_W !=0) STATUS |= setbit[Z]; else STATUS &= clearbit[Z];}
void COMFd0f019(void) {_W = 0x00ff & (!f[19]); if(_W !=0) STATUS |= setbit[Z]; else STATUS &= clearbit[Z];}
void COMFd0f020(void) {_W = 0x00ff & (!f[20]); if(_W !=0) STATUS |= setbit[Z]; else STATUS &= clearbit[Z];}
void COMFd0f021(void) {_W = 0x00ff & (!f[21]); if(_W !=0) STATUS |= setbit[Z]; else STATUS &= clearbit[Z];}
void COMFd0f022(void) {_W = 0x00ff & (!f[22]); if(_W !=0) STATUS |= setbit[Z]; else STATUS &= clearbit[Z];}
void COMFd0f023(void) {_W = 0x00ff & (!f[23]); if(_W !=0) STATUS |= setbit[Z]; else STATUS &= clearbit[Z];}
void COMFd0f024(void) {_W = 0x00ff & (!f[24]); if(_W !=0) STATUS |= setbit[Z]; else STATUS &= clearbit[Z];}
void COMFd0f025(void) {_W = 0x00ff & (!f[25]); if(_W !=0) STATUS |= setbit[Z]; else STATUS &= clearbit[Z];}
void COMFd0f026(void) {_W = 0x00ff & (!f[26]); if(_W !=0) STATUS |= setbit[Z]; else STATUS &= clearbit[Z];}
void COMFd0f027(void) {_W = 0x00ff & (!f[27]); if(_W !=0) STATUS |= setbit[Z]; else STATUS &= clearbit[Z];}
void COMFd0f028(void) {_W = 0x00ff & (!f[28]); if(_W !=0) STATUS |= setbit[Z]; else STATUS &= clearbit[Z];}
void COMFd0f029(void) {_W = 0x00ff & (!f[29]); if(_W !=0) STATUS |= setbit[Z]; else STATUS &= clearbit[Z];}
void COMFd0f030(void) {_W = 0x00ff & (!f[30]); if(_W !=0) STATUS |= setbit[Z]; else STATUS &= clearbit[Z];}
void COMFd0f031(void) {_W = 0x00ff & (!f[31]); if(_W !=0) STATUS |= setbit[Z]; else STATUS &= clearbit[Z];}
void COMFd1f000(void) {INDR_ADDR = 0x00ff & (!INDR_ADDR); if(INDR_ADDR !=0) STATUS |= setbit[Z]; else STATUS &= clearbit[Z];}
void COMFd1f001(void) {f[ 1] = 0x00ff & (!f[ 1]); if(f[ 1] !=0) STATUS |= setbit[Z]; else STATUS &= clearbit[Z];}
void COMFd1f002(void) {f[ 2] = 0x00ff & (!f[ 2]); if(f[ 2] !=0) STATUS |= setbit[Z]; else STATUS &= clearbit[Z];}
void COMFd1f003(void) {f[ 3] = 0x00ff & (!f[ 3]); if(f[ 3] !=0) STATUS |= setbit[Z]; else STATUS &= clearbit[Z];}
void COMFd1f004(void) {f[ 4] = 0x00ff & (!f[ 4]); if(f[ 4] !=0) STATUS |= setbit[Z]; else STATUS &= clearbit[Z];}
void COMFd1f005(void) {f[ 5] = 0x00ff & (!f[ 5]); if(f[ 5] !=0) STATUS |= setbit[Z]; else STATUS &= clearbit[Z];}
void COMFd1f006(void) {f[ 6] = 0x00ff & (!f[ 6]); if(f[ 6] !=0) STATUS |= setbit[Z]; else STATUS &= clearbit[Z];}
void COMFd1f007(void) {f[ 7] = 0x00ff & (!f[ 7]); if(f[ 7] !=0) STATUS |= setbit[Z]; else STATUS &= clearbit[Z];}
void COMFd1f008(void) {f[ 8] = 0x00ff & (!f[ 8]); if(f[ 8] !=0) STATUS |= setbit[Z]; else STATUS &= clearbit[Z];}
void COMFd1f009(void) {f[ 9] = 0x00ff & (!f[ 9]); if(f[ 9] !=0) STATUS |= setbit[Z]; else STATUS &= clearbit[Z];}
void COMFd1f010(void) {f[10] = 0x00ff & (!f[10]); if(f[10] !=0) STATUS |= setbit[Z]; else STATUS &= clearbit[Z];}
void COMFd1f011(void) {f[11] = 0x00ff & (!f[11]); if(f[11] !=0) STATUS |= setbit[Z]; else STATUS &= clearbit[Z];}
void COMFd1f012(void) {f[12] = 0x00ff & (!f[12]); if(f[12] !=0) STATUS |= setbit[Z]; else STATUS &= clearbit[Z];}
void COMFd1f013(void) {f[13] = 0x00ff & (!f[13]); if(f[13] !=0) STATUS |= setbit[Z]; else STATUS &= clearbit[Z];}
void COMFd1f014(void) {f[14] = 0x00ff & (!f[14]); if(f[14] !=0) STATUS |= setbit[Z]; else STATUS &= clearbit[Z];}
void COMFd1f015(void) {f[15] = 0x00ff & (!f[15]); if(f[15] !=0) STATUS |= setbit[Z]; else STATUS &= clearbit[Z];}
void COMFd1f016(void) {f[16] = 0x00ff & (!f[16]); if(f[16] !=0) STATUS |= setbit[Z]; else STATUS &= clearbit[Z];}
void COMFd1f017(void) {f[17] = 0x00ff & (!f[17]); if(f[17] !=0) STATUS |= setbit[Z]; else STATUS &= clearbit[Z];}
void COMFd1f018(void) {f[18] = 0x00ff & (!f[18]); if(f[18] !=0) STATUS |= setbit[Z]; else STATUS &= clearbit[Z];}
void COMFd1f019(void) {f[19] = 0x00ff & (!f[19]); if(f[19] !=0) STATUS |= setbit[Z]; else STATUS &= clearbit[Z];}
void COMFd1f020(void) {f[20] = 0x00ff & (!f[20]); if(f[20] !=0) STATUS |= setbit[Z]; else STATUS &= clearbit[Z];}
void COMFd1f021(void) {f[21] = 0x00ff & (!f[21]); if(f[21] !=0) STATUS |= setbit[Z]; else STATUS &= clearbit[Z];}
void COMFd1f022(void) {f[22] = 0x00ff & (!f[22]); if(f[22] !=0) STATUS |= setbit[Z]; else STATUS &= clearbit[Z];}
void COMFd1f023(void) {f[23] = 0x00ff & (!f[23]); if(f[23] !=0) STATUS |= setbit[Z]; else STATUS &= clearbit[Z];}
void COMFd1f024(void) {f[24] = 0x00ff & (!f[24]); if(f[24] !=0) STATUS |= setbit[Z]; else STATUS &= clearbit[Z];}
void COMFd1f025(void) {f[25] = 0x00ff & (!f[25]); if(f[25] !=0) STATUS |= setbit[Z]; else STATUS &= clearbit[Z];}
void COMFd1f026(void) {f[26] = 0x00ff & (!f[26]); if(f[26] !=0) STATUS |= setbit[Z]; else STATUS &= clearbit[Z];}
void COMFd1f027(void) {f[27] = 0x00ff & (!f[27]); if(f[27] !=0) STATUS |= setbit[Z]; else STATUS &= clearbit[Z];}
void COMFd1f028(void) {f[28] = 0x00ff & (!f[28]); if(f[28] !=0) STATUS |= setbit[Z]; else STATUS &= clearbit[Z];}
void COMFd1f029(void) {f[29] = 0x00ff & (!f[29]); if(f[29] !=0) STATUS |= setbit[Z]; else STATUS &= clearbit[Z];}
void COMFd1f030(void) {f[30] = 0x00ff & (!f[30]); if(f[30] !=0) STATUS |= setbit[Z]; else STATUS &= clearbit[Z];}
void COMFd1f031(void) {f[31] = 0x00ff & (!f[31]); if(f[31] !=0) STATUS |= setbit[Z]; else STATUS &= clearbit[Z];}