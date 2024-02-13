/*
 *
 *   Copyright (C) 2024 STEM4ukraine
 *   Website https://github.com/STEM4ukraine
 *
 *   This program is free software: you can redistribute it and/or modify
 *   it under the terms of the GNU General Public License as published by
 *   the Free Software Foundation, either version 3 of the License, or
 *   (at your option) any later version.
 *
 *   This program is distributed in the hope that it will be useful,
 *   but WITHOUT ANY WARRANTY; without even the implied warranty of
 *   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *   GNU General Public License at <http://www.gnu.org/licenses/> 
 *   for more details.
 *
 */

#include <avr/pgmspace.h>

// the code has been uploaded using a USB-ASP programmer 
// and the "USB-ASP slow (Microcore)" setting in the arduino IDE

// Microcore attiny13 selected
// using 1.2MHz internal oscillator
// EEPROM retained
// no bootloader

// notes B up to A defined as
// MSB XXXX0000 as pitch index into the (note period in uS) array
// LSB 0000XXXX as note duration

#define A_2 98 //6*16 + 2  // 568  // freq 880
#define A_1 97 //6*16 + 1  // 568  // freq 880
#define A_0 96 //6*16 + 0  // 568  // freq 880
#define G1  81 //5*16 + 1  // 638  // freq 784
#define G0  80 //5*16 + 0  // 638  // freq 784
#define FS3 67 //4*16 + 3  // 676  // freq 740
#define FS2 66 //4*16 + 2  // 676  // freq 740
#define FS1 65 //4*16 + 1  // 676  // freq 740
#define FS0 64 //4*16 + 0  // 676  // freq 740
#define E1  49 //3*16 + 1  // 758  // freq 659
#define E0  48 //3*16 + 0  // 758  // freq 659
#define D1  33 //2*16 + 1  // 851  // freq 587
#define D0  32 //2*16 + 0  // 851  // freq 587
#define CS1 17 //1*16 + 1  // 902  // freq 554
#define CS0 16 //1*16 + 0  // 902  // freq 554
#define B3  3  //0*16 + 3  // 1012 // freq 494
#define B2  2  //0*16 + 2  // 1012 // freq 494
#define B0  0  //0*16 + 0  // 1012 // freq 494

#define NOTE_DELAY_MS 10 // between notes

// first 8 bars of anthem is first 18 notes, followed by first 14 notes, then 19th note played twice

static const uint16_t note_periods[7] PROGMEM = {1012, 902, 851, 758, 676, 638, 568};

static const uint8_t notesPDtest[8] PROGMEM = {B0, CS0, D0, E0, FS0, G0, A_0, G0};

// first 8 bars
static const uint8_t notes12PD[18] PROGMEM = {FS2, FS0, FS0, E0, FS0, G0, A_2, G0, FS1, E1, D1, FS1, CS1, FS1, B2, CS0, D1, E1};

// 9th-12th bars + 13 into next 3 bars
static const uint8_t notes34PD[18] PROGMEM = {CS1, CS1, FS0, E0, D0, CS0, B0, CS0, D0, B0, CS1, CS1, D1, D1, E1, E1, FS3, FS3};

// final 9 notes of 15-16th bars
static const uint8_t notes4PD[9] PROGMEM = {FS1, CS1, FS1, B2, CS0, D0, E0, FS0, G0};

// next 4 bars + repeat first 11 into next 3 bars
static const uint8_t notes56PD[18] PROGMEM = {A_2, A_0, A_1, FS1, E1, E1, A_0, G0, FS0, E0, D1, D1, E1, E1, FS2, E0, FS1, G1};

// final 5 notes = end of 7th, 8th bar, i.e. index 
static const uint8_t notes6PD[5] PROGMEM = {FS1, CS1, FS1, B3, B3};

uint8_t led = 0;
uint8_t button = 0;

void setup() {
  pinMode(0, OUTPUT);
  pinMode(1, OUTPUT);
  pinMode(2, OUTPUT);
  pinMode(3, OUTPUT);
  pinMode(4, OUTPUT);
  pinMode(5, INPUT);
}

