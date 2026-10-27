/*
 *  USB Pickit Interface.c
 *  CocoaPic
 *
 *  Created by Orla O'Brien on Tue Nov 11 2003.
 *  Copyright (c) 2003 __MyCompanyName__. All rights reserved.
 *
 */
#import <Cocoa/Cocoa.h>
//#include <Carbon/Carbon.h>

#include "USB Pickit Interface.h"
//#include <stdio.h>
#include <mach/mach.h>
//#include <CoreFoundation/CFNumber.h>
#include <IOKit/IOKitLib.h>
#include <IOKit/IOCFPlugIn.h>
#include <IOKit/usb/IOUSBLib.h>

#include "PIC12F675/Headers/PIC12F675_cpuCore.h"
#include "PIC12F675/Headers/PIC12F675_cpuMemory.h"



extern int ROM[ROM_no_of_bytes];

mach_port_t 	masterPort = 0;				// requires <mach/mach.h>
char		outBuf[8];

UInt8		inBuf[8096];

UInt8		eepromBuf[8096];

int 		count = 0;

UInt8 		myinPipeRef;
UInt8 		myoutPipeRef;

IOUSBInterfaceInterface **myInterface;

char requestVersion[8]			= { 'v','Z','Z','Z','Z','Z','Z','Z' };
char requestProgramModeOffThenOn[8]	= { 'p','P','Z','Z','Z','Z','Z','Z' };
char requestProgramModeOff[8]		= { 'p','V', 3,'Z','Z','Z','Z','Z' };
char requestEraseROM[8]			= { 'E','Z','Z','Z','Z','Z','Z','Z' };
char requestEraseEEPROM[8]		= { 'e','Z','Z','Z','Z','Z','Z','Z' };

char requestWriteToROM[8]		= { 'W',' ',' ','W',' ',' ','Z','Z' };
char requestWriteToEEPROM[8]		= { 'D',' ','D',' ','D',' ','D',' ' };

char requestReadFromROM[8]		= { 'R','R','R','R','R','R','R','R' };
char requestReadFromEEPROM[8]		= { 'r','r','r','r','r','r','r','r' };
char requestDeviceDetails[8] 		= { 'P','C','R','R','P','Z','Z','Z' };
char requestVersionAndProgramMode[8] 	= { 'P','V','Z','Z','Z','Z','Z','Z' };

//    printf("MyCallbackfunction: %d, %x, %d\n", (int)dummy, (unsigned int)result, (int)arg0);
void MyCallBackFunction(void *dummy, IOReturn result, void *arg0) { CFRunLoopStop(CFRunLoopGetCurrent()); }
void MyCallBackFunctionR(void *dummy, IOReturn result, void *arg0){CFRunLoopStop(CFRunLoopGetCurrent());}



// Send out the command ...
void myAsyncRequest(  void * data){
    
    CFRunLoopSourceRef	cfSource;
    
    (*myInterface)->CreateInterfaceAsyncEventSource(myInterface, &cfSource);
    CFRunLoopAddSource(CFRunLoopGetCurrent(), cfSource, kCFRunLoopDefaultMode);

    (*myInterface)->WritePipeAsync(myInterface, myoutPipeRef, data, 8, (IOAsyncCallback1) MyCallBackFunction, (void*)(UInt32)myinPipeRef);
	    
 
    CFRunLoopRun();
    CFRunLoopRemoveSource(CFRunLoopGetCurrent(), cfSource, kCFRunLoopDefaultMode);
}

void myAsyncRead(  int bufferOffset ){
    CFRunLoopSourceRef		cfSource;
    int i;
	
    (*myInterface)->CreateInterfaceAsyncEventSource(myInterface, &cfSource);
    CFRunLoopAddSource(CFRunLoopGetCurrent(), cfSource, kCFRunLoopDefaultMode);
    for (i=0; i < 8; i++){
	(*myInterface)->ReadPipeAsync(myInterface, myinPipeRef, inBuf+i*8+bufferOffset, 8, (IOAsyncCallback1) MyCallBackFunctionR, (void*)(UInt32)myoutPipeRef);
	CFRunLoopRunInMode(kCFRunLoopDefaultMode, 0.020 ,0); // 20 ms is almost enough
    }
    CFRunLoopRemoveSource(CFRunLoopGetCurrent(), cfSource, kCFRunLoopDefaultMode); 
}


