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

# Sept 10 : Starting with the remote 

At first lets start with creating the env* , for me as an arch user i just did sudo pacman -S rustup espup thts why i use arch btw its perfect ,

![i use arch btw](assets/archbtw.png)

ok i just came to know my reference package api example was from a older esp hal HOLY .... now i need to migrate why it sucks on this level , i really dont wanna do it again . Luckily i didnt use the esp api's in filter and pid files , imma gonna let deepseek do this .

all settled it is migrated now so now i need to implant the esp-now logic , ok np . No way this time again i will use old reference code 

![ref structs](assets/ref.png)

also i want to again say , crate is far better than pypi , "sometime truth hits hard " .  OK so imma gonna design the remote first easy peasy then the esp-now . i already have the required things in hand now , i mean i dont need to even buy a single thing(except a vero board and some headers to solder the esp i will buy it locally and by myself dont stress over orphy) so i thinks i shld build the remote first ok then if its decided then why not . My current esp(not the drone one but the one i have in home now ) is a esp32 devkitv1 likely wroom idk for sure but as i have it and 2 joysticks , i dont even need a pcb i will solder these to vero board and make it work . wait i found a old vero board too in my salavaged parts lets goooo!!

![salavaged parts](assets/comps.png)

so here i am goin to solder all of those on tht small vero board but idk where to mount the joysticks . ok mounted the joystics but my soldering is way too bad for this task . 
![joystick](assets/joystick.jpeg)

nah i am out i will just make a pcb for it too or buy a diff iron , its too hard with this freaking iron i just cant even solder a led with it , i tried to re tin still it didnt work and after a lot of tests it exploded lol luckily i didnt got hurt but my remote is strucked now imma gonna do the software first or even just use wifi ahhh it sucks as hell still the final looks . 

![it sucks](assets/wireout.jpeg)

**Total time spent: 5 hours**

# Sept 11 : Remote's pcb

I have got a sick idea instead of a physical remote , i will just connect the esp to my phone via serial and use the esp with esp-now and my phone for controlling(as esp-now is 1Mx times better than just wifi ) ok so i am currently thinkin what shld i use a web ui or a kotlin app , its kinda hard for me to chose but anyways frontend = deepseek is cooking so i dont need to worry . Wait why am i not making a seperate pcb for the remote ? ok perfect for poc , phone but as idh much works so i will be designing the pcb too in parallel . 

 Ok started the schematics , i againn used the same module for esp , and tps again and built the schematic , heres it

![schematic](assets/rm_sch.png)

ok next is pcb . Ok pcb done too

![pcb](assets/rm_pcb.png)

wait why my 5v and 3.3v have same power rail??

![prob](assets/batt_err.png)

bruh it hella sucks wait when i highlighted i came to know its gnd shared not 5v wtf am i blind now?

![err](assets/sch_rm.png)

ok pcb done likely but i feel like i am forgeting something anyways take a look :

![done](assets/done_pcb.png)

ohhhhh i get it what i forgot the mounting holes for the joysticks ok ok lemme add tht too

![done](assets/final_rm.png)

done finally the remote too , now i need to upgrade the bom but not only tht i wanna show ya guys a really funny thing 

![lol](assets/3d_lol.png)

this is the 3d pic but lol my footprints are likr my buzzer is a 2 pin diode and my pins for the joystick looks hillarious .
ok today did a lot of heavy lifting now matane next day .

also now i tested it in jlcpcb take a look
![jlcpcb](assets/jlc.png)

but theres a huge problem , the mpu6500 is out of stock same goes for mpy9250 so now i need to switch to mpu6050 , hell naw i will need to change my gerber , code and a lot of more things ahhh it sucks .Ok i found a alt in robu but it costs likely 8$ while the old one was barely 1.5$ nah not gonna waste this much money instead i will use a mpu6050 and redesign . ok so thts the task for tomorrow lemme upgrade the bom for now

**Total time spent: 6 hours**


# Sept 12 : Restart

