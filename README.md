hexadisplay
===========

![Image of the display](pictures/hexadisplay.jpg)

Setup
-----

### Parts list:

- D1 Mini ESP32
- A **beefy** power supply. 5V 4A. (Maybe 12V works fine too, but haven't it tested yet)
- Clip frame 70x100 (e.g. [this one](https://www.amazon.de/FlexiPeople-Cliprahmen-Bilderrahmen-Plakatrahmen-Rahmenlos/dp/B07ZXPSWK7/))
- ~3 spools of white filament. PLA
- 1 spool of black filament. PLA
- 2x WS2812B strips. Length: 5m. 30 LEDs per meter.
- Aluminiom foil
- bare wire (I used paperclips)

### 3D Printing

Here be dragons.
For now look in the scad folder.

### Wiring diagram

Here be dragons.

### Assembly

Here be dragons.

### Firmware

Prereq:

- Make sure platformio is installed
- Connect the ESP32 to the usb port

```bash
vi firmware/src/config.h # adapt for your needs
printf '#define WIFI_PASSWORD "%s"' "$(head -n1)" > firmware/src/config_secure.h
# Enter your wifi password
cd firmware
make upload
```

### send a picture

Prereq:

- Firmware is flashed
- rust and cargo are installed

```bash
cd clients/send_image
cargo run 192.168.69.155:1234 /path/to/file
```
