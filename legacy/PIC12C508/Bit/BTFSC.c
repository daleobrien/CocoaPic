/*
 *  BTFSC.c
 *  ToolTest
 *
 *  Created by Orla O'Brien on Sun May 12 2002.
 *  Copyright (c) 2001 __MyCompanyName__. All rights reserved.
 *
 */
#include "cpuMemory.h"
#include "BTFSC.h"



void BTFSCb0f000(void) {if ( INDR_ADDR ^ (1<<0)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb0f001(void) {if ( f[ 1] ^ (1<<0)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb0f002(void) {if ( f[ 2] ^ (1<<0)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb0f003(void) {if ( f[ 3] ^ (1<<0)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb0f004(void) {if ( f[ 4] ^ (1<<0)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb0f005(void) {if ( f[ 5] ^ (1<<0)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb0f006(void) {if ( f[ 6] ^ (1<<0)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb0f007(void) {if ( f[ 7] ^ (1<<0)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb0f008(void) {if ( f[ 8] ^ (1<<0)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb0f009(void) {if ( f[ 9] ^ (1<<0)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb0f010(void) {if ( f[10] ^ (1<<0)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb0f011(void) {if ( f[11] ^ (1<<0)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb0f012(void) {if ( f[12] ^ (1<<0)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb0f013(void) {if ( f[13] ^ (1<<0)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb0f014(void) {if ( f[14] ^ (1<<0)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb0f015(void) {if ( f[15] ^ (1<<0)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb0f016(void) {if ( f[16] ^ (1<<0)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb0f017(void) {if ( f[17] ^ (1<<0)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb0f018(void) {if ( f[18] ^ (1<<0)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb0f019(void) {if ( f[19] ^ (1<<0)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb0f020(void) {if ( f[20] ^ (1<<0)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb0f021(void) {if ( f[21] ^ (1<<0)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb0f022(void) {if ( f[22] ^ (1<<0)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb0f023(void) {if ( f[23] ^ (1<<0)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb0f024(void) {if ( f[24] ^ (1<<0)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb0f025(void) {if ( f[25] ^ (1<<0)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb0f026(void) {if ( f[26] ^ (1<<0)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb0f027(void) {if ( f[27] ^ (1<<0)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb0f028(void) {if ( f[28] ^ (1<<0)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb0f029(void) {if ( f[29] ^ (1<<0)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb0f030(void) {if ( f[30] ^ (1<<0)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb0f031(void) {if ( f[31] ^ (1<<0)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb1f000(void) {if ( INDR_ADDR ^ (1<<1)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb1f001(void) {if ( f[ 1] ^ (1<<1)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb1f002(void) {if ( f[ 2] ^ (1<<1)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb1f003(void) {if ( f[ 3] ^ (1<<1)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb1f004(void) {if ( f[ 4] ^ (1<<1)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb1f005(void) {if ( f[ 5] ^ (1<<1)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb1f006(void) {if ( f[ 6] ^ (1<<1)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb1f007(void) {if ( f[ 7] ^ (1<<1)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb1f008(void) {if ( f[ 8] ^ (1<<1)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb1f009(void) {if ( f[ 9] ^ (1<<1)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb1f010(void) {if ( f[10] ^ (1<<1)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb1f011(void) {if ( f[11] ^ (1<<1)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb1f012(void) {if ( f[12] ^ (1<<1)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb1f013(void) {if ( f[13] ^ (1<<1)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb1f014(void) {if ( f[14] ^ (1<<1)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb1f015(void) {if ( f[15] ^ (1<<1)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb1f016(void) {if ( f[16] ^ (1<<1)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb1f017(void) {if ( f[17] ^ (1<<1)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb1f018(void) {if ( f[18] ^ (1<<1)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb1f019(void) {if ( f[19] ^ (1<<1)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb1f020(void) {if ( f[20] ^ (1<<1)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb1f021(void) {if ( f[21] ^ (1<<1)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb1f022(void) {if ( f[22] ^ (1<<1)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb1f023(void) {if ( f[23] ^ (1<<1)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb1f024(void) {if ( f[24] ^ (1<<1)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb1f025(void) {if ( f[25] ^ (1<<1)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb1f026(void) {if ( f[26] ^ (1<<1)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb1f027(void) {if ( f[27] ^ (1<<1)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb1f028(void) {if ( f[28] ^ (1<<1)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb1f029(void) {if ( f[29] ^ (1<<1)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb1f030(void) {if ( f[30] ^ (1<<1)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb1f031(void) {if ( f[31] ^ (1<<1)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb2f000(void) {if ( INDR_ADDR ^ (1<<2)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb2f001(void) {if ( f[ 1] ^ (1<<2)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb2f002(void) {if ( f[ 2] ^ (1<<2)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb2f003(void) {if ( f[ 3] ^ (1<<2)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb2f004(void) {if ( f[ 4] ^ (1<<2)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb2f005(void) {if ( f[ 5] ^ (1<<2)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb2f006(void) {if ( f[ 6] ^ (1<<2)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb2f007(void) {if ( f[ 7] ^ (1<<2)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb2f008(void) {if ( f[ 8] ^ (1<<2)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb2f009(void) {if ( f[ 9] ^ (1<<2)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb2f010(void) {if ( f[10] ^ (1<<2)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb2f011(void) {if ( f[11] ^ (1<<2)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb2f012(void) {if ( f[12] ^ (1<<2)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb2f013(void) {if ( f[13] ^ (1<<2)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb2f014(void) {if ( f[14] ^ (1<<2)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb2f015(void) {if ( f[15] ^ (1<<2)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb2f016(void) {if ( f[16] ^ (1<<2)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb2f017(void) {if ( f[17] ^ (1<<2)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb2f018(void) {if ( f[18] ^ (1<<2)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb2f019(void) {if ( f[19] ^ (1<<2)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb2f020(void) {if ( f[20] ^ (1<<2)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb2f021(void) {if ( f[21] ^ (1<<2)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb2f022(void) {if ( f[22] ^ (1<<2)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb2f023(void) {if ( f[23] ^ (1<<2)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb2f024(void) {if ( f[24] ^ (1<<2)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb2f025(void) {if ( f[25] ^ (1<<2)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb2f026(void) {if ( f[26] ^ (1<<2)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb2f027(void) {if ( f[27] ^ (1<<2)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb2f028(void) {if ( f[28] ^ (1<<2)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb2f029(void) {if ( f[29] ^ (1<<2)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb2f030(void) {if ( f[30] ^ (1<<2)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb2f031(void) {if ( f[31] ^ (1<<2)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb3f000(void) {if ( INDR_ADDR ^ (1<<3)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb3f001(void) {if ( f[ 1] ^ (1<<3)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb3f002(void) {if ( f[ 2] ^ (1<<3)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb3f003(void) {if ( f[ 3] ^ (1<<3)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb3f004(void) {if ( f[ 4] ^ (1<<3)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb3f005(void) {if ( f[ 5] ^ (1<<3)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb3f006(void) {if ( f[ 6] ^ (1<<3)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb3f007(void) {if ( f[ 7] ^ (1<<3)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb3f008(void) {if ( f[ 8] ^ (1<<3)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb3f009(void) {if ( f[ 9] ^ (1<<3)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb3f010(void) {if ( f[10] ^ (1<<3)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb3f011(void) {if ( f[11] ^ (1<<3)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb3f012(void) {if ( f[12] ^ (1<<3)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb3f013(void) {if ( f[13] ^ (1<<3)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb3f014(void) {if ( f[14] ^ (1<<3)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb3f015(void) {if ( f[15] ^ (1<<3)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb3f016(void) {if ( f[16] ^ (1<<3)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb3f017(void) {if ( f[17] ^ (1<<3)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb3f018(void) {if ( f[18] ^ (1<<3)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb3f019(void) {if ( f[19] ^ (1<<3)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb3f020(void) {if ( f[20] ^ (1<<3)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb3f021(void) {if ( f[21] ^ (1<<3)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb3f022(void) {if ( f[22] ^ (1<<3)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb3f023(void) {if ( f[23] ^ (1<<3)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb3f024(void) {if ( f[24] ^ (1<<3)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb3f025(void) {if ( f[25] ^ (1<<3)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb3f026(void) {if ( f[26] ^ (1<<3)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb3f027(void) {if ( f[27] ^ (1<<3)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb3f028(void) {if ( f[28] ^ (1<<3)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb3f029(void) {if ( f[29] ^ (1<<3)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb3f030(void) {if ( f[30] ^ (1<<3)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb3f031(void) {if ( f[31] ^ (1<<3)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb4f000(void) {if ( INDR_ADDR ^ (1<<4)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb4f001(void) {if ( f[ 1] ^ (1<<4)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb4f002(void) {if ( f[ 2] ^ (1<<4)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb4f003(void) {if ( f[ 3] ^ (1<<4)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb4f004(void) {if ( f[ 4] ^ (1<<4)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb4f005(void) {if ( f[ 5] ^ (1<<4)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb4f006(void) {if ( f[ 6] ^ (1<<4)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb4f007(void) {if ( f[ 7] ^ (1<<4)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb4f008(void) {if ( f[ 8] ^ (1<<4)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb4f009(void) {if ( f[ 9] ^ (1<<4)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb4f010(void) {if ( f[10] ^ (1<<4)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb4f011(void) {if ( f[11] ^ (1<<4)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb4f012(void) {if ( f[12] ^ (1<<4)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb4f013(void) {if ( f[13] ^ (1<<4)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb4f014(void) {if ( f[14] ^ (1<<4)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb4f015(void) {if ( f[15] ^ (1<<4)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb4f016(void) {if ( f[16] ^ (1<<4)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb4f017(void) {if ( f[17] ^ (1<<4)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb4f018(void) {if ( f[18] ^ (1<<4)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb4f019(void) {if ( f[19] ^ (1<<4)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb4f020(void) {if ( f[20] ^ (1<<4)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb4f021(void) {if ( f[21] ^ (1<<4)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb4f022(void) {if ( f[22] ^ (1<<4)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb4f023(void) {if ( f[23] ^ (1<<4)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb4f024(void) {if ( f[24] ^ (1<<4)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb4f025(void) {if ( f[25] ^ (1<<4)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb4f026(void) {if ( f[26] ^ (1<<4)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb4f027(void) {if ( f[27] ^ (1<<4)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb4f028(void) {if ( f[28] ^ (1<<4)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb4f029(void) {if ( f[29] ^ (1<<4)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb4f030(void) {if ( f[30] ^ (1<<4)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb4f031(void) {if ( f[31] ^ (1<<4)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb5f000(void) {if ( INDR_ADDR ^ (1<<5)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb5f001(void) {if ( f[ 1] ^ (1<<5)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb5f002(void) {if ( f[ 2] ^ (1<<5)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb5f003(void) {if ( f[ 3] ^ (1<<5)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb5f004(void) {if ( f[ 4] ^ (1<<5)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb5f005(void) {if ( f[ 5] ^ (1<<5)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb5f006(void) {if ( f[ 6] ^ (1<<5)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb5f007(void) {if ( f[ 7] ^ (1<<5)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb5f008(void) {if ( f[ 8] ^ (1<<5)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb5f009(void) {if ( f[ 9] ^ (1<<5)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb5f010(void) {if ( f[10] ^ (1<<5)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb5f011(void) {if ( f[11] ^ (1<<5)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb5f012(void) {if ( f[12] ^ (1<<5)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb5f013(void) {if ( f[13] ^ (1<<5)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb5f014(void) {if ( f[14] ^ (1<<5)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb5f015(void) {if ( f[15] ^ (1<<5)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb5f016(void) {if ( f[16] ^ (1<<5)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb5f017(void) {if ( f[17] ^ (1<<5)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb5f018(void) {if ( f[18] ^ (1<<5)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb5f019(void) {if ( f[19] ^ (1<<5)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb5f020(void) {if ( f[20] ^ (1<<5)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb5f021(void) {if ( f[21] ^ (1<<5)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb5f022(void) {if ( f[22] ^ (1<<5)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb5f023(void) {if ( f[23] ^ (1<<5)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb5f024(void) {if ( f[24] ^ (1<<5)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb5f025(void) {if ( f[25] ^ (1<<5)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb5f026(void) {if ( f[26] ^ (1<<5)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb5f027(void) {if ( f[27] ^ (1<<5)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb5f028(void) {if ( f[28] ^ (1<<5)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb5f029(void) {if ( f[29] ^ (1<<5)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb5f030(void) {if ( f[30] ^ (1<<5)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb5f031(void) {if ( f[31] ^ (1<<5)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb6f000(void) {if ( INDR_ADDR ^ (1<<6)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb6f001(void) {if ( f[ 1] ^ (1<<6)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb6f002(void) {if ( f[ 2] ^ (1<<6)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb6f003(void) {if ( f[ 3] ^ (1<<6)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb6f004(void) {if ( f[ 4] ^ (1<<6)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb6f005(void) {if ( f[ 5] ^ (1<<6)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb6f006(void) {if ( f[ 6] ^ (1<<6)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb6f007(void) {if ( f[ 7] ^ (1<<6)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb6f008(void) {if ( f[ 8] ^ (1<<6)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb6f009(void) {if ( f[ 9] ^ (1<<6)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb6f010(void) {if ( f[10] ^ (1<<6)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb6f011(void) {if ( f[11] ^ (1<<6)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb6f012(void) {if ( f[12] ^ (1<<6)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb6f013(void) {if ( f[13] ^ (1<<6)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb6f014(void) {if ( f[14] ^ (1<<6)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb6f015(void) {if ( f[15] ^ (1<<6)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb6f016(void) {if ( f[16] ^ (1<<6)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb6f017(void) {if ( f[17] ^ (1<<6)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb6f018(void) {if ( f[18] ^ (1<<6)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb6f019(void) {if ( f[19] ^ (1<<6)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb6f020(void) {if ( f[20] ^ (1<<6)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb6f021(void) {if ( f[21] ^ (1<<6)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb6f022(void) {if ( f[22] ^ (1<<6)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb6f023(void) {if ( f[23] ^ (1<<6)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb6f024(void) {if ( f[24] ^ (1<<6)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb6f025(void) {if ( f[25] ^ (1<<6)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb6f026(void) {if ( f[26] ^ (1<<6)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb6f027(void) {if ( f[27] ^ (1<<6)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb6f028(void) {if ( f[28] ^ (1<<6)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb6f029(void) {if ( f[29] ^ (1<<6)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb6f030(void) {if ( f[30] ^ (1<<6)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb6f031(void) {if ( f[31] ^ (1<<6)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb7f000(void) {if ( INDR_ADDR ^ (1<<7)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb7f001(void) {if ( f[ 1] ^ (1<<7)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb7f002(void) {if ( f[ 2] ^ (1<<7)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb7f003(void) {if ( f[ 3] ^ (1<<7)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb7f004(void) {if ( f[ 4] ^ (1<<7)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb7f005(void) {if ( f[ 5] ^ (1<<7)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb7f006(void) {if ( f[ 6] ^ (1<<7)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb7f007(void) {if ( f[ 7] ^ (1<<7)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb7f008(void) {if ( f[ 8] ^ (1<<7)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb7f009(void) {if ( f[ 9] ^ (1<<7)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb7f010(void) {if ( f[10] ^ (1<<7)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb7f011(void) {if ( f[11] ^ (1<<7)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb7f012(void) {if ( f[12] ^ (1<<7)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb7f013(void) {if ( f[13] ^ (1<<7)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb7f014(void) {if ( f[14] ^ (1<<7)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb7f015(void) {if ( f[15] ^ (1<<7)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb7f016(void) {if ( f[16] ^ (1<<7)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb7f017(void) {if ( f[17] ^ (1<<7)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb7f018(void) {if ( f[18] ^ (1<<7)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb7f019(void) {if ( f[19] ^ (1<<7)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb7f020(void) {if ( f[20] ^ (1<<7)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb7f021(void) {if ( f[21] ^ (1<<7)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb7f022(void) {if ( f[22] ^ (1<<7)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb7f023(void) {if ( f[23] ^ (1<<7)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb7f024(void) {if ( f[24] ^ (1<<7)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb7f025(void) {if ( f[25] ^ (1<<7)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb7f026(void) {if ( f[26] ^ (1<<7)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb7f027(void) {if ( f[27] ^ (1<<7)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb7f028(void) {if ( f[28] ^ (1<<7)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb7f029(void) {if ( f[29] ^ (1<<7)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb7f030(void) {if ( f[30] ^ (1<<7)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSCb7f031(void) {if ( f[31] ^ (1<<7)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}