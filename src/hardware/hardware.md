# Hall effect sensor
Most hall sensors for microcontrollers have a digital output (low when no magnet, high when magnet). [The one I'm eyeing](https://www.otronic.nl/a-61364086/sensors/lineaire-hall-sensor-voor-arduino-esp32-esp8266/) has both analog and digital pins, perfect for tuning.

We need a time reference to accurately "guess" the speed of the bicycle based on the time between inputs. If we can get the polling rate high enough, we could also look at the "analog pulse width" (How long the magnet is close enough to the sensor to detect it).

Got it working on it's digital pin, the GPIO pin you're reading with needs the pull low resistor enabled (the pin is active low).

Got it working in Rust as well. Interrupt driven would make it more accurate, I've still got to calculate how accurate the timing has got to be.
Signal makes it fine through 1k ohms, power doesnt. I think higer driving voltage will solve that. Hope I don't kill the sensor in the process hahah.


## Calculating speed:

25 km h, sensibe max speed for flat road cycling. : 7 m/s 
wheel diameter = 28 inches.

wheel perimieter = 2pi * r = pi * r = 88 inches = 223cm.
11 m/s for easier math = 5 wheel turns per second at max speed of close to 40 km/h
200ms between pulses. 

working backwards: we're measuring the time it takes the bike to roll 2.23 meters. 

measured time between pulses: 




# Lights
I've got SK6812 leds, more than we need. We could either use the Adafruit neopixel library (I HOPE there is one for rust), or implement led control from scratch. [here](https://cdn-shop.adafruit.com/product-files/1138/SK6812+LED+datasheet+.pdf) is a spec sheet. I also got WS2812s, but those are less bright. 

These leds could also be used to include turn signals on the bike, for shenanigans I guess.

"NeoPixels must be connected to GPIO10, GPIO12, GPIO18 or GPIO21 to work! GPIO18 is the standard pin."


# Display
The display is [this model](https://www.tinytronics.nl/shop/en/displays/tft/5-inch-lcd-display-800*480-pixels-with-touchscreen-raspberry-pi-compatible).

It might pose a problem that it's mounts right onto GPIO, blocking all powerlines. The following pins remain "uncovered" as long as you can find short female cables. 
| this side|  towards| sd| card|
| ---|--- | ---|--- |
| GPIO0| (27) |(28) |GPIO1|
| GPIO5 |(29) |(30) |GND |
| GPIO6 |(31) |(32) |GPIO12|
|GPIO13 |(33) |(34) |GND|
|GPIO19 |(35) |(36) |GPIO16|
|GPIO26 |(37) |(38) |GPIO20|
|   GND |(39) |(40) |GPIO21|

