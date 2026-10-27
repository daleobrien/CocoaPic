/*
 *  SWAPF.c
 *  ToolTest
 *
 *  Created by Orla O'Brien on Sun May 12 2002.
 *  Copyright (c) 2001 __MyCompanyName__. All rights reserved.
 *
 */
#include "cpuMemory.h"
#include "SWAPF.h"

void SWAPFd0f000(void) {_W=(INDR_ADDR>>4) | (INDR_ADDR<<4);}
void SWAPFd0f001(void) {_W=(f[ 1]>>4) | (f[ 1]<<4);}
void SWAPFd0f002(void) {_W=(f[ 2]>>4) | (f[ 2]<<4);}
void SWAPFd0f003(void) {_W=(f[ 3]>>4) | (f[ 3]<<4);}
void SWAPFd0f004(void) {_W=(f[ 4]>>4) | (f[ 4]<<4);}
void SWAPFd0f005(void) {_W=(f[ 5]>>4) | (f[ 5]<<4);}
void SWAPFd0f006(void) {_W=(f[ 6]>>4) | (f[ 6]<<4);}
void SWAPFd0f007(void) {_W=(f[ 7]>>4) | (f[ 7]<<4);}
void SWAPFd0f008(void) {_W=(f[ 8]>>4) | (f[ 8]<<4);}
void SWAPFd0f009(void) {_W=(f[ 9]>>4) | (f[ 9]<<4);}
void SWAPFd0f010(void) {_W=(f[10]>>4) | (f[10]<<4);}
void SWAPFd0f011(void) {_W=(f[11]>>4) | (f[11]<<4);}
void SWAPFd0f012(void) {_W=(f[12]>>4) | (f[12]<<4);}
void SWAPFd0f013(void) {_W=(f[13]>>4) | (f[13]<<4);}
void SWAPFd0f014(void) {_W=(f[14]>>4) | (f[14]<<4);}
void SWAPFd0f015(void) {_W=(f[15]>>4) | (f[15]<<4);}
void SWAPFd0f016(void) {_W=(f[16]>>4) | (f[16]<<4);}
void SWAPFd0f017(void) {_W=(f[17]>>4) | (f[17]<<4);}
void SWAPFd0f018(void) {_W=(f[18]>>4) | (f[18]<<4);}
void SWAPFd0f019(void) {_W=(f[19]>>4) | (f[19]<<4);}
void SWAPFd0f020(void) {_W=(f[20]>>4) | (f[20]<<4);}
void SWAPFd0f021(void) {_W=(f[21]>>4) | (f[21]<<4);}
void SWAPFd0f022(void) {_W=(f[22]>>4) | (f[22]<<4);}
void SWAPFd0f023(void) {_W=(f[23]>>4) | (f[23]<<4);}
void SWAPFd0f024(void) {_W=(f[24]>>4) | (f[24]<<4);}
void SWAPFd0f025(void) {_W=(f[25]>>4) | (f[25]<<4);}
void SWAPFd0f026(void) {_W=(f[26]>>4) | (f[26]<<4);}
void SWAPFd0f027(void) {_W=(f[27]>>4) | (f[27]<<4);}
void SWAPFd0f028(void) {_W=(f[28]>>4) | (f[28]<<4);}
void SWAPFd0f029(void) {_W=(f[29]>>4) | (f[29]<<4);}
void SWAPFd0f030(void) {_W=(f[30]>>4) | (f[30]<<4);}
void SWAPFd0f031(void) {_W=(f[31]>>4) | (f[31]<<4);}
void SWAPFd1f000(void) {INDR_ADDR=(INDR_ADDR>>4) | (INDR_ADDR<<4);}
void SWAPFd1f001(void) {f[ 1]=(f[ 1]>>4) | (f[ 1]<<4);}
void SWAPFd1f002(void) {f[ 2]=(f[ 2]>>4) | (f[ 2]<<4);}
void SWAPFd1f003(void) {f[ 3]=(f[ 3]>>4) | (f[ 3]<<4);}
void SWAPFd1f004(void) {f[ 4]=(f[ 4]>>4) | (f[ 4]<<4);}
void SWAPFd1f005(void) {f[ 5]=(f[ 5]>>4) | (f[ 5]<<4);}
void SWAPFd1f006(void) {f[ 6]=(f[ 6]>>4) | (f[ 6]<<4);}
void SWAPFd1f007(void) {f[ 7]=(f[ 7]>>4) | (f[ 7]<<4);}
void SWAPFd1f008(void) {f[ 8]=(f[ 8]>>4) | (f[ 8]<<4);}
void SWAPFd1f009(void) {f[ 9]=(f[ 9]>>4) | (f[ 9]<<4);}
void SWAPFd1f010(void) {f[10]=(f[10]>>4) | (f[10]<<4);}
void SWAPFd1f011(void) {f[11]=(f[11]>>4) | (f[11]<<4);}
void SWAPFd1f012(void) {f[12]=(f[12]>>4) | (f[12]<<4);}
void SWAPFd1f013(void) {f[13]=(f[13]>>4) | (f[13]<<4);}
void SWAPFd1f014(void) {f[14]=(f[14]>>4) | (f[14]<<4);}
void SWAPFd1f015(void) {f[15]=(f[15]>>4) | (f[15]<<4);}
void SWAPFd1f016(void) {f[16]=(f[16]>>4) | (f[16]<<4);}
void SWAPFd1f017(void) {f[17]=(f[17]>>4) | (f[17]<<4);}
void SWAPFd1f018(void) {f[18]=(f[18]>>4) | (f[18]<<4);}
void SWAPFd1f019(void) {f[19]=(f[19]>>4) | (f[19]<<4);}
void SWAPFd1f020(void) {f[20]=(f[20]>>4) | (f[20]<<4);}
void SWAPFd1f021(void) {f[21]=(f[21]>>4) | (f[21]<<4);}
void SWAPFd1f022(void) {f[22]=(f[22]>>4) | (f[22]<<4);}
void SWAPFd1f023(void) {f[23]=(f[23]>>4) | (f[23]<<4);}
void SWAPFd1f024(void) {f[24]=(f[24]>>4) | (f[24]<<4);}
void SWAPFd1f025(void) {f[25]=(f[25]>>4) | (f[25]<<4);}
void SWAPFd1f026(void) {f[26]=(f[26]>>4) | (f[26]<<4);}
void SWAPFd1f027(void) {f[27]=(f[27]>>4) | (f[27]<<4);}
void SWAPFd1f028(void) {f[28]=(f[28]>>4) | (f[28]<<4);}
void SWAPFd1f029(void) {f[29]=(f[29]>>4) | (f[29]<<4);}
void SWAPFd1f030(void) {f[30]=(f[30]>>4) | (f[30]<<4);}
void SWAPFd1f031(void) {f[31]=(f[31]>>4) | (f[31]<<4);}