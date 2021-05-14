#include "config.h"
#include "panel.h"
#include <Arduino.h>
#include <Update.h>
#include <WebServer.h>
#include <base64.hpp>

WebServer http(HTTP_PORT);

const char *update_firmware_content =
	"<form method='POST' action='/update_firmware' enctype='multipart/form-data'>"
	"<input type='file' name='update'><input type='submit' value='Update'>"
	"</form>";

static void
http_blit_panel() {
	if (!http.hasArg("payload")) {
		http.send(400, "text/plain", "Missing payload");
		return;
	}
	Serial.println("Drawing via http");
	String payload_str = http.arg("payload");
	char *payload_base64 = (char *)malloc(payload_str.length() * sizeof(char));
	if (payload_base64 == NULL) {
		http.send(500, "text/plain", "Cannot malloc for payload_base64");
		return;
	}
	payload_str.toCharArray(payload_base64, payload_str.length());

	char *payload = (char *)malloc(payload_str.length() * sizeof(char));
	if (payload == NULL) {
		http.send(500, "text/plain", "Cannot malloc for payload");
		free((void *)payload);
		return;
	}
	int payload_len =
		decode_base64((unsigned char *)payload_base64, payload_str.length(),
					  (unsigned char *)payload);

	panel_blit((enum Command)payload[0], &payload[1], payload_len - 1);
	http.send(200, "text/plain", "Ok");
	free((void *)payload);
	free((void *)payload_base64);
}

static void
http_update_firmware_result() {
	http.sendHeader("Connection", "close");
	http.send(200, "text/plain", (Update.hasError()) ? "FAIL" : "OK");
	ESP.restart();
}

static void
http_update_firmware() {
	HTTPUpload &upload = http.upload();
	if (upload.status == UPLOAD_FILE_START) {
		Serial.setDebugOutput(true);
		Serial.printf("Update: %s", upload.filename.c_str());
		Serial.println();
		if (!Update.begin()) { // start with max available size
			Update.printError(Serial);
		}
	} else if (upload.status == UPLOAD_FILE_WRITE) {
		if (Update.write(upload.buf, upload.currentSize) !=
			upload.currentSize) {
			Update.printError(Serial);
		}
	} else if (upload.status == UPLOAD_FILE_END) {
		if (Update.end(true)) { // true to set the size to the current progress
			Serial.printf("Update Success: %u",
						  upload.totalSize);
			Serial.println("Rebooting...");
		} else {
			Update.printError(Serial);
		}
		Serial.setDebugOutput(false);
	} else {
		Serial.printf("Update Failed Unexpectedly (likely broken connection): "
					  "status=%d\n",
					  upload.status);
	}
}

void
http_setup() {
	http.begin();
	http.on("/", HTTP_POST, http_blit_panel);
	http.on("/update_firmware", HTTP_GET, []() {
		http.sendHeader("Connection", "close");
		http.send(200, "text/html", update_firmware_content);
	});
	http.on("/update_firmware", HTTP_POST, http_update_firmware_result,
			http_update_firmware);
}

void
http_process() {
	http.handleClient();
}
