#include <Arduino.h>
#include <WiFi.h>
#include <WiFiUdp.h>
#include "config.h"
#include "panel.h"

WiFiUDP udp;

void
udp_setup() {
	udp.begin(WiFi.localIP(), UDP_PORT);
}

void
udp_process() {
	char buffer[512];
	int packet_size = udp.parsePacket();
	if (packet_size == 0) {
		return;
	}

	int cmd = udp.read();
	if (cmd != -1) {
		int len = udp.read(buffer, sizeof buffer);
		panel_blit((enum Command)cmd, buffer, len);
	}
	udp.flush();
}