The previous day , my reviewer told me my pcb is half cooked , but after watching a tutorial i came to know why he said it .
So this time i am goin to document it like a pro , heres the first schematics portion

![restart](assets/restart.png)

The esp , the tps , the bl5612's , the mpu every thing will be separated this time also according to the datasheet, the gpio19 and gpio20 are D+ and D-

![data](assets/data_1.png)

next is connecting all the motor drivers with good enough pins .

![done](assets/restart_2.png)

done now i need to connect the mpu , as the breakout is always cheaper in india , why not ?

![esp](assets/esp.png)

i think i can just mount the mpu on top of the esp done  and a double tape will do the vibration avoidance thing .

![schematic](assets/resch_1.png)

This time i added a bmp180 too with the mpu6050 , also fixed the esp's en funcition by adding a 10k ohm res between 3.3v and en instead of the old direct connection .

after it , i added the caps needed on bl5612 ic's

![caps_added](assets/cap_bl.png)

ok now i think imma gonna add the motors , thts easy .
Done already

![motors_added](assets/motor_sch.png)

Now i think i have to add the caps on the tps and on the esp , on the mpu and bmp too . 

done i added a lots of caps this time , for the motors , for esp , for the mpu .....

![caps](assets/rest_3.png)

Now i need to add a button to en for reset i think a trace wd be just fine no button needed at all .

![trace_addedd](assets/trace.png)

ohh i actually wired the en cap wrong , it was on 3.3v instead of en , i fixed tht also changed the cap value to 10uf as idc if the delay is 100ms .

This time i ran drc , fixed some minor probs and added a battery , also 4 22uf caps in parallel :

![final](assets/final_sch.png)

Ok after it i added all the footprints , this one took me a lot of time really a lot of time 

![footprint](assets/footprints_assi.png)

Bruh wtf did i do wrong?? i got some big ic's instead of my poor resistors , lol needa fix tht 

![lol](assets/wtf.png)

ok i fixed tht now lets dive into pcb wait maybe i shld save tht for tomorrow i already did a lot of work today !

![fixeddd](assets/fix.png)

**Total time spent: 8 hours**

# Sept 13 : Starin the pcb

![pcb started](assets/pcb_res.png)

placed the bl5612's onto their right place now lets see what can my small raccon like brain do?

Bruh wtf i really placed another tps instead of my inductor , footprint is really hell .

![fixed footprint](assets/ind_fx.png)

ok fixed it .

btw the barometer looks really big , needa fix its size .

![done](assets/bar_fix.png)

done now i need to place everything on right place then seperate the ground ............and alll the tasks.

wait lemme resolv the 3d model first or i wont be able to mount the pcb . 

ok as shadow said , my own fcstd , done 

![iusearchbtw](assets/right.png)
---
![iusearchbtw](assets/side.png) 
---
![iusearchbtw](assets/top.png)
---

done the model looks really sick
path is :
[path to model](./3d/lux.FCStd)

i took the old model as reference thts why it went this fast , still took me a bit time as i rebuild the geometry .

ok now the pcb , now finally its favorable as i have the model done but wait i forgot the mounting holes uhhh it sucks .

![comps placed](assets/ol.png)

placed all the components on their right position now i need a bit adjustment and then placement done

![placement](assets/nw.png)

looks like done but now some finishing ,
![cropped](assets/crped.png)

i cropped the pcb and fixed the placing looks like thts all before connection .

oh man the 3d model really ate all my time .

hm so i made filled zone for the gnd , and started routing 

![motorsss](assets/moto.png)

i added the thick af trace for it .

![via](assets/via.png)

this time i made via's a lot more fat for more power draw . also connectd other things ,

![usb](assets/usb.png)

then i did the usb part , i used really small 0.1mm traces as d+ and d- doesnt carry much power, after tht i wired the tps to inductor , and more tht i forgot check the photo for it.


**Total time spent: 8 hours**

# Sept 14 : Finishing the pcb

today i wanna power the motors correctly :

![pwr](assets/mpwr.png)

