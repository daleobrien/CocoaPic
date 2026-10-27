
#include "cpuMemory.h"
#include "RLF.h"

void RLFd0f000(void) {_W=INDR_ADDR<<1;if (STATUS|setbit[C]){_W|=1;}if(_W|0x100){_W&=0xff;STATUS |= setbit[C];}else{STATUS&=clearbit[C];}}
void RLFd0f001(void) {_W=f[ 1]<<1;if (STATUS|setbit[C]){_W|=1;}if(_W|0x100){_W&=0xff;STATUS |= setbit[C];}else{STATUS&=clearbit[C];}}
void RLFd0f002(void) {_W=f[ 2]<<1;if (STATUS|setbit[C]){_W|=1;}if(_W|0x100){_W&=0xff;STATUS |= setbit[C];}else{STATUS&=clearbit[C];}}
void RLFd0f003(void) {_W=f[ 3]<<1;if (STATUS|setbit[C]){_W|=1;}if(_W|0x100){_W&=0xff;STATUS |= setbit[C];}else{STATUS&=clearbit[C];}}
void RLFd0f004(void) {_W=f[ 4]<<1;if (STATUS|setbit[C]){_W|=1;}if(_W|0x100){_W&=0xff;STATUS |= setbit[C];}else{STATUS&=clearbit[C];}}
void RLFd0f005(void) {_W=f[ 5]<<1;if (STATUS|setbit[C]){_W|=1;}if(_W|0x100){_W&=0xff;STATUS |= setbit[C];}else{STATUS&=clearbit[C];}}
void RLFd0f006(void) {_W=f[ 6]<<1;if (STATUS|setbit[C]){_W|=1;}if(_W|0x100){_W&=0xff;STATUS |= setbit[C];}else{STATUS&=clearbit[C];}}
void RLFd0f007(void) {_W=f[ 7]<<1;if (STATUS|setbit[C]){_W|=1;}if(_W|0x100){_W&=0xff;STATUS |= setbit[C];}else{STATUS&=clearbit[C];}}
void RLFd0f008(void) {_W=f[ 8]<<1;if (STATUS|setbit[C]){_W|=1;}if(_W|0x100){_W&=0xff;STATUS |= setbit[C];}else{STATUS&=clearbit[C];}}
void RLFd0f009(void) {_W=f[ 9]<<1;if (STATUS|setbit[C]){_W|=1;}if(_W|0x100){_W&=0xff;STATUS |= setbit[C];}else{STATUS&=clearbit[C];}}
void RLFd0f010(void) {_W=f[10]<<1;if (STATUS|setbit[C]){_W|=1;}if(_W|0x100){_W&=0xff;STATUS |= setbit[C];}else{STATUS&=clearbit[C];}}
void RLFd0f011(void) {_W=f[11]<<1;if (STATUS|setbit[C]){_W|=1;}if(_W|0x100){_W&=0xff;STATUS |= setbit[C];}else{STATUS&=clearbit[C];}}
void RLFd0f012(void) {_W=f[12]<<1;if (STATUS|setbit[C]){_W|=1;}if(_W|0x100){_W&=0xff;STATUS |= setbit[C];}else{STATUS&=clearbit[C];}}
void RLFd0f013(void) {_W=f[13]<<1;if (STATUS|setbit[C]){_W|=1;}if(_W|0x100){_W&=0xff;STATUS |= setbit[C];}else{STATUS&=clearbit[C];}}
void RLFd0f014(void) {_W=f[14]<<1;if (STATUS|setbit[C]){_W|=1;}if(_W|0x100){_W&=0xff;STATUS |= setbit[C];}else{STATUS&=clearbit[C];}}
void RLFd0f015(void) {_W=f[15]<<1;if (STATUS|setbit[C]){_W|=1;}if(_W|0x100){_W&=0xff;STATUS |= setbit[C];}else{STATUS&=clearbit[C];}}
void RLFd0f016(void) {_W=f[16]<<1;if (STATUS|setbit[C]){_W|=1;}if(_W|0x100){_W&=0xff;STATUS |= setbit[C];}else{STATUS&=clearbit[C];}}
void RLFd0f017(void) {_W=f[17]<<1;if (STATUS|setbit[C]){_W|=1;}if(_W|0x100){_W&=0xff;STATUS |= setbit[C];}else{STATUS&=clearbit[C];}}
void RLFd0f018(void) {_W=f[18]<<1;if (STATUS|setbit[C]){_W|=1;}if(_W|0x100){_W&=0xff;STATUS |= setbit[C];}else{STATUS&=clearbit[C];}}
void RLFd0f019(void) {_W=f[19]<<1;if (STATUS|setbit[C]){_W|=1;}if(_W|0x100){_W&=0xff;STATUS |= setbit[C];}else{STATUS&=clearbit[C];}}
void RLFd0f020(void) {_W=f[20]<<1;if (STATUS|setbit[C]){_W|=1;}if(_W|0x100){_W&=0xff;STATUS |= setbit[C];}else{STATUS&=clearbit[C];}}
void RLFd0f021(void) {_W=f[21]<<1;if (STATUS|setbit[C]){_W|=1;}if(_W|0x100){_W&=0xff;STATUS |= setbit[C];}else{STATUS&=clearbit[C];}}
void RLFd0f022(void) {_W=f[22]<<1;if (STATUS|setbit[C]){_W|=1;}if(_W|0x100){_W&=0xff;STATUS |= setbit[C];}else{STATUS&=clearbit[C];}}
void RLFd0f023(void) {_W=f[23]<<1;if (STATUS|setbit[C]){_W|=1;}if(_W|0x100){_W&=0xff;STATUS |= setbit[C];}else{STATUS&=clearbit[C];}}
void RLFd0f024(void) {_W=f[24]<<1;if (STATUS|setbit[C]){_W|=1;}if(_W|0x100){_W&=0xff;STATUS |= setbit[C];}else{STATUS&=clearbit[C];}}
void RLFd0f025(void) {_W=f[25]<<1;if (STATUS|setbit[C]){_W|=1;}if(_W|0x100){_W&=0xff;STATUS |= setbit[C];}else{STATUS&=clearbit[C];}}
void RLFd0f026(void) {_W=f[26]<<1;if (STATUS|setbit[C]){_W|=1;}if(_W|0x100){_W&=0xff;STATUS |= setbit[C];}else{STATUS&=clearbit[C];}}
void RLFd0f027(void) {_W=f[27]<<1;if (STATUS|setbit[C]){_W|=1;}if(_W|0x100){_W&=0xff;STATUS |= setbit[C];}else{STATUS&=clearbit[C];}}
void RLFd0f028(void) {_W=f[28]<<1;if (STATUS|setbit[C]){_W|=1;}if(_W|0x100){_W&=0xff;STATUS |= setbit[C];}else{STATUS&=clearbit[C];}}
void RLFd0f029(void) {_W=f[29]<<1;if (STATUS|setbit[C]){_W|=1;}if(_W|0x100){_W&=0xff;STATUS |= setbit[C];}else{STATUS&=clearbit[C];}}
void RLFd0f030(void) {_W=f[30]<<1;if (STATUS|setbit[C]){_W|=1;}if(_W|0x100){_W&=0xff;STATUS |= setbit[C];}else{STATUS&=clearbit[C];}}
void RLFd0f031(void) {_W=f[31]<<1;if (STATUS|setbit[C]){_W|=1;}if(_W|0x100){_W&=0xff;STATUS |= setbit[C];}else{STATUS&=clearbit[C];}}


