#include <Arduino.h>
#include <WiFi.h>
#include "config.h"
#include "panel.h"
#include "udp.h"
#include "http.h"

static void
wifi_setup() {
	WiFi.mode(WIFI_STA);
	WiFi.setHostname(WIFI_HOSTNAME);
	WiFi.begin(WIFI_SSID, WIFI_PASSWORD);

	panel_clear();
	for(int i = 0; WiFi.status() != WL_CONNECTED; i++) {
		if (i % 2 == 0) {
			panel_set_led(-1, 255, 0, 0);
		} else {
			panel_set_led(-1, 0, 255, 0);
		}
		panel_draw();
		delay(500);
	}
	panel_set_led(-1, 0, 0, 255);
	panel_draw();

	Serial.print("Connected, IP address: ");
	Serial.println(WiFi.localIP());
}

void
setup() {
	Serial.begin(115200);
	panel_setup();
	wifi_setup();
	udp_setup();
	http_setup();
}

void
loop() {
	udp_process();
	http_process();
	panel_process();
}
