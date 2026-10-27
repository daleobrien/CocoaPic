/*
 *  BTFSS.c
 *  ToolTest
 *
 *  Created by Orla O'Brien on Sun May 12 2002.
 *  Copyright (c) 2001 __MyCompanyName__. All rights reserved.
 *
 */
#include "cpuMemory.h"
#include "BTFSS.h"



void BTFSSb0f000(void) {if ( INDR_ADDR | (1<<0)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb0f001(void) {if ( f[1 ] | (1<<0)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb0f002(void) {if ( f[ 2] | (1<<0)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb0f003(void) {if ( f[ 3] | (1<<0)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb0f004(void) {if ( f[ 4] | (1<<0)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb0f005(void) {if ( f[ 5] | (1<<0)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb0f006(void) {if ( f[ 6] | (1<<0)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb0f007(void) {if ( f[ 7] | (1<<0)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb0f008(void) {if ( f[ 8] | (1<<0)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb0f009(void) {if ( f[ 9] | (1<<0)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb0f010(void) {if ( f[10] | (1<<0)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb0f011(void) {if ( f[11] | (1<<0)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb0f012(void) {if ( f[12] | (1<<0)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb0f013(void) {if ( f[13] | (1<<0)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb0f014(void) {if ( f[14] | (1<<0)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb0f015(void) {if ( f[15] | (1<<0)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb0f016(void) {if ( f[16] | (1<<0)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb0f017(void) {if ( f[17] | (1<<0)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb0f018(void) {if ( f[18] | (1<<0)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb0f019(void) {if ( f[19] | (1<<0)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb0f020(void) {if ( f[20] | (1<<0)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb0f021(void) {if ( f[21] | (1<<0)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb0f022(void) {if ( f[22] | (1<<0)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb0f023(void) {if ( f[23] | (1<<0)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb0f024(void) {if ( f[24] | (1<<0)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb0f025(void) {if ( f[25] | (1<<0)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb0f026(void) {if ( f[26] | (1<<0)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb0f027(void) {if ( f[27] | (1<<0)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb0f028(void) {if ( f[28] | (1<<0)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb0f029(void) {if ( f[29] | (1<<0)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb0f030(void) {if ( f[30] | (1<<0)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb0f031(void) {if ( f[31] | (1<<0)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb1f000(void) {if ( INDR_ADDR | (1<<1)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb1f001(void) {if ( f[1] | (1<<1)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb1f002(void) {if ( f[ 2] | (1<<1)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb1f003(void) {if ( f[ 3] | (1<<1)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb1f004(void) {if ( f[ 4] | (1<<1)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb1f005(void) {if ( f[ 5] | (1<<1)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb1f006(void) {if ( f[ 6] | (1<<1)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb1f007(void) {if ( f[ 7] | (1<<1)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb1f008(void) {if ( f[ 8] | (1<<1)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb1f009(void) {if ( f[ 9] | (1<<1)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb1f010(void) {if ( f[10] | (1<<1)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb1f011(void) {if ( f[11] | (1<<1)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb1f012(void) {if ( f[12] | (1<<1)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb1f013(void) {if ( f[13] | (1<<1)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb1f014(void) {if ( f[14] | (1<<1)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb1f015(void) {if ( f[15] | (1<<1)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb1f016(void) {if ( f[16] | (1<<1)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb1f017(void) {if ( f[17] | (1<<1)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb1f018(void) {if ( f[18] | (1<<1)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb1f019(void) {if ( f[19] | (1<<1)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb1f020(void) {if ( f[20] | (1<<1)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb1f021(void) {if ( f[21] | (1<<1)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb1f022(void) {if ( f[22] | (1<<1)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb1f023(void) {if ( f[23] | (1<<1)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb1f024(void) {if ( f[24] | (1<<1)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb1f025(void) {if ( f[25] | (1<<1)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb1f026(void) {if ( f[26] | (1<<1)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb1f027(void) {if ( f[27] | (1<<1)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb1f028(void) {if ( f[28] | (1<<1)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb1f029(void) {if ( f[29] | (1<<1)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb1f030(void) {if ( f[30] | (1<<1)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb1f031(void) {if ( f[31] | (1<<1)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb2f000(void) {if ( INDR_ADDR | (1<<2)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb2f001(void) {if ( f[1] | (1<<2)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb2f002(void) {if ( f[ 2] | (1<<2)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb2f003(void) {if ( f[ 3] | (1<<2)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb2f004(void) {if ( f[ 4] | (1<<2)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb2f005(void) {if ( f[ 5] | (1<<2)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb2f006(void) {if ( f[ 6] | (1<<2)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb2f007(void) {if ( f[ 7] | (1<<2)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb2f008(void) {if ( f[ 8] | (1<<2)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb2f009(void) {if ( f[ 9] | (1<<2)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb2f010(void) {if ( f[10] | (1<<2)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb2f011(void) {if ( f[11] | (1<<2)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb2f012(void) {if ( f[12] | (1<<2)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb2f013(void) {if ( f[13] | (1<<2)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb2f014(void) {if ( f[14] | (1<<2)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb2f015(void) {if ( f[15] | (1<<2)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb2f016(void) {if ( f[16] | (1<<2)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb2f017(void) {if ( f[17] | (1<<2)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb2f018(void) {if ( f[18] | (1<<2)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb2f019(void) {if ( f[19] | (1<<2)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb2f020(void) {if ( f[20] | (1<<2)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb2f021(void) {if ( f[21] | (1<<2)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb2f022(void) {if ( f[22] | (1<<2)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb2f023(void) {if ( f[23] | (1<<2)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb2f024(void) {if ( f[24] | (1<<2)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb2f025(void) {if ( f[25] | (1<<2)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb2f026(void) {if ( f[26] | (1<<2)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb2f027(void) {if ( f[27] | (1<<2)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb2f028(void) {if ( f[28] | (1<<2)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb2f029(void) {if ( f[29] | (1<<2)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb2f030(void) {if ( f[30] | (1<<2)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb2f031(void) {if ( f[31] | (1<<2)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb3f000(void) {if ( INDR_ADDR | (1<<3)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb3f001(void) {if ( f[1] | (1<<3)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb3f002(void) {if ( f[ 2] | (1<<3)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb3f003(void) {if ( f[ 3] | (1<<3)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb3f004(void) {if ( f[ 4] | (1<<3)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb3f005(void) {if ( f[ 5] | (1<<3)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb3f006(void) {if ( f[ 6] | (1<<3)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb3f007(void) {if ( f[ 7] | (1<<3)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb3f008(void) {if ( f[ 8] | (1<<3)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb3f009(void) {if ( f[ 9] | (1<<3)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb3f010(void) {if ( f[10] | (1<<3)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb3f011(void) {if ( f[11] | (1<<3)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb3f012(void) {if ( f[12] | (1<<3)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb3f013(void) {if ( f[13] | (1<<3)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb3f014(void) {if ( f[14] | (1<<3)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb3f015(void) {if ( f[15] | (1<<3)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb3f016(void) {if ( f[16] | (1<<3)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb3f017(void) {if ( f[17] | (1<<3)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb3f018(void) {if ( f[18] | (1<<3)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb3f019(void) {if ( f[19] | (1<<3)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb3f020(void) {if ( f[20] | (1<<3)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb3f021(void) {if ( f[21] | (1<<3)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb3f022(void) {if ( f[22] | (1<<3)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb3f023(void) {if ( f[23] | (1<<3)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb3f024(void) {if ( f[24] | (1<<3)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb3f025(void) {if ( f[25] | (1<<3)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb3f026(void) {if ( f[26] | (1<<3)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb3f027(void) {if ( f[27] | (1<<3)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb3f028(void) {if ( f[28] | (1<<3)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb3f029(void) {if ( f[29] | (1<<3)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb3f030(void) {if ( f[30] | (1<<3)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb3f031(void) {if ( f[31] | (1<<3)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb4f000(void) {if ( INDR_ADDR | (1<<4)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb4f001(void) {if ( f[1] | (1<<4)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb4f002(void) {if ( f[ 2] | (1<<4)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb4f003(void) {if ( f[ 3] | (1<<4)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb4f004(void) {if ( f[ 4] | (1<<4)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb4f005(void) {if ( f[ 5] | (1<<4)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb4f006(void) {if ( f[ 6] | (1<<4)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb4f007(void) {if ( f[ 7] | (1<<4)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb4f008(void) {if ( f[ 8] | (1<<4)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb4f009(void) {if ( f[ 9] | (1<<4)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb4f010(void) {if ( f[10] | (1<<4)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb4f011(void) {if ( f[11] | (1<<4)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb4f012(void) {if ( f[12] | (1<<4)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb4f013(void) {if ( f[13] | (1<<4)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb4f014(void) {if ( f[14] | (1<<4)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb4f015(void) {if ( f[15] | (1<<4)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb4f016(void) {if ( f[16] | (1<<4)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb4f017(void) {if ( f[17] | (1<<4)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb4f018(void) {if ( f[18] | (1<<4)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb4f019(void) {if ( f[19] | (1<<4)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb4f020(void) {if ( f[20] | (1<<4)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb4f021(void) {if ( f[21] | (1<<4)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb4f022(void) {if ( f[22] | (1<<4)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb4f023(void) {if ( f[23] | (1<<4)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb4f024(void) {if ( f[24] | (1<<4)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb4f025(void) {if ( f[25] | (1<<4)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb4f026(void) {if ( f[26] | (1<<4)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb4f027(void) {if ( f[27] | (1<<4)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb4f028(void) {if ( f[28] | (1<<4)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb4f029(void) {if ( f[29] | (1<<4)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb4f030(void) {if ( f[30] | (1<<4)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb4f031(void) {if ( f[31] | (1<<4)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb5f000(void) {if ( INDR_ADDR | (1<<5)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb5f001(void) {if ( f[1] | (1<<5)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb5f002(void) {if ( f[ 2] | (1<<5)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb5f003(void) {if ( f[ 3] | (1<<5)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb5f004(void) {if ( f[ 4] | (1<<5)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb5f005(void) {if ( f[ 5] | (1<<5)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb5f006(void) {if ( f[ 6] | (1<<5)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb5f007(void) {if ( f[ 7] | (1<<5)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb5f008(void) {if ( f[ 8] | (1<<5)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb5f009(void) {if ( f[ 9] | (1<<5)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb5f010(void) {if ( f[10] | (1<<5)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb5f011(void) {if ( f[11] | (1<<5)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb5f012(void) {if ( f[12] | (1<<5)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb5f013(void) {if ( f[13] | (1<<5)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb5f014(void) {if ( f[14] | (1<<5)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb5f015(void) {if ( f[15] | (1<<5)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb5f016(void) {if ( f[16] | (1<<5)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb5f017(void) {if ( f[17] | (1<<5)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb5f018(void) {if ( f[18] | (1<<5)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb5f019(void) {if ( f[19] | (1<<5)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb5f020(void) {if ( f[20] | (1<<5)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb5f021(void) {if ( f[21] | (1<<5)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb5f022(void) {if ( f[22] | (1<<5)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb5f023(void) {if ( f[23] | (1<<5)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb5f024(void) {if ( f[24] | (1<<5)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb5f025(void) {if ( f[25] | (1<<5)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb5f026(void) {if ( f[26] | (1<<5)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb5f027(void) {if ( f[27] | (1<<5)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb5f028(void) {if ( f[28] | (1<<5)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb5f029(void) {if ( f[29] | (1<<5)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb5f030(void) {if ( f[30] | (1<<5)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb5f031(void) {if ( f[31] | (1<<5)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb6f000(void) {if ( INDR_ADDR | (1<<6)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb6f001(void) {if ( f[1] | (1<<6)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb6f002(void) {if ( f[ 2] | (1<<6)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb6f003(void) {if ( f[ 3] | (1<<6)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb6f004(void) {if ( f[ 4] | (1<<6)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb6f005(void) {if ( f[ 5] | (1<<6)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb6f006(void) {if ( f[ 6] | (1<<6)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb6f007(void) {if ( f[ 7] | (1<<6)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb6f008(void) {if ( f[ 8] | (1<<6)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb6f009(void) {if ( f[ 9] | (1<<6)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb6f010(void) {if ( f[10] | (1<<6)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb6f011(void) {if ( f[11] | (1<<6)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb6f012(void) {if ( f[12] | (1<<6)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb6f013(void) {if ( f[13] | (1<<6)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb6f014(void) {if ( f[14] | (1<<6)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb6f015(void) {if ( f[15] | (1<<6)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb6f016(void) {if ( f[16] | (1<<6)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb6f017(void) {if ( f[17] | (1<<6)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb6f018(void) {if ( f[18] | (1<<6)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb6f019(void) {if ( f[19] | (1<<6)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb6f020(void) {if ( f[20] | (1<<6)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb6f021(void) {if ( f[21] | (1<<6)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb6f022(void) {if ( f[22] | (1<<6)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb6f023(void) {if ( f[23] | (1<<6)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb6f024(void) {if ( f[24] | (1<<6)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb6f025(void) {if ( f[25] | (1<<6)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb6f026(void) {if ( f[26] | (1<<6)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb6f027(void) {if ( f[27] | (1<<6)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb6f028(void) {if ( f[28] | (1<<6)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb6f029(void) {if ( f[29] | (1<<6)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb6f030(void) {if ( f[30] | (1<<6)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb6f031(void) {if ( f[31] | (1<<6)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb7f000(void) {if ( INDR_ADDR | (1<<7)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb7f001(void) {if ( f[1] | (1<<7)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb7f002(void) {if ( f[ 2] | (1<<7)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb7f003(void) {if ( f[ 3] | (1<<7)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb7f004(void) {if ( f[ 4] | (1<<7)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb7f005(void) {if ( f[ 5] | (1<<7)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb7f006(void) {if ( f[ 6] | (1<<7)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb7f007(void) {if ( f[ 7] | (1<<7)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb7f008(void) {if ( f[ 8] | (1<<7)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb7f009(void) {if ( f[ 9] | (1<<7)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb7f010(void) {if ( f[10] | (1<<7)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb7f011(void) {if ( f[11] | (1<<7)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb7f012(void) {if ( f[12] | (1<<7)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb7f013(void) {if ( f[13] | (1<<7)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb7f014(void) {if ( f[14] | (1<<7)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb7f015(void) {if ( f[15] | (1<<7)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb7f016(void) {if ( f[16] | (1<<7)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb7f017(void) {if ( f[17] | (1<<7)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb7f018(void) {if ( f[18] | (1<<7)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb7f019(void) {if ( f[19] | (1<<7)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb7f020(void) {if ( f[20] | (1<<7)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb7f021(void) {if ( f[21] | (1<<7)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb7f022(void) {if ( f[22] | (1<<7)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb7f023(void) {if ( f[23] | (1<<7)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb7f024(void) {if ( f[24] | (1<<7)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb7f025(void) {if ( f[25] | (1<<7)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb7f026(void) {if ( f[26] | (1<<7)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb7f027(void) {if ( f[27] | (1<<7)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb7f028(void) {if ( f[28] | (1<<7)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb7f029(void) {if ( f[29] | (1<<7)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb7f030(void) {if ( f[30] | (1<<7)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
void BTFSSb7f031(void) {if ( f[31] | (1<<7)  ) PC_Pointer++;  WDT_timer++; RTCC++;numberOfCyclesTaken++;}