done the right sided motors are fine now . phew!

![pcb](assets/pcbb.png)

this time i connected the usb c , the tps a bit more , and connected the resistors with the esp , its just some minor job . after a lil more connection it will be done . 

![done](assets/pcb_dne.png)

ok done the pcb looks cool
![3d](assets/pcb3d.png)

heres the 3d model , it looks damn cool .

![power](assets/pwrr.png)

just take a look at the tps portion i made it tightly packed looks really cooool .

![3dd](assets/3dd.png)

ahh i placed the mounting holes wrong now i need to replace them correctly ok lets do tht 

done

![3d](assets/3d_.png)

wait but the edges are out of the frame needa fix tht eaxy job , cut the pcb baby.
no wait imma not gonna cut the pcb now , it will make me do a lot of things again.

![jlc](assets/jlccccc.png)

lol the small via  pcb costs more than all the comps together , lol again .

so i need to make the via's bigger ah .

![vias](assets/jllc.png)

fixed it uh now atleast it costs less than the comps .
![jlcpcb](assets/jlccc.png)

ok now i need to do the remote .

**Total time spent: 5.5 hours**

# Sept 15 : Remote schematics

i just realized i was wastin resources by allowcating a whole  esp32 s3 n16r8 to the remote , and i found esp32 c3 tht costs half the cost of the s3 and also as thts a breakout i am savin a lot of money too lol .
lol i cant find the footprint of esp32 c3 super mini , i tried gemini , gpt , google , no one was able to find it , looks like needa make it myself or make gpt make it for me haha buhhahhahhaha .

wait i found it , from reddit lol . but its footprint only no symbol i need symbol too so i think i might make the symbol myself . 

