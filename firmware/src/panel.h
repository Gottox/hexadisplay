/**
 * @author      : tox (tox@rootkit)
 * @file        : panel
 * @created     : Tuesday Mar 16, 2021 20:29:51 CET
 */

#ifndef PANEL_H

#define PANEL_H

enum Command {
	CMD_NOOP = 0,
	CMD_RESET = 1 << 0,
	CMD_CLRSCN = 1 << 1,
	CMD_SHOW = 1 << 2,
};

void panel_draw();

void panel_clear();

void panel_process(enum Command cmd, char *buffer, int len);

void panel_set_led(int led, int r, int g, int b);

void panel_setup();

#endif /* end of include guard PANEL_H */

