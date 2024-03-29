# STEM4ukraine-Saint-Javelin

A simple PCB with sound effects and the light show set off by pressing of the button!

An ideal school STEM project demonstrating use of GPIO for detecting button presses with software debouncing, charlieplexxing of multiple LEDs and playing music on the Piezo speaker.

The STEM4ukraine-Saint-Javelin-v1.ino firmware is written in the Arduino IDE and requires the Microcore attiny13 library, uses the 1.2MHz internal oscillator, with EEPROM retained and no bootloader. Feel free to modify it!

Celebrate Saint Javelin while you play the Ukrainian national anthem on the built in speaker!

The front of the PCB

![prototype front](images/SaintJavelinFront.jpg)

The rear of the prototype PCB

![prototype back](images/SaintJavelinReverse.jpg)

The schematic

![Schematic](images/STEM4ukraine-Saint-Javelin.svg)

Bill of materials:

- U1:  attiny13
- C1:  100nF
- R1,R2,R3:  220R
- R4:  10k
- R5:  47k
- LED1-7:  yellow 3mm LED
- SW1: momentary switch
- PIEZO:  piezo speaker
- USB1:  180 degree vertical through hole type B USB socket
- EXTSW:  optional 2.5mm header for external wiring for switch
- EXT5V:  optional 2.5mm header for external wiring for 5V power
