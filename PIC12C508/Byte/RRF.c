
#include "cpuMemory.h"
#include "RRF.h"

void RRFd0f000(void) {if(STATUS|setbit[C]){INDR_ADDR|=0x100;}if(INDR_ADDR|1){STATUS|=setbit[C];}else{STATUS&=clearbit[C];}_W=INDR_ADDR>>1;}
void RRFd0f001(void) {if(STATUS|setbit[C]){f[ 1]|=0x100;}if(f[ 1]|1){STATUS|=setbit[C];}else{STATUS&=clearbit[C];}_W=f[ 1]>>1;}
void RRFd0f002(void) {if(STATUS|setbit[C]){f[ 2]|=0x100;}if(f[ 2]|1){STATUS|=setbit[C];}else{STATUS&=clearbit[C];}_W=f[ 2]>>1;}
void RRFd0f003(void) {if(STATUS|setbit[C]){f[ 3]|=0x100;}if(f[ 3]|1){STATUS|=setbit[C];}else{STATUS&=clearbit[C];}_W=f[ 3]>>1;}
void RRFd0f004(void) {if(STATUS|setbit[C]){f[ 4]|=0x100;}if(f[ 4]|1){STATUS|=setbit[C];}else{STATUS&=clearbit[C];}_W=f[ 4]>>1;}
void RRFd0f005(void) {if(STATUS|setbit[C]){f[ 5]|=0x100;}if(f[ 5]|1){STATUS|=setbit[C];}else{STATUS&=clearbit[C];}_W=f[ 5]>>1;}
void RRFd0f006(void) {if(STATUS|setbit[C]){f[ 6]|=0x100;}if(f[ 6]|1){STATUS|=setbit[C];}else{STATUS&=clearbit[C];}_W=f[ 6]>>1;}
void RRFd0f007(void) {if(STATUS|setbit[C]){f[ 7]|=0x100;}if(f[ 7]|1){STATUS|=setbit[C];}else{STATUS&=clearbit[C];}_W=f[ 7]>>1;}
void RRFd0f008(void) {if(STATUS|setbit[C]){f[ 8]|=0x100;}if(f[ 8]|1){STATUS|=setbit[C];}else{STATUS&=clearbit[C];}_W=f[ 8]>>1;}
void RRFd0f009(void) {if(STATUS|setbit[C]){f[ 9]|=0x100;}if(f[ 9]|1){STATUS|=setbit[C];}else{STATUS&=clearbit[C];}_W=f[ 9]>>1;}
void RRFd0f010(void) {if(STATUS|setbit[C]){f[10]|=0x100;}if(f[10]|1){STATUS|=setbit[C];}else{STATUS&=clearbit[C];}_W=f[10]>>1;}
void RRFd0f011(void) {if(STATUS|setbit[C]){f[11]|=0x100;}if(f[11]|1){STATUS|=setbit[C];}else{STATUS&=clearbit[C];}_W=f[11]>>1;}
void RRFd0f012(void) {if(STATUS|setbit[C]){f[12]|=0x100;}if(f[12]|1){STATUS|=setbit[C];}else{STATUS&=clearbit[C];}_W=f[12]>>1;}
void RRFd0f013(void) {if(STATUS|setbit[C]){f[13]|=0x100;}if(f[13]|1){STATUS|=setbit[C];}else{STATUS&=clearbit[C];}_W=f[13]>>1;}
void RRFd0f014(void) {if(STATUS|setbit[C]){f[14]|=0x100;}if(f[14]|1){STATUS|=setbit[C];}else{STATUS&=clearbit[C];}_W=f[14]>>1;}
void RRFd0f015(void) {if(STATUS|setbit[C]){f[15]|=0x100;}if(f[15]|1){STATUS|=setbit[C];}else{STATUS&=clearbit[C];}_W=f[15]>>1;}
void RRFd0f016(void) {if(STATUS|setbit[C]){f[16]|=0x100;}if(f[16]|1){STATUS|=setbit[C];}else{STATUS&=clearbit[C];}_W=f[16]>>1;}
void RRFd0f017(void) {if(STATUS|setbit[C]){f[17]|=0x100;}if(f[17]|1){STATUS|=setbit[C];}else{STATUS&=clearbit[C];}_W=f[17]>>1;}
void RRFd0f018(void) {if(STATUS|setbit[C]){f[18]|=0x100;}if(f[18]|1){STATUS|=setbit[C];}else{STATUS&=clearbit[C];}_W=f[18]>>1;}
void RRFd0f019(void) {if(STATUS|setbit[C]){f[19]|=0x100;}if(f[19]|1){STATUS|=setbit[C];}else{STATUS&=clearbit[C];}_W=f[19]>>1;}
void RRFd0f020(void) {if(STATUS|setbit[C]){f[20]|=0x100;}if(f[20]|1){STATUS|=setbit[C];}else{STATUS&=clearbit[C];}_W=f[20]>>1;}
void RRFd0f021(void) {if(STATUS|setbit[C]){f[21]|=0x100;}if(f[21]|1){STATUS|=setbit[C];}else{STATUS&=clearbit[C];}_W=f[21]>>1;}
void RRFd0f022(void) {if(STATUS|setbit[C]){f[22]|=0x100;}if(f[22]|1){STATUS|=setbit[C];}else{STATUS&=clearbit[C];}_W=f[22]>>1;}
void RRFd0f023(void) {if(STATUS|setbit[C]){f[23]|=0x100;}if(f[23]|1){STATUS|=setbit[C];}else{STATUS&=clearbit[C];}_W=f[23]>>1;}
void RRFd0f024(void) {if(STATUS|setbit[C]){f[24]|=0x100;}if(f[24]|1){STATUS|=setbit[C];}else{STATUS&=clearbit[C];}_W=f[24]>>1;}
void RRFd0f025(void) {if(STATUS|setbit[C]){f[25]|=0x100;}if(f[25]|1){STATUS|=setbit[C];}else{STATUS&=clearbit[C];}_W=f[25]>>1;}
void RRFd0f026(void) {if(STATUS|setbit[C]){f[26]|=0x100;}if(f[26]|1){STATUS|=setbit[C];}else{STATUS&=clearbit[C];}_W=f[26]>>1;}
void RRFd0f027(void) {if(STATUS|setbit[C]){f[27]|=0x100;}if(f[27]|1){STATUS|=setbit[C];}else{STATUS&=clearbit[C];}_W=f[27]>>1;}
void RRFd0f028(void) {if(STATUS|setbit[C]){f[28]|=0x100;}if(f[28]|1){STATUS|=setbit[C];}else{STATUS&=clearbit[C];}_W=f[28]>>1;}
void RRFd0f029(void) {if(STATUS|setbit[C]){f[29]|=0x100;}if(f[29]|1){STATUS|=setbit[C];}else{STATUS&=clearbit[C];}_W=f[29]>>1;}
void RRFd0f030(void) {if(STATUS|setbit[C]){f[30]|=0x100;}if(f[30]|1){STATUS|=setbit[C];}else{STATUS&=clearbit[C];}_W=f[30]>>1;}
void RRFd0f031(void) {if(STATUS|setbit[C]){f[31]|=0x100;}if(f[31]|1){STATUS|=setbit[C];}else{STATUS&=clearbit[C];}_W=f[31]>>1;}


