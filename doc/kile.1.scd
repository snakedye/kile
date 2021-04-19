kile(1)


#NAME
Kile - A tiling a layout generator for River


#DESCRIPTION
Kile is a layout generator for River.

It receives layout and option events to create a dynamic layout of windows.

The layout configuration is entirely done through River's option protocol.

Note: Kile could work on any Wayland compositor that implements
river-layout-unstable and river-options-unstable.


#OPTIONS
*-m <int>*, *--m <int>*, *--monitor <int>*
	The index of the monitor Kile will work on.

*-h*, *--h*, *--help*
	Display a help message.

*-d*, *--d*, *--debug*
	Enable verbose output.

*-n*, *--n*, *--namespace*
	The namespace of this layout generator.


#CONFIGURATION
Kile configured through River's option protocol or the riverctl ipc client.

To configure Kile you need to modify or initialize the value of a river option.
Some of them are already declared by riverctl(1).

The type following the name of the option is the one you need to give to riverctl
when you declare these options.

*layout (string)*
The layout namespace used to determine which layout should arrange this output.

*view_padding (uint)*
The padding in pixels of the each window within the layout.

*outer_padding (uint)*
The padding in pixels of the between the layout and the edges of the output.

*xoffset (int)*
The horizontal offset in pixels from a lateral screen edge.
Positive integers create an offset from the right of screen and negatives from the left.

*yoffset (int)*
The vertical offset in pixels from the top or bottom screen edge.
Positive integers create an offset from the top of screen and negatives from the bottom.

*main_amount (uint)*
An arbitrary positive integer indicating the amount of main views.

*main_factor (float)*
A floating point numger indicating the relative size of the area reserved for main views.
Note that layouts commonly expect values between 0.1 and 0.9.

*command <instruction>*
Execute a command within Kile.

You will most like set a command in this format.
<riverctl set-option -focused-output command "something">.

	That something would be
	- *smart-padding <true | false>*
		Enable of disable smart padding i.e gaps only if there's at least one view displayed.

	- *set-tag <focused | all | 0..32 >:<outer-layout>:<inner-layout..>:<app-id>*
			All of these field are escapable except the first
			which is the tag this command will be applied too.
			Layout will be explained later.

	- *clear-tag <all | focused | 0..32 >*
			Clears a tag configuration.

	- *preferred-app <app-id>*
			The app id of the application matching this string will be zoomed automatically.


##LAYOUT
Kile has a special way to generate layouts.

The screen geometry is split into smaller areas following the outer layout
and the layout within these areas are determined by the inner layout.
Inner layouts are generated from left to right in the order they are presented.
This allows you to create more complex layouts from the basic ones given.

Kile will never and should never give an empty area,
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

