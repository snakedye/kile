# A tiling layout generator for [river](https://github.com/ifreund/river)

### Dependencies
- rust
- scdoc ( optional for man page )

## Commands

You can send a command to *kile* by setting a value to the `command` option.

```shell
  riverctl mod-layout-value <namespace> string command ...
```

### `set-tag focused:v:Dh:firefox`

Declares the configuration of a tag.

This command is set the outer layout of the tag to vertical (v)
and the inner layout to [ dwindle (D), horizontal (h) ].

The last element is the window. An application matching that 
app_id or tagmask will be automatically brought to the main area.

All the fields except the first are escapable i.e you need to say 
which tag you want to declare or edit but can ommit to put something in
all the other fields. Which means `set-tag 4::hh`
or `set-tag all:::Chromium` are also valid commands.

### `clear-tag all`

Clears the configuration of all the tag.
The default configuration is `f:f` which would be 
a full outer and inner layout similar to monocle.

### `window-rule ( app_id | tagmask )`

The **app_id** is the preferred application of the focused tag.
The **tagmask** is the bitwise integer corresponding to a tag.
Apps from this tag will be zoomed

### `smart-padding (true | false)`

Enables or disable smart padding.

See `kile(1)` or `doc/kile.1.scd` for more info.

## Building

[![Packaging status](https://repology.org/badge/vertical-allrepos/kile-wl.svg)](https://repology.org/project/kile-wl/versions)

```shell
git clone https://gitlab.com/snakedye/kile.git
cd kile
cargo build --release
```
