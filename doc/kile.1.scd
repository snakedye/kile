KILE(1) "https://gitlab.com/snakedye/kile" "General Commands Manual"

# NAME

kile - a tiling a layout generator for river

# DESCRIPTION

*kile* is a layout generator for *river*.

It receives layout and option events to create a dynamic layout of windows.

Note: *kile* could work on any Wayland compositor 
that implements *river-layout-v2*.


# ARGUMENTS

*-h*, *--h*, *--help*
	Display a help message.

*-n*, *--n*, *--namespace*
	The namespace of this layout generator.

# CONFIGURATION

On launch, the default configuration is `f:f` which would be 
a full outer and inner layout similar to monocle.

To configure and modify the layouts 
you need to modify or set theses layout values.

## LAYOUT VALUES
*main_amount* _(int)_
	An arbitrary positive integer indicating the amount of main views.

*main_factor* _(fixed)_
	A floating point numger indicating the relative size of the area reserved for main views.
	Note that layouts commonly expect values between 0.1 and 0.9.

*main_index* _(int)_
	An arbitrary positive integer indicating the index of the main area in the outer layout.

*view_padding* _(int)_
	The padding in pixels of the each window within the layout.

*outer_padding* _(int)_
	The padding in pixels of the between the layout and the edges of the output.

*xoffset* _(int)_
	The horizontal offset in pixels from a lateral screen edge.
	Positive integers create an offset from 
	the right of screen and negatives from the left.

*yoffset* _(int)_
	The vertical offset in pixels from the top or bottom screen edge.
	Positive integers create an offset from 
	the top of screen and negatives from the bottom.

*smart_padding* _(string)_
	Enables or disables smart padding.
	Possible values are "true" or "false".

*set_tag* _(string)_
	Sets the tag configuration. All the fields except the first are escapable.
	format: [focused | all | _int_]:[outer-layout]:[inner-layout]:[app-id]

*clear_tag* _(string)_
	Clear the configuration of the given tag(s)
	Possible values *all*, *focused* and an _int_ between *1 and 32* inclusively.

*window_rule* _(string)_
	- _app_id _app_id_
	- _tag _tagmask_

## LAYOUT

*kile* generate a dynamic layout or window layouts.

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

	- *f* : full

	- *d* : dwindle (vertical)

	- *D* : dwindle (horizontal)

*outer-layout*
	A character from the list of layouts above.

*inner-layout*
	A string of layouts like "h" or "hDv".


*riverctl*(1), *rivertile*(7)