[the random reddit link](https://drive.google.com/file/d/1Ia2y5KgLuTCjiLleW4krdE9uNQLiYWtD/view)

now i will need to find or make the sym , wait its the s3 supermini not c3 super mini ahhhh life sucks ....

ok i will ditch the c3 supermini idea .. so what i am supposed to do now? maybe ditch the remote and controll with a phone? lol lets do tht ,nah ths not something like me to do , 
ok gah then imma gonna build with a esp module not breakout but tht costs too much for a basic thing , wait wait i can use my existing esp ah but tht one doesnt hav any external antenna , idk what to do now , i will just go with the c3 super mini thts good , i cant even do phone seria to my esp as my esp has uart instead of native esp like the s3 or c3 series . 

ohhhh i found gold take a look :

![Peter's electric trick and electronic blog](assets/esp.png)

oh i can do it to make my esp do the work already why worry ? 

wait even better ESP-M1 ESP8285 found a esp8266 and it is far cheaper imma gonna use it . wait its out of stock lol , maybe i shld go ahead and use my esp32-s3 as planned at first lol .  i checked Espressif ESP32-S3-MINI-1U-N8 Module but 
![esp](image.png)
it has lower sensivity so only option original route ahhhh i just wasted 5 hrs to save a few hundred rs and failed lol .

![lol](assets/rmote.png)

ok thts enough i gotta sleep now or gonna die.

**Total time spent: 7 hours**

# Sept 16 :  

ok i started today with some research , looks like imma gonna reuse my joysticks and then but for tht i need a soldering but currently i only have a hot air station lol lets use tht 

![sw](assets/sw.png)

added switchs for the schematics  , now i think i shld design the pcb instead .

![pcb](assets/pcb_rm.png)

started the pcb 

![done halfways](assets/pdne.png)

ah lol i did a lot of work still its no t even halfways done 

![done](assets/dne.png)

done finally i think now i can sleep in ease but wait idh any buzzer or led , let tht go just lets take it to jlcpcb i am realy sleepy .

wait arent the buttons too big??

and the vias are too small to manufacture , so i need to make them bigger , fixed them now need to rerun the drc 

![drc](assets/vias.png)

ran drc again , min dia size was err'ing so i changed all 

![viasss](assets/viass.png)

oh holy sh*t! now i need to fix it for the drone tooooo .

![rt](assets/rt.png)

this is the 3d, but i tried turning rt on , and lol my i5 1135g7 on zen 7.2.3 , got absolutely roasted af .

i will do the drones pcb and remotes cad tomorrow matane

**Total time spent: 7 hours**

# Sept 17 : cad for remote

lol lemme add the joystick footprint first , how can i forget tht ?

also i considered using a custom vtx but the costs lol , it crosses the 50$ as mine is t3 lol lets stic to the plan 

wow wow found a perfect antenna 

![antenna](assets/ante.png)

ts is 2.4ghz and dipole perfect , i read the dataset too its damn good. now need to fix the remote lol .
wait i need to remove the switchs to somewhere else as i need to use adc-1 on my esp for the nrx and nry of my joysticks as espnow shares a bit with the adc-2 lane
maybe? as it does wit wifi too !

![joystck](assets/joy.png)

done added the joystick support on the schematic now need to add joystick on pcb too.

![led](assets/led.png)

added led too , but findin it and the resistors were a really boarin task still did it and found the cheapest one as usual .

![res](assets/ledr.png)

added a led for the c type usb too and as its 5v i used 2x 220 ohm res on serial as on bulk its cheaper , 330ohm might give me more brightness but why waste money?

![done](assets/trc.png)

done finally now i need the .step for the joysticks

![grabcad](assets/3d_j.png)

grabcad is the best i found the model in just a minute .

added the 3d model for the joystick

![lol](assets/jystck.png)

lol mounted on the wrong direction lemme fix tht .

![joystick](assets/jystckk.png)

done but wait i didnt do the free cad thing today at all , ahhh ok gonna do tomorrow lemme export the pcb .step first 

done!

![sch](assets/schhh.png)

it looooks tooo odd , the schematic led portion but like i care , it works , it is documented , it is separated and tagged thts all i care abt lol 

**Total time spent: 5 hours**

# Sept 18 : 

![jy](assets/jy.png)

i placed th joystick wrong needa rotate tht .

![done](assets/jyy.png)

fixed it  lol . it was problamatic.

![switch](assets/sww.png)

added the joystick switch connection to sch now next is pcb .

![don](assets/rmt.png)

added to pcb too , now lets make the case for the remote

i expanded the pcb , and tried to adjust atleast one screw hole for the joystick and this is my 5th attempt lol

![screw screw](assets/screw^2.png)

still cant do tht maybe the 7th time, thts my lucky number ,

![13](assets/13.png)

lol its the 13th time still got only one screw on right place ,

![finally ](assets/fianlly.png)

finally now both are on right places uf.

![remote](assets/rmtt.png)

thts how it looks i added the screws and not it looks good enough but the switchs are too small to even reach the surface of the cad ima gonna make .
anyways it means lets start with the cad , no i dont wanna it sucks ahhh but i need to do tht anyways lol .

made the gerber and exported the step now next is cad .....

![started](assets/cadd.png)

started now need to make another sketch and rm the pcb body volume 

![cadd](assets/caddd.png)

thts how it looks not i think i will go with minimal cover on the topside .

ohh holy sht i forgot the usb c anyways gonna do tht tomorrow .

**Total time spent: 4.5 hours**

# Sept 19:

ok i just saw my joysticks arent alligned AHHHHHHHH , wtf and i did a bit work in the cad , it sucks as hell now i will need to redo it , ok lets fix tht atleast.

![done](assets/sucks.png)

this time i wont do tht err again .

![fian](assets/sym.png)

finally it looks symetric now !

![antenna](assets/ant.png)

after reading the datasheet , made a hole for the antenna , perfect size right?

![pocket](assets/c.png)

perfect position to make a hole lol , i made the hole for usb c too now need to add a pad to cover the other side hole too !

![filled](assets/filled.png)

ok filled the hole on the other side , now need to rm the extra fillin .

![done](assets/dnee.png)

now just need to make the upper cover for the remote and done easy peasy 

![remote](assets/rm_c.png)

doesnt look tht cool but if it works , it works .

![done](assets/ddoonnee.png)

now its comepletely done , added bosses to screw the pcb in too , now its done , just need to tune the hole for the pcb with the joystick a bit and done , easy lol

![fixed](assets/ffixed.png)

fixed the screw mountin prob so am i done ? already?

![1](assets/rrmt.png)

so this is the final look of the the remote , now gonna export stl .

**Total time spent: 3 hours**

# Sept 20 : Today we're codin 

at first i changed the names frm mpu6500 to mpu6050 in the code , also the WHO_AM_I resister value frm the configureable 0x70 and 0x73 in mpu6500 to 0x68 in mpu6050 , oh man readin datasheets sucks . i am not gonna touch any other resisters anyways , even it doesnt even exist on mpu6050 , why clean tht up . Maybe i use a mpu6500 on v2 ? thts why i wont remove them . 

Mizuketa another change , the temps values 

![done](assets/fr.png)

now wht shld i change ? ohhh the api's of the whole code lol forgot them all . 
done thts all for today i am sleepy

**Total time spent: 0.5 hours**

# Sept 21 : Found a major bug 

today i looked at the pcb AND I FOUND A SEVERE BUG , i didnt connect the correct power pins , i mean 
![pcb_err](assets/pcbbb.png)

see this ,the gnd and gnd1 are separated as intended but the 5v and 5va are separated too thts a prob needa fix tht , i think i need diodes only thts enough lol

thx god , i examined the pcb with a lot of time instead of just submittin it .

ok now its getting complex the gpio0 portion , i thought i wd just use a diode between 3.3v and gnd BUT I AM USING ONLY 1 TPS so the 3.3v will be powered if usb plugged , so OHHHHHH I have a perfect solution , usb power , get lost , i can just use battery to power the mcu all the time and the usb will only do d+ and d- , am i not a freakin genious?

AH but now idh the usb power detector lol

Oh i can use the d+ as a detector i mean i can use a nand gate and done hell yeah 

oh wait i can just use the usb's 5v why disturb my d+ , wait wait if its the case why dont i just use my current architecture but use the battery's 5va and use tht as the battery power and gnd is when usb plugged?

ah lol , lemme change tht section

![tps](assets/tps_fxd.png)

fixed the tps portion now need to do same with the esp .No wait ,now the 3.3v is permanent , even worse , i cant use this now 

![fool](assets/fool.png)

so i cant use my utterly fool design , hm .

LOL i just read the datasheet again and it said , GPIO0 is pulled high default but if some brat wanna pull it up with e res , no prob lol.

i just never needed tht thing ahhhhhh .

maybe i will add a led for the usb connection . but nah not now , i am already tired readin datasheets for a hour.

![fixed](assets/fiixed.png)

now needa fix the pcb .

![gnd1](assets/gnd.png)

ohh hell , its such a mess 

![fixed](assets/fiixxed.png)

fixed this mess a lot uf.

![finally](assets/fxd.png)

finally routed everything now just need to export the gerbers again uf .

ok done now i can push and sleep finally .

**Total time spent: 2.5 hours**

# Sept 22 :

As i thought the remote also has the same prob lol

![lol](assets/llol.png)

now needa fix tht , so again if i do this , the remote will be battery mandetory but idh any place for battery on the case , maybe i will add one small battery and mount it somewhere like under the remote , not a prob . 

Holy shit , my laptop got shut down in mid and gave me casps  specific err , like youre dead . i was scared and teared down the whole laptop down , and recovered it now the "s" button is acting odd and touchpad not workin ahhhhhhhh tht s is jusst getting everywhere ahhh . ok back to work . 

How the hell can i work with this piece of shit , S is just gettin spammed .

i cant even use kicad in peace ahhhh , i thought after fixing the pcb , i wd supgrasde the readme but its not possible . 

ok fixed it but with the cost of my steak of 18 days and a external keyboard ah .

so wht shld i do now? ok enough today i basically did nothin . 

**Total time spent: 1 hours**