void transferData(IOUSBInterfaceInterface **intf, UInt8 inPipeRef, UInt8 outPipeRef)
{
 //   IOReturn			err;
 // CFRunLoopSourceRef		cfSource;
    int				i;

   unsigned char ver[3];

    myInterface = intf;
    myinPipeRef = inPipeRef;
    myoutPipeRef = outPipeRef;

    
    for(i=0;i<8096;i++){ inBuf[i]=-1;}

    myAsyncRequest( requestProgramModeOffThenOn );
    
    myAsyncRequest( requestVersion );
    myAsyncRead( 0 );

 //   printf("\nVersion\n");
 //   for(i=0;i<8;i++){ printf("%X ",inBuf[i] ); }  
    [[NSNotificationCenter defaultCenter] postNotificationName: @"status" object:[NSString stringWithFormat:@"Got Version %i.%i%i",inBuf[0] ,inBuf[1],inBuf[2] ]  ];
 
    ver[0]= inBuf[0];
    ver[1]= inBuf[1];
    ver[2]= inBuf[2];
    
    myAsyncRequest( requestProgramModeOffThenOn );
    
    for(i=0;i<32;i++){ 
	myAsyncRequest( requestReadFromROM );
	myAsyncRead( i*64 );
        [[NSNotificationCenter defaultCenter] postNotificationName: @"status" object:[NSString stringWithFormat:@"Reading ROM %i%%",(int)(100*i/32)  ]  ];
        
    }
    
    myAsyncRequest(requestReadFromROM );
    myAsyncRead( 32*64 );
    [[NSNotificationCenter defaultCenter] postNotificationName: @"status" object:[NSString stringWithFormat:@"Reading ROM 100%%"]  ];
    
    /*
    printf("\nROM\n"); // 1024 words

    for(j=62;j<64;j++){  //128
	for(i=0;i<10;i++){
	    a = inBuf[i*2  +32*j];
	    b = inBuf[i*2+1+32*j];
	    printf("%X ",a+ (b<<8));
	}
	printf(" ... \n");
    }
    
    printf(" ... \n");
    for(j=60;j<62;j++){  //128
	for(i=0;i<10;i++){
	    a = inBuf[i*2  +32*j];
	    b = inBuf[i*2+1+32*j];
	    printf("%x ",a+ (b<<8));
	}
	printf(" ... \n");
    }*/

    for(i=0;i<ROM_no_of_bytes-1;i++){ ROM[i] = inBuf[2*i] + (inBuf[2*i+1]<<8);  }
    
    for(i=0;i<8096;i++){ inBuf[i]=0; }

    myAsyncRequest( requestProgramModeOffThenOn);
/*
    printf("\nWrite to EEPROM\n");  // 128 bytes
    for(i=0;i<32;i++){
    	requestWriteToEEPROM[1] = 4*i;
	requestWriteToEEPROM[3] = 4*i+1;
	requestWriteToEEPROM[5] = 4*i+2;
	requestWriteToEEPROM[7] = 4*i+3;
	myAsyncRequest( requestWriteToEEPROM );
              
    }
*/
    for(i=0;i<8096;i++){ inBuf[i]=0; }
    myAsyncRequest( requestProgramModeOffThenOn);
  //  printf("\nEEPROM\n");  // 128 bytes
    for(i=0;i<8;i++){
	myAsyncRequest( requestReadFromEEPROM);
	myAsyncRead(  8*i );
        [[NSNotificationCenter defaultCenter] postNotificationName: @"status" object:[NSString stringWithFormat:@"Reading EEPROM %i%%",(int)(100*i/8)  ]  ];

    }
    [[NSNotificationCenter defaultCenter] postNotificationName: @"status" object:[NSString stringWithFormat:@"Reading EEPROM %i%%",100  ]  ];

/*
    for(j=0;j<6;j++){  //128
	for(i=0;i<32;i++){
	  printf("%X ",inBuf[i + 32*j] );
	}  printf("\n");
    }*/

    myAsyncRequest(requestProgramModeOff);
    [[NSNotificationCenter defaultCenter] postNotificationName: @"status" object:[NSString stringWithFormat:@"Finished Read of PicKit <Version %i.%i%i>",ver[0],ver[1],ver[2]  ]  ];

}


