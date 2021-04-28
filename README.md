# A tiling layout generator for [river](https://github.com/ifreund/river)

### Dependencies
- rust
- scdoc ( optional for man page )

## Layout Values

You can edit *kile*'s layout by setting or modifying layout values.

#### main_amount _(int)_

An arbitrary positive integer indicating the amount of main views.

#### main_factor _(fixed)_

A floating point numger indicating the relative size of the area reserved for main views.
Note that layouts commonly expect values between 0.1 and 0.9.

#### main_index _(int)_

An arbitrary positive integer indicating the index of the main area in the outer layout.

#### view_padding _(int)_

The padding in pixels of the each window within the layout.

#### outer_padding _(int)_

The padding in pixels of the between the layout and the edges of the output.

#### xoffset _(int)_

The horizontal offset in pixels from a lateral screen edge.
Positive integers create an offset from 
the right of screen and negatives from the left.

#### yoffset _(int)_

The vertical offset in pixels from the top or bottom screen edge.
Positive integers create an offset from 
the top of screen and negatives from the bottom.

#### smart_padding _(string)_

Enables or disables smart padding.
Possible values are "true" or "false".

#### set_tag _(string)_

Sets the tag configuration. All the fields except the first are escapable.
```
[focused | all | (int)]:[outer-layout]:[inner-layout]:[app-id | (int)]
```

#### clear_tag _(string)_

Clear the configuration of the given tag(s)
Possible values *all*, *focused* and an _int_ between *1 and 32* inclusively.

#### window_rule _(string)_
- *_app_id* (app_id)
- *_tag* (tagmask)

## Building

[![Packaging status](https://repology.org/badge/vertical-allrepos/kile-wl.svg)](https://repology.org/project/kile-wl/versions)

```shell
git clone https://gitlab.com/snakedye/kile.git
cd kile
cargo build --release
```
