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

## COMMANDS

*(mod_)main_amount* _(int)_
	An arbitrary positive integer indicating the amount of main views.

*(mod_)main_ratio* _(float)_
	A floating point numger indicating the relative size of the area reserved for main views.
	Note that layouts commonly expect values between 0.1 and 0.9.

*(mod_)main_index* _(int)_
	An arbitrary positive integer indicating the index of the main area in a layout.

*(mod_)view_padding* _(int)_
	The padding in pixels of the each view within the layout.

*(mod_)outer_padding* _(int)_
	The padding in pixels between the layout and the edges of the output.

*xoffset* _(int)_
	The horizontal offset in pixels from a lateral screen edge.
	Positive integers create an offset from 
	the right of screen and negatives from the left.

*yoffset* _(int)_
	The vertical offset in pixels from the top or bottom screen edge.
	Positive integers create an offset from 
	the top of screen and negatives from the bottom.

*dimension* _(uint)_ _(uint)_ _(uint)_ _(uint)_
	A custom dimension of the output.

	format: 

*resize* _(bool)_
	Declare if the output's geometry relative to kile can or cannot be resized.

*smart_padding* _(bool)_
	Enables or disables smart padding.

*tag_order* _ascend/descend_
	The order in which the tags are sorted. If it's *descend*ing, the lowest tag will have priority when multiple are focused and vice versa.

*focused*, *all* or *1..32* _(string)_ _(layout)_
	The configuration of a tag. _0..32_ means all values between 1 and 32 inclusively.
	*focused* will set the _layout_ of the focused tag, *all*, all tags and for numbers
	between *1 and 32*.

	_string_ the name of the layout, seperated from the layout by a new line. This will be useful once river-toplevel-v1 is implementated. 
	_layout_ the layout definition of the corresponding tag. See #LAYOUTS.

*clear* _(tag)_
	Clear the configuration of the given tag(s)

	Possible values *all*, *focused* and an _uint_ between *1 and 32* inclusively.

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

	- views are spead evenly accross all "slave" areas.
	- views at the top of the stack will be displayed in the main area.
	- when no sub layout is defined, no geometry will be generated from this layout.


format: *( layout : layout layout ... )* 

*examples:*
	- ( v: h h )
	- ( d: h ( v: full dec ) )
	- ( ( ver: h h ) : ver ful )

## PARAMETERS 

This layout provides custom layout parameters to a specific layout.

When this layout is not nested in another, it also serves to set the layout parameters of the tag.


format: *( layout main_amount main_ratio main_index )* 


*examples:*
	- ( deck 0 0.6 0 )
	- ( ( v: h h ) 1 0.6 0 )

## CONDITIONAL

Let's you determine which layout to generate in an area depending on the amount of views occupying it. 


The accepted comparison symbols are
	- **>** (greater than)
	- **=** (equal to)
	- **<** (lower than)


The layout on the left hand side of the _?_ symbol will be generated when the condition is met
and the layout on the right hand side will be generated when the condition isn't. 


format: *( view_amount *_<|>|=_* layout ? layout )* 


*examples*
	- ( 2 > h ? v )


If the amount of views in that area is superior to 2, the layout in that area will be horizontal.
If the amount of views in that area is inferior or equal to 2, the layout in that area will be vertical.


# SEE ALSO

*riverctl*(1), *rivertile*(7)
