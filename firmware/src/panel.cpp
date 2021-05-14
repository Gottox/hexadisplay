#include <Arduino.h>
#include "config.h"
#include <FastLED.h>

#define BANK1_PIN 33
#define BANK2_PIN 14
#define BANK3_PIN 18
#define BANK4_PIN 19
#define BANK5_PIN 23
#define BANK6_PIN 5
#define BANK7_PIN 22
#define BANK8_PIN 21
#define BANK9_PIN 27

enum Command {
	CMD_NOOP = 0,
	CMD_RESET = 1 << 0,
	CMD_CLRSCN = 1 << 1,
	CMD_SHOW = 1 << 2,
};

const uint16_t row_offset = 22;
const uint16_t bank_count = 9;
const uint16_t leds_per_bank = 45;
const uint16_t led_count = leds_per_bank * bank_count;
const uint16_t pixel_count = bank_count * (leds_per_bank + 1); // one dead pixel per bank due to hexagonal

static uint8_t current_buffer_fill = 0;
static uint8_t current_buffer_show = 0;
CRGB draw_buffer[PANEL_BUFFER_COUNT][pixel_count];
CRGB leds[led_count];
uint16_t cursor = 0;

void
panel_draw() {
	for (int16_t bank = 0, buffer_index = 0; bank < bank_count; bank++) {
		uint16_t target_offset = bank * leds_per_bank;
		for (int16_t led = leds_per_bank - row_offset; led < leds_per_bank; led++, buffer_index++) {
			leds[target_offset + led] = draw_buffer[current_buffer_show][buffer_index];
		}
		buffer_index++; // Skip one pixel
		for (int16_t led = row_offset; led >= 0; led--, buffer_index++) {
			leds[target_offset + led] = draw_buffer[current_buffer_show][buffer_index];
		}
	}
	current_buffer_show++;
	current_buffer_fill++;
	FastLED.show();
}

void
panel_clear() {
	fill_solid(draw_buffer[current_buffer_fill], led_count, CRGB::Black);
}

void
panel_set_led(int led, int r, int g, int b) {
	draw_buffer[current_buffer_fill][led] = CRGB(r, g, b);
}


void
panel_blit(enum Command cmd, char *buffer, int len) {
	// Drop command if it doesn't contains pixeldata
	if (len % 3 != 0) {
		return;
	}
	Serial.print("Draw: ");

	if (cmd & CMD_RESET) {
		Serial.print("Reset ");
		cursor = 0;
	}
	if (cmd & CMD_CLRSCN) {
		panel_clear();
		Serial.print("ClearScreen ");
	}
	for (int i = 0; i < len && cursor < pixel_count; i+=3, cursor++) {
		draw_buffer[current_buffer_fill][cursor] = CRGB(buffer[i], buffer[i+1], buffer[i+2]);
	}
	if (cmd & CMD_SHOW) {
		panel_draw();
	}
	Serial.println();
}

void panel_setup() {
	LEDS.addLeds<WS2811, BANK1_PIN, GRB>(&leds[leds_per_bank * 0],
										 leds_per_bank);
	LEDS.addLeds<WS2811, BANK2_PIN, GRB>(&leds[leds_per_bank * 1],
										 leds_per_bank);
	LEDS.addLeds<WS2811, BANK3_PIN, GRB>(&leds[leds_per_bank * 2],
										 leds_per_bank);
	LEDS.addLeds<WS2811, BANK4_PIN, GRB>(&leds[leds_per_bank * 3],
										 leds_per_bank);
	LEDS.addLeds<WS2811, BANK5_PIN, GRB>(&leds[leds_per_bank * 4],
										 leds_per_bank);
	LEDS.addLeds<WS2811, BANK6_PIN, GRB>(&leds[leds_per_bank * 5],
										 leds_per_bank);
	LEDS.addLeds<WS2811, BANK7_PIN, GRB>(&leds[leds_per_bank * 6],
										 leds_per_bank);
	LEDS.addLeds<WS2811, BANK8_PIN, GRB>(&leds[leds_per_bank * 7],
										 leds_per_bank);
	LEDS.addLeds<WS2811, BANK9_PIN, GRB>(&leds[leds_per_bank * 8],
										 leds_per_bank);
}

void
panel_process() {

}
