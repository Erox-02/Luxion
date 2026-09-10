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

# Sept 5 : 

I already have the probs i need to solve but i made a new decision , i will just wire the d+ and d- on io20 and 19 , so no cp2102 as the esp already have the required pins , why waste money? 

But now i have mapped d+ and d- to gpio20 and 19 so now i need to remount the buzzer also the mpu , i think if i am rewiring , why not spi thts cleaner than i2c for a flight controller , so now i need conn_01x10 instead . Uf finally done now i need to add some good looking caps and then done the schematic , wait i have already spent 3hrs and basically did nothing but read datasheets and clculated the values for caps and wired some small things , ah! .

![schematic](assets/sch_v1.png)

Now when i went to robu for the caps , i saw those cheap as hell but then comes 10rs is min for a single comp , i really need to order whole 25 caps just to fullfill their condition of 10rs minimum , anyways bulk costs way cheaper , when i tried to find alts , i saw each costing 3rs while on bulk 
the same comp(from diff company) costs 0.40rs each. For the first time i am ordering more things to save money lol .

![robu](assets/robu.png)

Finally done the caps arc Imma gonna die if i sill continue .

![schematic](assets/sch_v2.png)

**Total time spent: 6 hours**

# Sept 6 : Footprint + pcb 

At firsted i started off with the usb connection , it might have looked finished but the old connection wd have caused the esp to struck in bootloader always so i did a easy trick diode between io0 and gnd done .

![diode](assets/diode.pngdiode.png)

Next the battery the one in my cart is a bonka 600mah 25c 1s lipo , and it have jst output but i cant get the exact size of the connecter , so i am just gonna leave the pcb with a conn_01x02 for the battery lol .

![battery](assets/bat.png)

next if footprints : it took me literal an hour to find the exact footprints , it was hell , metric vs normal size of smd . As i was doin it for the first time , i sucked even more still somehow i pulled it off . And for the sod diode section it was too confusing .

![footprints](assets/footprints.png)

Now after tht i went to the pcb , and i came to know i only have 90x26mm space for the pcb . I tried to route and saw my mpu isnt connected to anything just because i changed the pin names from the original footprint , ah! .  Now i have 2 options either to go with the old pin 1-10 or make my own footprint for the mpu , quite a hard decision ! i took the easy one and reverted .

![sch_last](assets/sch_v3.png)

now imma gonna do the pcb but before tht i have the frames size

![frame](assets/size.png)

i have measured i need 91x26mm^2 pcb tht will do the work probably. Now the main work routing the pcb . 
Why it looks this messy !!

![pcb](assets/pcb.png)

Actually i chosed the wrong footprint for the inductor too :x why it only happens with me ahhhhhhhhhhhhhhhhhhh. Ok i upgraded the footpring bt now it look like really big.

![inductor_smaller_footprint](assets/ind_sucks.png)

i literally tried to wire those every thing in just 2 layers its toooo hard for me ah!

![pcb](assets/pcb_v2.png)

lol I HAD TOO MANY ERRORS IN THE SCHEMATIC NOW NEED TO REFACTOR THE SCHEMATIC FIRST aAHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHH.Its already 6hrs ahh .

![sch_final](assets/sch_v4.png)

finally this time i examined it carefully before goin to pcb , now lets do the pcb shit again .
Ok this time i place the motor positions with a scale , i mean i measured the motors position wrt to the frame and placed the comps now need to wire .

![pcb](assets/pcb_v3.png)

Ahh again after another hour i kinda did the placement , take a look here 

![pcb](assets/pcb_v4.png)

Finally done i am crying , wait i didnt run drc .

![pcb](assets/pcb_v5.png)


**Total time spent: 8 hours**

# Sept 6 ptb :  Pcb polishing

DRC says i have errors but thts mostly trash as it says my tps63022's footprint has lower sized hole than the min , ahh my eyes are on fire ,i need to sleep still imma gonna fix it first .

![finally](assets/pcb_v6.png)

looks like fixed and now i have a new job which is to add the drone's frame and pcb both in freecad and then check . 
It had a lil screw fitting issue i fixed tht but forgot to sc ah ! 

