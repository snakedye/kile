KILE(1) "https://gitlab.com/snakedye/kile" "General Commands Manual"

# NAME

kile - a tiling a layout generator for river

# DESCRIPTION

*kile* is a layout generator for *river*.

The idea behind kile is that it's supposed to the layout generator that does it all.
A few basic layouts are provided and through the *recursive* and *assisted* layout syntax, users can declare new unique nested layouts.

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

*focused*, *all* or *1..32* _(string)_
	Sets the tag configuration. _0..32_ means all values between 1 and 32 inclusively.
	*focused* will edit the layout of the focused tag, *all*, all tags and for numbers
	between *1 and 32*, the layout of the corresponding tag.

	format: "<layout> | <main_amount> | <main_factor> | <main_index>"

*clear* _(string)_
	Clear the configuration of the given tag(s)

	Possible values *all*, *focused* and an _int_ between *1 and 32* inclusively.

## LAYOUTS

The base layouts are the following.

	- "f" | "ful" | "full"

	- "d" | "dec" | "deck"

	- "v" | "ver" | "vertical"

	- "h" | "hor" | "horizontal"

If you wish to combine theses layouts the syntax is similar s-expressions.

## RECURSIVE

*Recursive* layouts are contained within brackets "{}".
	format: *{ layout : layout layout ... }* 

*examples:*
	- { v: h h }
	- { d: h { v: full dec } }
	- { { ver: h h } : ver ful }

## ASSISTED

*Assisted* layouts are contained within braces "()".
	format: *( layout main_amount main_factor main_index )* 

*examples:*
	- ( deck 0 0.6 0 )
	- ( { v: h h } 1 0.6 0 )



*riverctl*(1), *rivertile*(7)