void dealWithPipes(IOUSBInterfaceInterface **intf, UInt8 numPipes)
{
    int					i;
    IOReturn				err;			
    UInt8				inPipeRef = 0;
    UInt8				outPipeRef = 0;
    UInt8				direction, number, transferType, interval;
    UInt16				maxPacketSize;
    
    // pipes are one based, since zero is the default control pipe
    for (i=1; i <= numPipes; i++)
    {
	err = (*intf)->GetPipeProperties(intf, i, &direction, &number, &transferType, &maxPacketSize, &interval);
	if (err)
	{
	 //   printf("dealWithPipes: unable to get pipe properties for pipe %d, err = %08x\n", i, err);
	   return;
	}

	
	if (transferType != kUSBInterrupt)
	{
	   // printf("dealWithPipes: skipping pipe %d because it is not a interrupt pipe\n", i);
	    continue;
	}
	
	if ((direction == kUSBIn) && !inPipeRef)
	{
	  //  printf("dealWithPipes: grabbing INTERRUPT IN pipe index %d, number %d\n",i, number);
	    inPipeRef = i;
	}
	if ((direction == kUSBOut) && !outPipeRef)
	{
	    //printf("dealWithPipes: grabbing INTERRUPT OUT pipe index %d, number %d\n", i, number);
	    outPipeRef = i;
	}
    }
    if (inPipeRef && outPipeRef)
	transferData(intf, inPipeRef, outPipeRef);
}

void dealWithInterface(io_service_t usbInterfaceRef)
{
    IOReturn				err;
    IOCFPlugInInterface 		**iodev;		// requires <IOKit/IOCFPlugIn.h>
    IOUSBInterfaceInterface 		**intf;
    SInt32 				score;
    UInt8				numPipes;


    err = IOCreatePlugInInterfaceForService(usbInterfaceRef, kIOUSBInterfaceUserClientTypeID, kIOCFPlugInInterfaceID, &iodev, &score);
    if (err || !iodev)
    {
	printf("dealWithInterface: unable to create plugin. ret = %08x, iodev = %p\n", err, iodev);
	return;
    }
    err = (*iodev)->QueryInterface(iodev, CFUUIDGetUUIDBytes(kIOUSBInterfaceInterfaceID), (LPVOID)&intf);
    (*iodev)->Release(iodev);				// done with this
    if (err || !intf)
    {
	printf("dealWithInterface: unable to create a device interface. ret = %08x, intf = %p\n", err, intf);
	return;
    }
    err = (*intf)->USBInterfaceOpen(intf);
    if (err)
    {
	printf("dealWithInterface: unable to open interface. ret = %08x\n", err);
	return;
    }
    err = (*intf)->GetNumEndpoints(intf, &numPipes);
    if (err)
    {
	printf("dealWithInterface: unable to get number of endpoints. ret = %08x\n", err);
	(*intf)->USBInterfaceClose(intf);
	(*intf)->Release(intf);
	return;
    }
 
    if (numPipes)  dealWithPipes(intf, numPipes);

    
    err = (*intf)->USBInterfaceClose(intf);
    if (err)
    {
	printf("dealWithInterface: unable to close interface. ret = %08x\n", err);
	return;
    }
    err = (*intf)->Release(intf);
    if (err)
    {
	printf("dealWithInterface: unable to release interface. ret = %08x\n", err);
	return;
    }
}