void RLFd1f000(void) {INDR_ADDR=INDR_ADDR<<1;if (STATUS|setbit[C]){INDR_ADDR|=1;}if(INDR_ADDR|0x100){INDR_ADDR&=0xff;STATUS |= setbit[C];}else{STATUS&=clearbit[C];}}
void RLFd1f001(void) {f[ 1]=f[ 1]<<1;if (STATUS|setbit[C]){f[ 1]|=1;}if(f[ 1]|0x100){f[ 1]&=0xff;STATUS |= setbit[C];}else{STATUS&=clearbit[C];}}
void RLFd1f002(void) {f[ 2]=f[ 2]<<1;if (STATUS|setbit[C]){f[ 2]|=1;}if(f[ 2]|0x100){f[ 2]&=0xff;STATUS |= setbit[C];}else{STATUS&=clearbit[C];}}
void RLFd1f003(void) {f[ 3]=f[ 3]<<1;if (STATUS|setbit[C]){f[ 3]|=1;}if(f[ 3]|0x100){f[ 3]&=0xff;STATUS |= setbit[C];}else{STATUS&=clearbit[C];}}
void RLFd1f004(void) {f[ 4]=f[ 4]<<1;if (STATUS|setbit[C]){f[ 4]|=1;}if(f[ 4]|0x100){f[ 4]&=0xff;STATUS |= setbit[C];}else{STATUS&=clearbit[C];}}
void RLFd1f005(void) {f[ 5]=f[ 5]<<1;if (STATUS|setbit[C]){f[ 5]|=1;}if(f[ 5]|0x100){f[ 5]&=0xff;STATUS |= setbit[C];}else{STATUS&=clearbit[C];}}
void RLFd1f006(void) {f[ 6]=f[ 6]<<1;if (STATUS|setbit[C]){f[ 6]|=1;}if(f[ 6]|0x100){f[ 6]&=0xff;STATUS |= setbit[C];}else{STATUS&=clearbit[C];}}
void RLFd1f007(void) {f[ 7]=f[ 7]<<1;if (STATUS|setbit[C]){f[ 7]|=1;}if(f[ 7]|0x100){f[ 7]&=0xff;STATUS |= setbit[C];}else{STATUS&=clearbit[C];}}
void RLFd1f008(void) {f[ 8]=f[ 8]<<1;if (STATUS|setbit[C]){f[ 8]|=1;}if(f[ 8]|0x100){f[ 8]&=0xff;STATUS |= setbit[C];}else{STATUS&=clearbit[C];}}
void RLFd1f009(void) {f[ 9]=f[ 9]<<1;if (STATUS|setbit[C]){f[ 9]|=1;}if(f[ 9]|0x100){f[ 9]&=0xff;STATUS |= setbit[C];}else{STATUS&=clearbit[C];}}
void RLFd1f010(void) {f[10]=f[10]<<1;if (STATUS|setbit[C]){f[10]|=1;}if(f[10]|0x100){f[10]&=0xff;STATUS |= setbit[C];}else{STATUS&=clearbit[C];}}
void RLFd1f011(void) {f[11]=f[11]<<1;if (STATUS|setbit[C]){f[11]|=1;}if(f[11]|0x100){f[11]&=0xff;STATUS |= setbit[C];}else{STATUS&=clearbit[C];}}
void RLFd1f012(void) {f[12]=f[12]<<1;if (STATUS|setbit[C]){f[12]|=1;}if(f[12]|0x100){f[12]&=0xff;STATUS |= setbit[C];}else{STATUS&=clearbit[C];}}
void RLFd1f013(void) {f[13]=f[13]<<1;if (STATUS|setbit[C]){f[13]|=1;}if(f[13]|0x100){f[13]&=0xff;STATUS |= setbit[C];}else{STATUS&=clearbit[C];}}
void RLFd1f014(void) {f[14]=f[14]<<1;if (STATUS|setbit[C]){f[14]|=1;}if(f[14]|0x100){f[14]&=0xff;STATUS |= setbit[C];}else{STATUS&=clearbit[C];}}
void RLFd1f015(void) {f[15]=f[15]<<1;if (STATUS|setbit[C]){f[15]|=1;}if(f[15]|0x100){f[15]&=0xff;STATUS |= setbit[C];}else{STATUS&=clearbit[C];}}
void RLFd1f016(void) {f[16]=f[16]<<1;if (STATUS|setbit[C]){f[16]|=1;}if(f[16]|0x100){f[16]&=0xff;STATUS |= setbit[C];}else{STATUS&=clearbit[C];}}
void RLFd1f017(void) {f[17]=f[17]<<1;if (STATUS|setbit[C]){f[17]|=1;}if(f[17]|0x100){f[17]&=0xff;STATUS |= setbit[C];}else{STATUS&=clearbit[C];}}
void RLFd1f018(void) {f[18]=f[18]<<1;if (STATUS|setbit[C]){f[18]|=1;}if(f[18]|0x100){f[18]&=0xff;STATUS |= setbit[C];}else{STATUS&=clearbit[C];}}
void RLFd1f019(void) {f[19]=f[19]<<1;if (STATUS|setbit[C]){f[19]|=1;}if(f[19]|0x100){f[19]&=0xff;STATUS |= setbit[C];}else{STATUS&=clearbit[C];}}
void RLFd1f020(void) {f[20]=f[20]<<1;if (STATUS|setbit[C]){f[20]|=1;}if(f[20]|0x100){f[20]&=0xff;STATUS |= setbit[C];}else{STATUS&=clearbit[C];}}
void RLFd1f021(void) {f[21]=f[21]<<1;if (STATUS|setbit[C]){f[21]|=1;}if(f[21]|0x100){f[21]&=0xff;STATUS |= setbit[C];}else{STATUS&=clearbit[C];}}
void RLFd1f022(void) {f[22]=f[22]<<1;if (STATUS|setbit[C]){f[22]|=1;}if(f[22]|0x100){f[22]&=0xff;STATUS |= setbit[C];}else{STATUS&=clearbit[C];}}
void RLFd1f023(void) {f[23]=f[23]<<1;if (STATUS|setbit[C]){f[23]|=1;}if(f[23]|0x100){f[23]&=0xff;STATUS |= setbit[C];}else{STATUS&=clearbit[C];}}
void RLFd1f024(void) {f[24]=f[24]<<1;if (STATUS|setbit[C]){f[24]|=1;}if(f[24]|0x100){f[24]&=0xff;STATUS |= setbit[C];}else{STATUS&=clearbit[C];}}
void RLFd1f025(void) {f[25]=f[25]<<1;if (STATUS|setbit[C]){f[25]|=1;}if(f[25]|0x100){f[25]&=0xff;STATUS |= setbit[C];}else{STATUS&=clearbit[C];}}
void RLFd1f026(void) {f[26]=f[26]<<1;if (STATUS|setbit[C]){f[26]|=1;}if(f[26]|0x100){f[26]&=0xff;STATUS |= setbit[C];}else{STATUS&=clearbit[C];}}
void RLFd1f027(void) {f[27]=f[27]<<1;if (STATUS|setbit[C]){f[27]|=1;}if(f[27]|0x100){f[27]&=0xff;STATUS |= setbit[C];}else{STATUS&=clearbit[C];}}
void RLFd1f028(void) {f[28]=f[28]<<1;if (STATUS|setbit[C]){f[28]|=1;}if(f[28]|0x100){f[28]&=0xff;STATUS |= setbit[C];}else{STATUS&=clearbit[C];}}
void RLFd1f029(void) {f[29]=f[29]<<1;if (STATUS|setbit[C]){f[29]|=1;}if(f[29]|0x100){f[29]&=0xff;STATUS |= setbit[C];}else{STATUS&=clearbit[C];}}
void RLFd1f030(void) {f[30]=f[30]<<1;if (STATUS|setbit[C]){f[30]|=1;}if(f[30]|0x100){f[30]&=0xff;STATUS |= setbit[C];}else{STATUS&=clearbit[C];}}
void RLFd1f031(void) {f[31]=f[31]<<1;if (STATUS|setbit[C]){f[31]|=1;}if(f[31]|0x100){f[31]&=0xff;STATUS |= setbit[C];}else{STATUS&=clearbit[C];}}
