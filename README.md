# FWL_Rpi5_kstars_ekos_Hat_0_9

This repository is for a Raspberry Pi-5 Hat that provides various hardware control functions for Astrophotography equipment. 

Currently this repository only contains the KiCAD files, but I will add more photos and instructions to build your own as I progress.

Here is the link to order PCBs at OshPark:  https://oshpark.com/shared_projects/S5zMuqNM

This is the link to DigiKey Parts List: https://www.digikey.com/en/mylists/list/DTLNYIF8ZV

The DigiKey list includes 10% attrition, so depending on how good you think you are you can save a bit of money there.

The files in the "Linux Software" folder are system files and aliases to help you control the various functions of the hat.  The systemd files are written so that all of the DC power ports and Dew Heater ports turn on once the Raspberry Pi has completed its boot up.  The udev rules will allow you to control the Rpi GPIO ports from the terminal without using sudo.  The aliases, are shortcuts to turning the DC power ports and Dew Heaters on and off from a terminal.

