#include "config.h"
#include "panel.h"
#include <Arduino.h>
#include <WiFi.h>
#include <WiFiUdp.h>
#include <CRC8.h>

#define PACKET_SIZE_MAX 2048

WiFiUDP udp;
CRC8 crc;
static char *buffer;

void
udp_setup() {
	udp.begin(WiFi.localIP(), UDP_PORT);
	buffer = new char[PACKET_SIZE_MAX];
}

void
udp_process() {

	int packet_size = udp.parsePacket();
	if (packet_size == 0) {
		return;
	}


	int want_crc = udp.read();
	if (want_crc == 0) {
		udp.flush();
		return;
	}
	crc.reset();
	int cmd = udp.read();
	if (cmd == -1) {
		udp.flush();
		return;
	}

	crc.add(cmd);
	int len = udp.read(buffer, PACKET_SIZE_MAX);
	crc.add((uint8_t *)buffer, len);
	if (want_crc == crc.getCRC()) {
		panel_blit((enum Command)cmd, buffer, len);
	} else {
		Serial.print("CRC mismatch: ");
		Serial.print(want_crc);
		Serial.print(" != ");
		Serial.println(crc.getCRC());
	}
	udp.flush();
}
