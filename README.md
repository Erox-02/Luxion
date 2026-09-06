# Luxion 

A custom drone flight controller designed from scratch.

## looks
---
![pcb](assets/pcb_v6.png) 
---
![3d_mod](assets/3d_mod.png)
---

## Hardware

- ESP32-S3-WROOM-1U-N16R8
- MPU6500 
- 4× BL5612 H-bridge motor drivers
- TPS63020 3.3V buck-boost
- USB(technically but not really)
- external antenna
- 1S 3.7V 600mAh lipo
- and a lot of caps 
- a diode
- 1x 560k and 100k ohm resistors

## status

- [x] schematic
- [x] layout & routing
- [x] frame
- [ ] PCB fab
- [ ] assembly
- [ ] firmware
- [ ] First test

## Helper

Remote on the way

---
## Bill of Materials

| Part | Qty | Link |
|---|---:|---|
| BL5612-BL H-Bridge Motor Driver | 4 | [Robu](https://robu.in/product/bl5612-blshanghai-belling-h-bridge-motor-driver-ic-1-5a-sop-8/) |
| MPU6500 6-Axis IMU | 1 | [Robu](https://robu.in/product/mpu6500-gyroscope-accelerometer-digital-motion-processor-dmp-6-axis-motion-sensor-with-i2c-spi-interface/) |
| ESP32-S3-WROOM-1U-N16R8 | 1 | [Robu](https://robu.in/product/espressif-esp32-s3-wroom-1u-n16r8-module/) |
| TPS63020DSJR | 1 | [Robu](https://robu.in/product/tps63020dsjr-texas-instruments-boost-type-adjustable-1-2v5-5v-1-8v5-5v-vson-14-ep3x4-dc-dc-converters-rohs/) |
| SRN8040-1R5Y 1.5µH Inductor | 1 | [Robu](https://robu.in/product/srn8040-1r5y-bourns-srn8040-1r5y-power-inductor-smd-1-5-%c2%b5h-7-a-shielded-8-2-a-srn8040-series/) |
| 22µF 16V 1206 X5R Capacitor | 1 | [Robu](https://robu.in/product/1206x226k160ct-samsung-smd-multilayer-ceramic-capacitor-22-%c2%b5f-16-v-1206-3216-metric-%c2%b1-10-x5r/) |
| 100nF 25V 0402 X7R Capacitor | 5 | [Robu](https://robu.in/product/im02b104k250nb-fh-smd-multilayer-ceramic-capacitor-0-1-%c2%b5f100-nf-25-v-0402-1005-metric-%c2%b1-10-x7r-cl/) |
| 10µF 10V 0805 X5R Capacitor | 5 | [Robu](https://robu.in/product/cs2012x5r106m100nre-samwha-10-%c2%b5f-10v-x5r-0805-multilayer-ceramic-capacitors-mlcc-smd-smt-rohs/) |
| 100kΩ 0603 Resistor | 1 | [Robu](https://robu.in/product/100k-ohm-1-4w-0603-surface-mount-chip-resistor-pack-of-100/) |
| 560kΩ 0603 Resistor | 1 | [Robu](https://robu.in/product/erjp03j564v-panasonic-200mw-thick-film-resistors-%c2%b15-%c2%b1200ppm-%e2%84%83-560k%cf%89-0603-chip-resistor-surface-mount-rohs/) |
| 1N4148WS SOD-323 Diode | 1 | [Robu](https://robu.in/product/1n4148ws-sod-323-805-diodereel-of-3000/) |
| 3V Active Buzzer | 1 | [Robu](https://robu.in/product/3v-active-electromagnetic-buzzer-pack-of-5/) |
| 8520 Coreless Motor | 4 | [Robu](https://robu.in/product/8520-magnetic-micro-coreless-motor-for-micro-quadcopters-2xcw-2xccw/) |
| Bonka 3.7V 600mAh 25C 1S LiPo | 1 | [Robu](https://robu.in/product/bonka-3-7v-600mah-25c-1s-lithium-polymer-battery-pack/) |

---

> I use arch btw
