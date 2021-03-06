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

*(default_)(mod_)main_amount* _(int)_
	An arbitrary positive integer indicating the amount of main views.

*(default_)(mod_)main_ratio* _(float)_
	A floating point numger indicating the relative size of the area reserved for main views.
	Note that layouts commonly expect values between 0.1 and 0.9.

*(default_)(mod_)main_index* _(int)_
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

*resize* _(bool)_
	Declare if the output's geometry relative to kile can or cannot be resized.

*smart_padding* _(bool)_
	Enables or disables smart padding.

*order* _ascend/descend_
	The order in which the tags are sorted. If it's *descend*ing, the highest tag will have priority when multiple are focused and vice versa.

*default*, *focused*, *all* or *1..32* _(string)_ _(layout)_
	The configuration of a tag. _0..32_ means all values between 1 and 32 inclusively.
	*focused* will set the _layout_ of the focused tag, *all*, all tags and for numbers
	between *1 and 32*.

	_string_ the name of the layout, seperated from the layout by a new line. This will be useful once river-toplevel-v1 is implementated. 
	_layout_ the layout definition of the corresponding tag. See *LAYOUTS*.

*clear* _(tag)_
	Clear the configuration of the given tag(s)

	Possible values *all*, *default*, *focused* and an _uint_ between *1 and 32* inclusively.

# LAYOUTS

kile only ships with the following layouts.

	- "f" | "ful" | "full" 

	- "d" | "dec" | "deck"

	- "v" | "ver" | "vertical"

	- "h" | "hor" | "horizontal"

Instead of manually implementating all variations of each dynamic layout involving splits or stacking,
kile let's the user declare his own layout from existing ones through an s-expression like syntax.

Examples are provided in the layout file in contrib/.

## RECURSIVE

This layout is the core of every nested layout. It essentially let's you define a layout of layouts 
or in other words the disposition of layouts within one. The _:_ symbol is the delimiter between a layout and its _sublayouts_.

The layout generated follow these rules.

	- views at the top of the stack will be displayed in the main area.
	- views are spread evenly accross all other areas ie those that aren't main.
	- the areas generated by layout on the left side of _:_ are for the _sublayouts_ not views. 
	- the area generated at _main_index_ will have _main_amount_ of views/areas unless _main_amount_ is 0.
	- this layout will *always* create as much areas as necessary to inhabit the amount of views assigned to its area.


format: *( layout : layout layout ... )* 

*examples:*
	- (v: h h)
	- (d: h (v: full dec))
	- ((ver: h h) : ver ful)

## PARAMETERS 

This layout provides custom layout parameters to a specific layout. If a parameter isn't specified, the layout will inherit the one from his parent.

When this layout is not nested in another, it also serves to set the layout parameters of the tag.
Note that not all parameters need to be defined, if you want to escape one, just insert an invalid character.

format: *( layout main_amount main_ratio main_index )* 


*examples:*
	- (vertical 2)
	- ((v: h h) 1 0.6 0)

## CONDITIONAL

Let's you determine which layout to generate in an area depending on the value of certain parameters. 


The accepted comparison symbols are
	- **>** (greater than)
	- **=** (equal to)
	- **<** (lower than)


Symbols associated to parameters
	- **?** : the amount of views/areas in that area
	- **!** : the index of the main area in that layout
	- **%** : the ratio of the main area relative others


format: *( view_amount *_<|>|=_* layout *_!|?|%_* layout )* 

The layout between the _operator_ and the _parameter_ symbol will be generated if the condition is true.
In the case it's false the layout on the right side of the _parameter_ symbol will be generated.


*examples*
	- (2 > h ? v)
	- (0.6 > (v: h h) % f)


# SEE ALSO

*riverctl*(1), *rivertile*(7)
