# Luxion 

Luxion is a custom made drone made with rust unlike standard c++ drones .
It uses a custom pcb, ESP32-S3-WROOM-1U , MPU6500-IMU(breakout as it costs less than the bare), four motor drivers, and a 1S LiPo to power the drone.
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

### Wiring

io4 -> bl1 ina(cw)
io5 -> bl1 inb(cw)
io6 -> bl2 ina(ccw)
io7 -> bl2 inb(ccw)
io15 -> bl3 ina(ccw)
io16 -> bl3 inb(ccw)
io17 -> bl4 ina (cw)
io18 -> bl4 inb (cw)
io19 -> d-
io20 -> d+
io46 -> int(mpu)
io10 -> ncs
io11 -> sda
io12 -> scl
io13 -> ad0

done thts for the mpu 

## Remote 

The remote's pcb is on the ./kicad/remote dir .
It currently uses the same power regualtion and mcu as the drone but it wasnt just merely copy pasted also the current bom doesnt have the joysticks , as i have mine already i didnt add tht to bom .

## Firmware

> still on the way but lemme add the progress here

I have written likely 60% of the firmware already before getting approved or building the physical drone .

Current stack have :
[erox@archbtw luxion]$ tree src 
src
├── filter.rs
├── main.rs
├── motor.rs
├── mpu.rs          // needs change currently
└── pid.rs

1 directory, 5 files

the mpu.rs is a custom driver(really basic) for the mpu6500 , the filte* take mpu's data(gyro and acc) then converts tht into yaw roll and pitch , the moto* haved ldec and controlls the motors state , the pid is just pid controller nothing more , and the main.rs integrates them all and runs them when needed either in loop or once .

### Whats remainin?

- esp now impl
- build.rs
- proper pid vals and calibration
- bl5612 calibration 

## How to assemble

Solder all the components from bom(except the antenna) on the pcb ,
also You might need to solder a jst socket according to your battery 
(more detailed explanation after i do it myself first lol)

## Credits

This project uses:

- KiCad
- Blender for 3D renders
- A opensource frame from the link below

[link](https://www.thingiverse.com/thing:5013951)

## JLCPCB order

![order](assets/jlc.png)

> the price might fluctuate over time

## Bill of Materials

Part,Quantity,Unit Price (INR),Unit Price (USD),Total (INR),Total (USD),Link
8520 Magnetic Micro Coreless Motors – 2xCW + 2xCCW pack,1,459.00,4.83,459.00,4.83,https://robu.in/product/8520-magnetic-micro-coreless-motor-for-micro-quadcopters-2xcw-2xccw/
100kΩ 0603 resistor (MOQ),28,0.37,0.00,10.36,0.11,https://robu.in/product/100k-ohm-1-4w-0603-surface-mount-chip-resistor-pack-of-100/
ESP32-S3-WROOM-1U-N16R8,2,469.00,4.94,938.00,9.87,https://robu.in/product/espressif-esp32-s3-wroom-1u-n16r8-module/
100nF 0402 X7R capacitor (MOQ),25,0.40,0.00,10.00,0.11,https://robu.in/product/im02b104k250nb-fh-smd-multilayer-ceramic-capacitor-0-1-%c2%b5f100-nf-25-v-0402-1005-metric-%c2%b1-10-x7r-cl/
BL5612-BL H-Bridge Motor Driver IC,4,31.00,0.33,124.00,1.31,https://robu.in/product/bl5612-blshanghai-belling-h-bridge-motor-driver-ic-1-5a-sop-8/
Bonka 3.7V 600mAh 25C 1S LiPo,1,499.00,5.25,499.00,5.25,https://robu.in/product/bonka-3-7v-600mah-25c-1s-lithium-polymer-battery-pack/
1N4148WS SOD-323 diode (MOQ),13,0.77,0.01,10.01,0.11,https://robu.in/product/1n4148ws-sod-323-805-diodereel-of-3000/
150mm FrSky IPEX4 antenna,2,42.00,0.44,84.00,0.88,https://robu.in/product/150mm-frsky-receiver-antenna-new-version-ipex4/
22µF 16V 1206 X5R capacitor (MOQ),25,0.40,0.00,10.00,0.11,https://robu.in/product/1206x226k160ct-samsung-smd-multilayer-ceramic-capacitor-22-%c2%b5f-16-v-1206-3216-metric-%c2%b1-10-x5r/
10µF 10V 0805 X5R capacitor (MOQ),20,0.52,0.01,10.40,0.11,https://robu.in/product/cs2012x5r106m100nre-samwha-10-%c2%b5f-10v-x5r-0805-multilayer-ceramic-capacitors-mlcc-smd-smt-rohs/
SRN8040-1R5Y 1.5µH 7A inductor,2,23.00,0.24,46.00,0.48,https://robu.in/product/srn8040-1r5y-bourns-srn8040-1r5y-power-inductor-smd-1-5-%c2%b5h-7-a-shielded-8-2-a-srn8040-series/
560kΩ 0603 resistor (MOQ),28,0.36,0.00,10.08,0.11,https://robu.in/product/erjp03j564v-panasonic-200mw-thick-film-resistors-%c2%b15-%c2%b1200ppm-%e2%84%83-560k%cf%89-0603-chip-resistor-surface-mount-rohs/
JST 2P Male + Female Terminal Connection Socket,1,30.00,0.32,30.00,0.32,https://robu.in/product/jst-2p-malefemale-terminal-connection-socket/
MPU-6050 3-Axis Accelerometer + Gyro,1,151.00,1.59,151.00,1.59,https://robu.in/product/mpu-6050-gyro-sensor-2-accelerometer/
TPS63020DSJR,2,124.00,1.31,248.00,2.61,https://robu.in/product/tps63020dsjr-texas-instruments-boost-type-adjustable-1-2v5-5v-1-8v5-5v-vson-14-ep3x4-dc-dc-converters-rohs/
3V Active Electromagnetic Buzzer – pack of 5,1,18.00,0.19,18.00,0.19,https://robu.in/product/3v-active-electromagnetic-buzzer-pack-of-5/
Luxion PCB (Y6),1,389.50,4.10,389.50,4.10,JLCPCB
Remote PCB (Y7),5,76.00,0.80,380.00,4.00,JLCPCB
Luxion frame – 3201PA-F Nylon,1,224.20,2.36,224.20,2.36,JLC3DP
TOTAL,,,,3633.55,38.25,





