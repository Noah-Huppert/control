![Control logo - tablet with volume sliders](/img/icon.png)  
# Control
Open source universal remote

# Table Of Contents
- [Overview](#overview)

# Overview
Control allows you to control any IR remote controlled device from your phone, 
the web, or a command line interface.  

The Control platform is comprised of 3 main components:

- Control Hub
	- IR blaster located in the same room as devices you wish to control
- Control Server
	- Bridge which connects Control Clients with Control Hubs
- Control Clients
	- User interface for controlling your devices
	- Available on the Web, iOS, Android, GNU Linux, Mac OSX, and Windows

These components work with the following primitives:

- Device
	- A physical piece of hardware in the room to be controlled
	- Receives commands from the Control Hub
- Remote
	- Physical remote used to control a device
	- Control Hub learns IR codes and associates them with buttons on a 
	  remote
	- Ex: TV Remote with actions: Volume Up, Volume Down, Channel Up, 
	  Channel Down
- Action
	- A single step which controls one aspect of a device by transmitting 
	  an IR signal
	- Ex: Turn on TV, or Switch to HDMI 2
- Play
	- A sequence of Actions
	- Ex: The "Play Games" Play has the actions: Turn on TV, Switch to 
	  HDMI 2
- Dashboard
	- Collection of related Plays which users can run
	- Ex: Living Room Dashboard has the Plays: Watch TV, Play Games, Listen 
	  to Music

#### Icons
Icons provided by [Icons 8](https://icons8.com/).