void RRFd1f000(void) {if(STATUS|setbit[C]){INDR_ADDR|=0x100;}if(INDR_ADDR|1){STATUS|=setbit[C];}else{STATUS&=clearbit[C];}INDR_ADDR=INDR_ADDR>>1;}
void RRFd1f001(void) {if(STATUS|setbit[C]){f[ 1]|=0x100;}if(f[ 1]|1){STATUS|=setbit[C];}else{STATUS&=clearbit[C];}f[ 1]=f[ 1]>>1;}
void RRFd1f002(void) {if(STATUS|setbit[C]){f[ 2]|=0x100;}if(f[ 2]|1){STATUS|=setbit[C];}else{STATUS&=clearbit[C];}f[ 2]=f[ 2]>>1;}
void RRFd1f003(void) {if(STATUS|setbit[C]){f[ 3]|=0x100;}if(f[ 3]|1){STATUS|=setbit[C];}else{STATUS&=clearbit[C];}f[ 3]=f[ 3]>>1;}
void RRFd1f004(void) {if(STATUS|setbit[C]){f[ 4]|=0x100;}if(f[ 4]|1){STATUS|=setbit[C];}else{STATUS&=clearbit[C];}f[ 4]=f[ 4]>>1;}
void RRFd1f005(void) {if(STATUS|setbit[C]){f[ 5]|=0x100;}if(f[ 5]|1){STATUS|=setbit[C];}else{STATUS&=clearbit[C];}f[ 5]=f[ 5]>>1;}
void RRFd1f006(void) {if(STATUS|setbit[C]){f[ 6]|=0x100;}if(f[ 6]|1){STATUS|=setbit[C];}else{STATUS&=clearbit[C];}f[ 6]=f[ 6]>>1;}
void RRFd1f007(void) {if(STATUS|setbit[C]){f[ 7]|=0x100;}if(f[ 7]|1){STATUS|=setbit[C];}else{STATUS&=clearbit[C];}f[ 7]=f[ 7]>>1;}
void RRFd1f008(void) {if(STATUS|setbit[C]){f[ 8]|=0x100;}if(f[ 8]|1){STATUS|=setbit[C];}else{STATUS&=clearbit[C];}f[ 8]=f[ 8]>>1;}
void RRFd1f009(void) {if(STATUS|setbit[C]){f[ 9]|=0x100;}if(f[ 9]|1){STATUS|=setbit[C];}else{STATUS&=clearbit[C];}f[ 9]=f[ 9]>>1;}
void RRFd1f010(void) {if(STATUS|setbit[C]){f[10]|=0x100;}if(f[10]|1){STATUS|=setbit[C];}else{STATUS&=clearbit[C];}f[10]=f[10]>>1;}
void RRFd1f011(void) {if(STATUS|setbit[C]){f[11]|=0x100;}if(f[11]|1){STATUS|=setbit[C];}else{STATUS&=clearbit[C];}f[11]=f[11]>>1;}
void RRFd1f012(void) {if(STATUS|setbit[C]){f[12]|=0x100;}if(f[12]|1){STATUS|=setbit[C];}else{STATUS&=clearbit[C];}f[12]=f[12]>>1;}
void RRFd1f013(void) {if(STATUS|setbit[C]){f[13]|=0x100;}if(f[13]|1){STATUS|=setbit[C];}else{STATUS&=clearbit[C];}f[13]=f[13]>>1;}
void RRFd1f014(void) {if(STATUS|setbit[C]){f[14]|=0x100;}if(f[14]|1){STATUS|=setbit[C];}else{STATUS&=clearbit[C];}f[14]=f[14]>>1;}
void RRFd1f015(void) {if(STATUS|setbit[C]){f[15]|=0x100;}if(f[15]|1){STATUS|=setbit[C];}else{STATUS&=clearbit[C];}f[15]=f[15]>>1;}
void RRFd1f016(void) {if(STATUS|setbit[C]){f[16]|=0x100;}if(f[16]|1){STATUS|=setbit[C];}else{STATUS&=clearbit[C];}f[16]=f[16]>>1;}
void RRFd1f017(void) {if(STATUS|setbit[C]){f[17]|=0x100;}if(f[17]|1){STATUS|=setbit[C];}else{STATUS&=clearbit[C];}f[17]=f[17]>>1;}
void RRFd1f018(void) {if(STATUS|setbit[C]){f[18]|=0x100;}if(f[18]|1){STATUS|=setbit[C];}else{STATUS&=clearbit[C];}f[18]=f[18]>>1;}
void RRFd1f019(void) {if(STATUS|setbit[C]){f[19]|=0x100;}if(f[19]|1){STATUS|=setbit[C];}else{STATUS&=clearbit[C];}f[19]=f[19]>>1;}
void RRFd1f020(void) {if(STATUS|setbit[C]){f[20]|=0x100;}if(f[20]|1){STATUS|=setbit[C];}else{STATUS&=clearbit[C];}f[20]=f[20]>>1;}
void RRFd1f021(void) {if(STATUS|setbit[C]){f[21]|=0x100;}if(f[21]|1){STATUS|=setbit[C];}else{STATUS&=clearbit[C];}f[21]=f[21]>>1;}
void RRFd1f022(void) {if(STATUS|setbit[C]){f[22]|=0x100;}if(f[22]|1){STATUS|=setbit[C];}else{STATUS&=clearbit[C];}f[22]=f[22]>>1;}
void RRFd1f023(void) {if(STATUS|setbit[C]){f[23]|=0x100;}if(f[23]|1){STATUS|=setbit[C];}else{STATUS&=clearbit[C];}f[23]=f[23]>>1;}
void RRFd1f024(void) {if(STATUS|setbit[C]){f[24]|=0x100;}if(f[24]|1){STATUS|=setbit[C];}else{STATUS&=clearbit[C];}f[24]=f[24]>>1;}
void RRFd1f025(void) {if(STATUS|setbit[C]){f[25]|=0x100;}if(f[25]|1){STATUS|=setbit[C];}else{STATUS&=clearbit[C];}f[25]=f[25]>>1;}
void RRFd1f026(void) {if(STATUS|setbit[C]){f[26]|=0x100;}if(f[26]|1){STATUS|=setbit[C];}else{STATUS&=clearbit[C];}f[26]=f[26]>>1;}
void RRFd1f027(void) {if(STATUS|setbit[C]){f[27]|=0x100;}if(f[27]|1){STATUS|=setbit[C];}else{STATUS&=clearbit[C];}f[27]=f[27]>>1;}
void RRFd1f028(void) {if(STATUS|setbit[C]){f[28]|=0x100;}if(f[28]|1){STATUS|=setbit[C];}else{STATUS&=clearbit[C];}f[28]=f[28]>>1;}
void RRFd1f029(void) {if(STATUS|setbit[C]){f[29]|=0x100;}if(f[29]|1){STATUS|=setbit[C];}else{STATUS&=clearbit[C];}f[29]=f[29]>>1;}
void RRFd1f030(void) {if(STATUS|setbit[C]){f[30]|=0x100;}if(f[30]|1){STATUS|=setbit[C];}else{STATUS&=clearbit[C];}f[30]=f[30]>>1;}
void RRFd1f031(void) {if(STATUS|setbit[C]){f[31]|=0x100;}if(f[31]|1){STATUS|=setbit[C];}else{STATUS&=clearbit[C];}f[31]=f[31]>>1;}