void dealWithDevice(io_service_t usbDeviceRef)
{
    IOReturn				err;
    IOCFPlugInInterface 		**iodev;		// requires <IOKit/IOCFPlugIn.h>
    IOUSBDeviceInterface 		**dev;
    SInt32 				score;
    UInt8				numConf;
    IOUSBConfigurationDescriptorPtr	confDesc;
    IOUSBFindInterfaceRequest		interfaceRequest;
    io_iterator_t			iterator;
    io_service_t			usbInterfaceRef;
    
    err = IOCreatePlugInInterfaceForService(usbDeviceRef, kIOUSBDeviceUserClientTypeID, kIOCFPlugInInterfaceID, &iodev, &score);
    if (err || !iodev)
    {
	printf("dealWithDevice: unable to create plugin. ret = %08x, iodev = %p\n", err, iodev);
	return;
    }
    err = (*iodev)->QueryInterface(iodev, CFUUIDGetUUIDBytes(kIOUSBDeviceInterfaceID), (LPVOID)&dev);
    (*iodev)->Release(iodev);				// done with this
    if (err || !dev)
    {
	printf("dealWithDevice: unable to create a device interface. ret = %08x, dev = %p\n", err, dev);
	return;
    }
    err = (*dev)->USBDeviceOpen(dev);
    if (err)
    {
	printf("dealWithDevice: unable to open device. ret = %08x\n", err);
	return;
    }
    err = (*dev)->GetNumberOfConfigurations(dev, &numConf);
    if (err || !numConf)
    {
	printf("dealWithDevice: unable to obtain the number of configurations. ret = %08x\n", err);
        (*dev)->USBDeviceClose(dev);
        (*dev)->Release(dev);
	return;
    }
//    printf("dealWithDevice: found %d configurations\n", numConf);

    // Changed this to the second Descriptor ...  !!!
    err = (*dev)->GetConfigurationDescriptorPtr(dev, 1, &confDesc);  // get the first config desc (index 1) <0 is HID, coontrol ...> >
    if (err)
    {
	printf("dealWithDevice:unable to get config descriptor for index 0\n");
        (*dev)->USBDeviceClose(dev);
        (*dev)->Release(dev);
	return;
    }
    err = (*dev)->SetConfiguration(dev, confDesc->bConfigurationValue);
    if (err)
    {
	printf("dealWithDevice: unable to set the configuration\n");
        (*dev)->USBDeviceClose(dev);
        (*dev)->Release(dev);
	return;
    }
    
    interfaceRequest.bInterfaceClass    = kIOUSBFindInterfaceDontCare;		// requested class
    interfaceRequest.bInterfaceSubClass = kIOUSBFindInterfaceDontCare;		// requested subclass
    interfaceRequest.bInterfaceProtocol = kIOUSBFindInterfaceDontCare;		// requested protocol
    interfaceRequest.bAlternateSetting  = kIOUSBFindInterfaceDontCare;		// requested alt setting
    
    err = (*dev)->CreateInterfaceIterator(dev, &interfaceRequest, &iterator);
    
    if (err){
	printf("dealWithDevice: unable to create interface iterator\n");
        (*dev)->USBDeviceClose(dev);
        (*dev)->Release(dev);
	return;
    }
    
    while ( (usbInterfaceRef = IOIteratorNext(iterator)) )
    {
	dealWithInterface(usbInterfaceRef);	
	IOObjectRelease(usbInterfaceRef);				// no longer need this reference
    }
    
    IOObjectRelease(iterator);
    iterator = 0;

    err = (*dev)->USBDeviceClose(dev);
    if (err)
    {
	printf("dealWithDevice: error closing device - %08x\n", err);
	(*dev)->Release(dev);
	return;
    }
    err = (*dev)->Release(dev);
    if (err)
    {
	printf("dealWithDevice: error releasing device - %08x\n", err);
	return;
    }
}



int ReadFromPic (void)
{
    kern_return_t		err;
    CFMutableDictionaryRef 	matchingDictionary = 0;		// requires <IOKit/IOKitLib.h>

    SInt32			idVendor = 1240;
    SInt32			idProduct = 50;
    
    CFNumberRef			numberRef;
    io_iterator_t 		iterator = 0;
    io_service_t		usbDeviceRef;
    
     
    [[NSNotificationCenter defaultCenter] postNotificationName: @"status" object:[NSString stringWithCString:"Looking For PicKit ..."]  ];
       
    
    err = IOMasterPort(MACH_PORT_NULL, &masterPort);				
    if (err)
    {
        printf("USBSimpleExample: could not create master port, err = %08x\n", err);
        return err;
    }
    matchingDictionary = IOServiceMatching(kIOUSBDeviceClassName);	// requires <IOKit/usb/IOUSBLib.h>
    if (!matchingDictionary)
    {
        printf("USBSimpleExample: could not create matching dictionary\n");
        return -1;
    }
    numberRef = CFNumberCreate(kCFAllocatorDefault, kCFNumberSInt32Type, &idVendor);
    if (!numberRef)
    {
        printf("USBSimpleExample: could not create CFNumberRef for vendor\n");
        
        return -1;
    }
    
    CFDictionaryAddValue(matchingDictionary, CFSTR(kUSBVendorID), numberRef);
    CFRelease(numberRef);
    numberRef = 0;
    numberRef = CFNumberCreate(kCFAllocatorDefault, kCFNumberSInt32Type, &idProduct);
    if (!numberRef)
    {
        printf("USBSimpleExample: could not create CFNumberRef for product\n");
        return -1;
    }
    CFDictionaryAddValue(matchingDictionary, CFSTR(kUSBProductID), numberRef);
    CFRelease(numberRef);
    numberRef = 0;
    
    err = IOServiceGetMatchingServices(masterPort, matchingDictionary, &iterator);
    matchingDictionary = 0;			// this was consumed by the above call
    
    while ( (usbDeviceRef = IOIteratorNext(iterator)) )
    {
        [[NSNotificationCenter defaultCenter] postNotificationName: @"status" object:[NSString stringWithCString:"Found PicKit ..."]  ];
        
	//printf("Found device %p\n", (void*)usbDeviceRef);
	dealWithDevice(usbDeviceRef);
	IOObjectRelease(usbDeviceRef);			// no longer need this reference
    }
    
    IOObjectRelease(iterator);
    iterator = 0;
    
    mach_port_deallocate(mach_task_self(), masterPort);
    return 0;
}