![3d](assets/3d.png)

This time i cut the pcb a lil to make it favorable with the 3d frame and this is the final result 

![coool](assets/pcb_3d.png)

look like this is all i can do for today . Also looks like the designing part is complete ok then heres the final look: 

![Final](assets/3d_mod.png)

**Total time spent: 2 hours**

# Sept 7 : cost estimation + readme fix

Today i didnt do much things just estimated the cost of pcb manufacturing and 3d printing i tried diff platforms like robu and jlcpcb also lion circuits but for pcb and 3d both jlcpcb makes it cheaper so why not , also with a 10$ coupon the shipping is free .

![pcb order](assets/jlcpcb.png)

After that i tried a lot to find robu coupons too but i was unable to find so i basically gave up and went to readme , patched and thts all 

**Total time spent: 2 hours**

# Sept 8 : Wiring added to readme + bug found

today my work is minimal as i have a small exam , i just added the wiring portion to readme and found a bug ,
after reading the esp datasheet i came to know there is a strapping problem for gpio46 in my gerber ,
i mean when download mode with the usb , i need to pull tht down .

![strapping ladout](assets/strapping_err.png)

**Total time spent: 1 hours**

# Sept 9 : Writing the Firmware 

Today imma gonna write the firmware maybe minimal ai but mostly me and my no_std rust ^w^

Bruh why the hell did i place m1 and m4 as cw instead of m1 and m3 i am definately goin to forget tht .
Ok wrote the init section of the main.rs now loop{} is remaining so i need to write the others first like mpu , pid *
it already took me 1 hrs idk how but it is kinda complex in no_std . 
Ok i wrote the motor.rs too its easy , i think the hard one will be the integrating these all . 

Now i will let deepseek help me with the pid , its not like i cant do tht myself but i dont wanna :C,
done , its easy , deepseek explained how the ki kd kp work in this case and the logic is good enough but lol i cant examine it until
i dont get lux approved and build the drone's physical body . Still lets mod the  pid for my needs ..

so pid.rs done too , next is filter.rs:

ok ik this filter was supposed to be a normal part but it isnt at all i need to either take a old reference code or let deepseek cook , ahh what am i gonna do?

i just took an old reference , modded it as it as i want and then asked gpt to explain the logic and equtions now its done or atleast i think so its done .
Its kinda easier than i thought we are just converting the a/gx , a/gy ,a/gx to angular degree thts all , using the del time thts all easypeasy. Also idh a magnetometer thts why i am doin acc + gyro to measure the down direction and if the drone is tilted and with that i am balancing the drift , it is the frankest explanation i have ever given. Btw initially i had a int vs fp bug in the struct of ComFil but i spotted tht even before commit , as expected i use arch btw .

ok now whts remainin? oh the mpu lemme do tht too ,

on mpu its kinda like i am writting the mpu driver myself instead of using the standard mpu6500 library blah blah blah as i am on no_std rust . Its not hard just you need to find the exact resisters and need to make the logic to cleanly read the WHO_Am_I res and test if it is the right one and then read sensor data's from the output resister easy right ?
done the mpu too , now i am in a pretty good position .
WAH i made THE BIGGEST MISTAKE IN MY WHOLE LIFE

![baka moment](assets/motor_ctrl.png)

how the hell can i make such a mistake literal spelling mistake ? 

ok fixed tht and main.rs architecture side done too now i will push and let deepseek debug and then mineself will debug the bugs tht will be generated by deepseek while debugging my bugs lol .

ok added firmware to readme done .

**Total time spent: 6 hours**

# Sept 10 :

At first lets start with creating the env* , for me as an arch user i just did sudo pacman -S rustup espup thts why i use arch btw its perfect ,

![i use arch btw](assets/archbtw.png)

ok i just came to know my reference package api example was from a older esp hal HOLY .... now i need to migrate why it sucks on this level , i really dont wanna do it again . Luckily i didnt use the esp api's in filter and pid files , imma gonna let deepseek do this .

all settled it is migrated now so now i need to implant the esp-now logic , ok np .





