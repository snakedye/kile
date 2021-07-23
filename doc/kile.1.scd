KILE(1) "https://gitlab.com/snakedye/kile" "General Commands Manual"

# NAME

kile - a tiling a layout generator for river

# DESCRIPTION

*kile* is a layout generator for *river*.

Through a lisp like syntax, users can define new dynamic layouts from existing ones. 

Note: *kile* could work on any Wayland compositor 
that implements *river-layout-v3*.


# ARGUMENTS

*-h*, *--h*, *--help*
	Display a help message.

*-n*, *--n*, *--namespace*
	The namespace of this layout generator.

# CONFIGURATION

On launch, the default layout is *Full*.

To configure kile you need to modify or set theses layout values using *riverctl*.

## LAYOUT COMMANDS

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
	The configuration of a tag. _0..32_ means all values between 1 and 32 inclusively.
	*focused* will set the _layout_ of the focused tag, *all*, all tags and for numbers
	between *1 and 32*, the _layout_ of the corresponding tag.

*clear* _(string)_
	Clear the configuration of the given tag(s)

	Possible values *all*, *focused* and an _int_ between *1 and 32* inclusively.

# LAYOUTS

kile only ships with the following layouts.

	- "f" | "ful" | "full"

	- "d" | "dec" | "deck"

	- "v" | "ver" | "vertical"

	- "h" | "hor" | "horizontal"

Instead of manually implementating all variations of each dynamic layout involving splits or stacking,
kile let's the user declare his own layout from existing ones through an s-expression like syntax.

Complete examples are provided in the layout file in contrib/.

## RECURSIVE

This layout is the core of every nested layout. It essentially let's you define a layout of layouts 
or in other words the disposition of layouts within one. Since it is itself a layout, fields can be a recursive layouts.

The layout generated follow these rules.

	- views at the top of the stack will be displayed in the main area.
	- when no inner layout is defined, no window geometry will be generated from this layout.


format: *{ layout : layout layout ... }* 

*examples:*
	- { v: h h }
	- { d: h { v: full dec } }
	- { { ver: h h } : ver ful }

## ASSISTED

This layout provides custom layout variables to a specific layout.

When this layout is not nested in another, it also serves to set the layout values for the tag.


format: *( layout main_amount main_factor main_index )* 


*examples:*
	- ( deck 0 0.6 0 )
	- ( { v: h h } 1 0.6 0 )


# SEE ALSO

*riverctl*(1), *rivertile*(7)
