// Automatically generated on Sunday, 2 November 2003

union u8 {
 struct  {
  unsigned char b7:1;  // this is MSB
  unsigned char b6:1;
  unsigned char b5:1;
  unsigned char b4:1;
  unsigned char b3:1;
  unsigned char b2:1;
  unsigned char b1:1;
  unsigned char b0:1;
 } bit;
 unsigned char a;
};

union  p8 {

    struct  {
  unsigned char PCH;
  unsigned char PCL;
 } byte;
 
 unsigned int a;
};

#define ROM_no_of_bytes  1024
#define RAM_no_of_bytes  128

extern union u8 f[RAM_no_of_bytes][2];
extern int   ROM [ROM_no_of_bytes];			// ROM space

extern int   breakPoints[ROM_no_of_bytes];		// halting points

extern unsigned char _W;

#define _C	f[3][0].bit.b0
#define  _DC	f[3][0].bit.b1
#define  _Z	f[3][0].bit.b2
#define  _PD	f[3][0].bit.b3
#define  _TO	f[3][0].bit.b4
#define  _RP0	f[3][0].bit.b5
#define  _RP1	f[3][0].bit.b6
#define  _IRP0	f[3][0].bit.b7

extern union p8 _PC;

extern unsigned int _Stack[7];
extern unsigned int _StackPointer;
extern unsigned char _TMR;
extern unsigned char _WDT;

// IS this needed ?
extern unsigned char WDT_prescaler;


#define _incTimers() (_TMR++)
