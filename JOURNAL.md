---
title: "Luxion"
author: "Dipanjan Dutta"
description: "A tiny esp32 drone but in rust"
created_at: "2026-09-04"
---

# Sept 4 : Started the schematic

Today i started the first part of lux , i simply designed the initial base(i mean schematic only) of pcb now gonna polish tht . Heres an img :
![schematic](assets/sch.png)
So to be frank it took me hours to make the decisions for the h bridge driver , the buck converter and both of them not out of stock everywhere
, i chose bl5612 for the h bridge driver , looks good enough but for the buck , i chosed ams1117 first then came to know its really inefficient and a 1s battery cant reliably suppy for it ,so i went to TPS63020DSJR  but soon i came to realize why it wasnt a good choice for me . THe wiring of it is really complex thts all still i somehow did it , now the next for tomorrow , i need to make the rails for ttl and start the pcb maybe? Also today i connected the mpu6500 ,it might look in complete but i intend to solder the breakout directly on my pcb(ah cant find bare cheaper than the breakout) so only 4 pins(gnd vcc scl and sda) are good enough no need for more hassle .

**Total time spent: 4 hours**