void turn_on_LED(int duration_us) { // n = 0-6
  switch (led%7) {
    case 0:
      PORTB = 0b00001101;
      break;
    case 1: 
      PORTB = 0b00001011;
      break;
    case 2: 
      PORTB = 0b00000111;
      delayMicroseconds(duration_us);
      PORTB = 0b00000001;
      break;
    case 3: 
      PORTB = 0b00001010;
      break;
    case 4: 
      PORTB = 0b00000110;
      delayMicroseconds(duration_us);
      PORTB = 0b00000010;
      break;
    case 5: 
      PORTB = 0b00001100;
      break;
    case 6:
      PORTB = 0b00000110;
      break;
    default:
      break;
  }
  delayMicroseconds(duration_us);
}

void turn_off_LEDs(int duration_us) {
  PORTB &= ~0b00001111;
  delayMicroseconds(duration_us);
}
 
void cycle_LEDs_once(int onduration_us, int offduration_us) {
  for (led = 0; led < 7; led++) {
      turn_on_LED(onduration_us);
  }
  turn_off_LEDs(offduration_us); // accentuate off time
  turn_off_LEDs(offduration_us);
  turn_off_LEDs(offduration_us);
}

void cycle_LEDs_n_times(int onduration_us, int offduration_us, int n) {
  for (int j = 0; j < n; j++) {
    cycle_LEDs_once(onduration_us,offduration_us);
  }
}

byte poll_button() {
  button = button|(analogRead(A0) < 1000);
  return button;
}

void pulse_LEDs() {
  int ramp_cycles = 5;
  int cycles = 20;
  int led_off_max_us = 234;
  int us_per_LED = 4;
  int i,k;
  for(k = 0; k < led_off_max_us && !poll_button(); k += 13) {
    for (i = 0; i < ramp_cycles; i++) {
      cycle_LEDs_n_times(us_per_LED,k,cycles);
    }
  }
  for(k = led_off_max_us; k > 0 && !poll_button(); k -= 13) {
    for (i = 0; i < ramp_cycles; i++) {
      cycle_LEDs_n_times(us_per_LED,k,cycles);
    }
  }
}

void play_note(const uint8_t * rawnote) {
  int periods;
  uint8_t k, raw_note, note_dur;
  uint16_t note_per, cycles, cycles_;

  raw_note = pgm_read_byte(rawnote);
  note_dur = raw_note & 0b00001111;
  periods = ((raw_note>>4) & 0b00001111);
  memcpy_P (&note_per, note_periods+periods, 2);

  // this would need modification for notes outside of the B4-A5 note range
  cycles = (0x82*500)/note_per; // just avoids overflow for B4-A5 note range
  turn_on_LED(2);
  for (k = 0; k < (note_dur+1); k++) {
    //turnOnLed(2);
    for (int i = 0; i < 3; i++) { // this, times overflow constrained cycles
      cycles_ = cycles;           // gives required duration
      while (cycles_ > 0) {
        PORTB |= (1 << PB4);
        delayMicroseconds(note_per);
        
        PORTB &= ~(1 << PB4);
        delayMicroseconds(note_per);
        cycles_--;
      }
    }
  }
  poll_button();
  led++;
  delay(NOTE_DELAY_MS);
}

void play_anthem() {
  char j;
  char raw_note;
  uint16_t note_dur, note_per;

//  for (j = 0; j < 8; j++) {
//    play_note(notesPDtest+j);
//  }

  // play first 18 notes and then the first 14 again
  for (j = 0; j < 32 && !poll_button(); j++) {
    play_note(notes12PD+(j%18));
  }

  // play last note twice
  for (j = 3; j < 5 && !poll_button(); j++) {
    play_note(notes6PD+j);
  }

  for (j = 0; j < 31 && !poll_button(); j++) {
    play_note(notes34PD+(j%18));
  }

  for (j = 0; j < 9 && !poll_button(); j++) {
    play_note(notes4PD+j);
  }

  for (j = 0; j < 29 && !poll_button(); j++) {
    play_note(notes56PD+(j%18));
  }

  for (j = 0; j < 5 && !poll_button(); j++) {
    play_note(notes6PD+j);
  }

}

// the main loop which repeatedly calls the LED lighting code, polls the button
// and plays the anthem if the button has been pressed
void loop() {

  button = 0;

  // allow break after every note, if button pressed.
  while(1) {
    if (poll_button()) {
      button = 0;
      delay(200); // debounce
      play_anthem();
      button = 0; // in case button pressed during anthem to finish it early
      delay(200); // debounce
    }
    pulse_LEDs();
  }
}
