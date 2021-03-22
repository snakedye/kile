# A tiling layout generator for [river](https://github.com/ifreund/river)

## How does it work?

kile is entirely configured through river-options:

- main_idex : the index of the main frame
- main_amount : the amount of windows in the main frame
- view_padding : the padding of each window
- outer_padding : the padding of the output
- kile_frame : the layout of the frames where windows are contained
- kile_windows : the layout of the windows within each frame

## Building

```
git clone https://gitlab.com/snakedye/rivertiler.git
cd rivertiler
cargo build --release
```
