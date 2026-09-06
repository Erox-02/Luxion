# Luxion 

Luxion is a custom made drone made with rust unlike standard c++ drones .
It uses a custom pcb, ESP32-S3(bare) , MPU6500-IMU(breakout as it costs less), four motor drivers, and a 1S LiPo to power the drone.
The firmware is still on the way , gonna finish it by next week .

## Function

It is a remote(phone/remote) controlled drone made from absolutely nothing , with 8520 motors and 65mm props it can easily take off with 2+ thrust-mass ratio . It is ideal for mini quadcoptor with mountable sensors ,as it has a lot of gpio's left any sensor can be easily mounted if not too heavy .

## Why it exists

from when i saw the pluto x  , a programable drone with esp 12f and stm32 , i always wanted to make one myself but better and cheaper thts why luxion exists today. I used esp32 instead of 2 mcu's , gonna use esp-now instead of wifi and a lot but the best part i didnt rely on the pluto x's
architecture for luxion but made my whole own .

## Design

### 3D Model

[download](3d/luxion_frame.stl)

also pics here :

![3dmod](assets/3d_mod.png)
---
![3dmod](assets/model.png)
---

### PCB

![pcb](assets/pcb_v6.png)

### Schematic

![schematic](assets/sch_v4.png)

## Firmware

I intend to use a firmware written in rust no std but it is still on the way.

## How to assemble

Solder all the components on the pcb ,
also You might need to solder a jst socket according to your battery 
(more detailed explanation after i do it myself first lol)

## Credits

Frame is taken from :

```
https://www.thingiverse.com/thing:5013951
```

## Bill of Materials

| Part | Used | Purchase / MOQ | Link |
|---|---:|---:|---|
| BL5612-BL H-Bridge Motor Driver | 4 | 4 | [Robu](https://robu.in/product/bl5612-blshanghai-belling-h-bridge-motor-driver-ic-1-5a-sop-8/) |
| MPU6500 6-Axis IMU | 1 | 1 | [Robu](https://robu.in/product/mpu6500-gyroscope-accelerometer-digital-motion-processor-dmp-6-axis-motion-sensor-with-i2c-spi-interface/) |
| ESP32-S3-WROOM-1U-N16R8 | 1 | 1 | [Robu](https://robu.in/product/espressif-esp32-s3-wroom-1u-n16r8-module/) |
| 3V Active Electromagnetic Buzzer | 1 | 1 | [Robu](https://robu.in/product/3v-active-electromagnetic-buzzer-pack-of-5/) |
| TPS63020DSJR | 1 | 1 | [Robu](https://robu.in/product/tps63020dsjr-texas-instruments-boost-type-adjustable-1-2v5-5v-1-8v5-5v-vson-14-ep3x4-dc-dc-converters-rohs/) |
| Bonka 3.7V 600mAh 25C 1S LiPo | 1 | 1 | [Robu](https://robu.in/product/bonka-3-7v-600mah-25c-1s-lithium-polymer-battery-pack/) |
| SRN8040-1R5Y 1.5µH Inductor | 1 | 1 | [Robu](https://robu.in/product/srn8040-1r5y-bourns-srn8040-1r5y-power-inductor-smd-1-5-%c2%b5h-7-a-shielded-8-2-a-srn8040-series/) |
| 22µF 16V 1206 X5R Capacitor | 1 | 25 (MOQ) | [Robu](https://robu.in/product/1206x226k160ct-samsung-smd-multilayer-ceramic-capacitor-22-%c2%b5f-16-v-1206-3216-metric-%c2%b1-10-x5r/) |
| 100nF 25V 0402 X7R Capacitor | 5 | 25 (MOQ) | [Robu](https://robu.in/product/im02b104k250nb-fh-smd-multilayer-ceramic-capacitor-0-1-%c2%b5f100-nf-25-v-0402-1005-metric-%c2%b1-10-x7r-cl/) |
| 10µF 10V 0805 X5R Capacitor | 5 | 20 (MOQ) | [Robu](https://robu.in/product/cs2012x5r106m100nre-samwha-10-%c2%b5f-10v-x5r-0805-multilayer-ceramic-capacitors-mlcc-smd-smt-rohs/) |
| 100kΩ 0603 Resistor | 1 | 28 (MOQ) | [Robu](https://robu.in/product/100k-ohm-1-4w-0603-surface-mount-chip-resistor-pack-of-100/) |
| 560kΩ 0603 Resistor | 1 | 28 (MOQ) | [Robu](https://robu.in/product/erjp03j564v-panasonic-200mw-thick-film-resistors-%c2%b15-%c2%b1200ppm-%e2%84%83-560k%cf%89-0603-chip-resistor-surface-mount-rohs/) |
| 1N4148WS SOD-323 Diode | 1 | 13 (MOQ) | [Robu](https://robu.in/product/1n4148ws-sod-323-805-diodereel-of-3000/) |
| 8520 Coreless Motor | 4 | 4 | [Robu](https://robu.in/product/8520-magnetic-micro-coreless-motor-for-micro-quadcopters-2xcw-2xccw/) |
| 150mm FrSky Receiver Antenna IPEX4 | 1 | 1 | [Robu](https://robu.in/product/150mm-frsky-receiver-antenna-new-version-ipex4/) |






