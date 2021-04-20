KILE(1) "https://gitlab.com/snakedye/kile" "General Commands Manual"

# NAME

kile - a tiling a layout generator for river

# DESCRIPTION

*kile* is a layout generator for *river*.

It receives layout and option events to create a dynamic layout of windows.

The layout configuration is entirely done through River's option protocol.

Note: Kile could work on any Wayland compositor that implements
river-layout-unstable and river-options-unstable.


# ARGUMENTS

*-m <int>*, *--m <int>*, *--monitor <int>*
	The index of the monitor Kile will work on.

*-h*, *--h*, *--help*
	Display a help message.

*-n*, *--n*, *--namespace*
	The namespace of this layout generator.

# CONFIGURATION

*kile* is layout generator configured through *river*'s
option protocol or the riverctl ipc client.

On launch *kile* will declare all the options it needs.
To configure it you need to modify the value of theses options.

## OPTIONS
*layout* _(string)_
	The layout namespace used to determine which layout should arrange this output.

*view_padding* _(uint)_
	The padding in pixels of the each window within the layout.

*outer_padding* (uint)
	The padding in pixels of the between the layout and the edges of the output.

*xoffset* _(int)_
	The horizontal offset in pixels from a lateral screen edge.
	Positive integers create an offset from 
	the right of screen and negatives from the left.

*yoffset* _(int)_
	The vertical offset in pixels from the top or bottom screen edge.
	Positive integers create an offset from 
	the top of screen and negatives from the bottom.

*main_amount* _(uint)_
	An arbitrary positive integer indicating the amount of main views.

*main_factor* _(float)_
	A floating point numger indicating the relative size of the area reserved for main views.
	Note that layouts commonly expect values between 0.1 and 0.9.

*command* _<instruction>_
	Execute a command within Kile.

## COMMANDS
Available commands.

	- *smart-padding <true | false>*

	- *set-tag <focused | all | 1..32 >:<outer-layout>:<inner-layout..>:<app-id>*

	- *clear-tag <all | focused | 1..32 >*

	- *preferred-app <app-id>*


## LAYOUT

kile has a special way to generate layouts.

The screen geometry is split into smaller areas following the outer layout
and the layout within these areas are determined by the inner layout.
Inner layouts are generated from left to right in the order they are presented.
This allows you to create more complex layouts from the basic ones given.

It will never and should never give an empty area,
no matter how funky your combinations may be.

The base layouts are the following.

	- *v* : vertical

	- *h* : horizontal

	- *t* : tab

	- *d* : dwindle(vertical)

	- *D* : dwindle(horizontal)

*outer-layout*
	A character from the list of layouts above.

*inner-layout*
	A string of layouts like "h" or "hDv".

*riverctl*(1), *river-layouts*(7)
