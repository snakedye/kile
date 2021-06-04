KILE(1) "https://gitlab.com/snakedye/kile" "General Commands Manual"

# NAME

kile - a tiling a layout generator for river

# DESCRIPTION

*kile* is a layout generator for *river*.

It receives layout events and creates a dynamic layouts of windows.

Note: *kile* could work on any Wayland compositor 
that implements *river-layout-v2*.


# ARGUMENTS

*-h*, *--h*, *--help*
	Display a help message.

*-n*, *--n*, *--namespace*
	The namespace of this layout generator.

# CONFIGURATION

On launch, the default layout is *Full*.

To configure kile you need to modify or set theses layout values using *riverctl*.

## LAYOUT VALUES

*main_amount* _(int)_
	An arbitrary positive integer indicating the amount of main views.

*main_factor* _(fixed)_
	A floating point numger indicating the relative size of the area reserved for main views.
	Note that layouts commonly expect values between 0.1 and 0.9.

*main_index* _(int)_
	An arbitrary positive integer indicating the index of the main area in a layout.

*view_padding* _(int)_
	The padding in pixels of the each window within the layout.

*outer_padding* _(int)_
	The padding in pixels between the layout and the edges of the output.

*xoffset* _(int)_
	The horizontal offset in pixels from a lateral screen edge.
	Positive integers create an offset from 
	the right of screen and negatives from the left.

*yoffset* _(int)_
	The vertical offset in pixels from the top or bottom screen edge.
	Positive integers create an offset from 
	the top of screen and negatives from the bottom.

*dimension* _(string)_
	A custom dimension of the output.

	format: "_(uint)_ _(uint)_ _(uint)_ _(uint)_"

*resize* _(string)_
	Declare if the output can or cannot be resized.

	Possible values are "true" or "false".

*smart_padding* _(string)_
	Enables or disables smart padding.

	Possible values are "true" or "false".

*set_tag* _(string)_
	Sets the tag configuration. All the fields except the first are escapable.

	format: "<tag>|<layout>|<main_amount>|<main_factor>|<main_index>"

	Values possibles for *<tag>*: "focused", "all" and _uint_.

*clear_tag* _(string)_
	Clear the configuration of the given tag(s)

	Possible values *all*, *focused* and an _int_ between *1 and 32* inclusively.

## LAYOUTS

The base layouts are the following.

	- "v" | "ver" | "vertical"

	- "h" | "hor" | "horizontal"

	- "t" | "tab" | "tabbed"

	- "f" | "ful" | "full"

	- "d" | "dwd" | "dwindle"

	- "D" | "Dwd" | "Dwindle"

## RECURSIVE

The syntax to create a *recursive* layout is "{}".
The content of the layout resides within the brackets.
*:* is the delimiter between the outer and inner layout. Both are property specific to the *recursive* layout.

*examples:*
	- {v:h,h}
	- {d:h{v:dwd,d}}
	- {{ver:dwindle,d}:tab,t}

## ASSISTED

The syntax to create a *assisted* layout is "()".
The content of the layout resides within the braces.
*;* is the delimiter between each field .ie *layout;main_amount;main_factor;main_index* 
to the *recursive* layout. Only characters are acceptable as layout denominators.

*examples:*
	- (vertical;0;0.6;0)
  - ({v:h,h};2;0.6;0)

*riverctl*(1), *rivertile*(7)
