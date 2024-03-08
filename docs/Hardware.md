# Hardware Overview

The following section describes the hardware setup for my homelab in case you would like to replicate it. The homelab configuration consists of 4 items. 

## Router
I ferget the exact modle, but it was probably a QUOTOM, I don't think the power really matters but rather, it is power consumption. I got the cheapest one with Intel NICs and lowest power consumption. Some people like to virtualize their router, which is an option, but I had no intention of debugging router issues running on a VM. 

## NAS
I have a Synology DS920+, which acts as my network's storage solution. The NAS has 4 drives which contain 4 2TB NAS-ready HDDs. 

## Switch and Access Points
I purchased a Unifi US-8-150W (130W) switch and a Unifi Wi-Fi 6 AP.
### A note about Ubiquity
While thier products work well, there is a requirement that you have their container available on the network for you to be able to make any modifications to the switch or AP (For example: If you'd like to update the Wi-Fi password). 

Thier security history is quite suspect. I have most of my configs and hardware behind a firewall so I guess that's somewhat comforting. 

A longwinded way of saying, I would not buy it again.

# Server
This is a low-powered fanless server designed for mobile vehiles. Most of the services we run do not take up a lot of resources, I don't care for transcoding, I just watch everything in the inteded resolution. The techinical specs of the CPU is: an 8-core  Intel(R) Core(TM) i7-5700EQ CPU @ 2.60GHz. 

Most of the time the 15 minute load averages does not exceed 0.03. It is overkill, I would recommend any used laptop that isn't a Raspberry-Pi. Certain docker containers do not fare well with ARM.
