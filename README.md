# A tiling layout generator for [river](https://github.com/ifreund/river)

## How do I generate layouts?

rivertiler comes with theses layouts hardcoded:

- ver
- hor
- tab
- cen
- dwd
- dwm

If you pass only one of this layouts to rivertiler, it will generate it. If you pass more,
the first argument is used to initialise the main frame were the following are generated.
The main frame will have the same amount of sub frames as the number of arguments you pass minus one.

A frame is basically a partition of the screen.

#### Exemple:
`rivertiler ver hor hor 0`

![rivertiler vertical horizontal horizontal](/img/1.png)

## More epic layouts
This is probably submit to change but here's how it works.

All windows, the output and views where layouts are generated are frames. A frame looks like this :
```rust
pub struct Frame{
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
    pub main_count: u32,
    pub main_index: u32,
    pub main_factor: f32,
    pub client_count: u32,
    pub state: State,
    pub layout: Layout
}
```
The layout generator only ouypuys the geometry at the end but all theses fields can be used to create your own layout.

The default layouts have a defined behaviour with some fields:
- x : the frame's x coordinate
- y : the frame's y coordinate
- w : the frame's x width
- h : the frame's height
- main_count : the number of clients in the frame
- main_index : the index of the main frame
- state : the state of a frame, if it's main or slave
- layout : the layout of the Frame

If you want to see more concrete example of layout, there are some in custom.
Imo it's easier to let the premade one do the heavy lifting (like cen).

## Building
#### Arch
It is available on the AUR under the name of `rivertiler-git`.
#### Manual
```
git clone https://gitlab.com/snakedye/rivertiler.git
cd rivertiler
cargo build --release
```

## Contributing
If you made a layout that isn't possible through command line arguments feel
free to make a PR.

This project will probably change a lot. Today it might a layout generator, tomorrow it might
a flying steno alien thingy!

####  Guidelines:
- All custom layouts are in src/custom.rs
- Layouts **must** fill the given frame

## TO-DOs

- [ ] Provide more integration with river-options

- [ ] Cooler stuff when river's layout protocol gets updated

