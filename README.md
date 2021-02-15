# A tiling layout generator for [river](https://github.com/ifreund/river)

## How do I generate layouts?

rivertiler comes with theses layouts hardcoded:

- tab
- left
- center
- dwindle
- vertical
- horizontal
- dwindle_mod

If you pass only one of this layouts to rivertiler, it will generate it. If you pass more,
the first argument is used to initialise the main frame were the following are generated.
The main frame will have the same amount of sub frames as the number of arguments you pass minus one.

A frame is basically a partition of the screen.

#### Exemples:
`rivertiler vertical horizontal horizontal`
- This will generate a master and stack layout practically identical to `rivertile left`.

![rivertiler vertical horizontal horizontal](/img/1.png)

`rivertiler dwindle_mod dwindle horizontal tab`
- This will generate a dwindle_mod layout with three frames that takes the entire screen,
the first frame has a dwindle layout, the second a horizontal split layout and
the thrid a tabbed layout.

![rivertiler dwindle_mod dwindle horizontal tab](/img/2.png)

## More epic layouts
The more complicated layouts are inside src/custom.rs.

Ideally you would never need to edit the source code but I don't plan to provide the ability
to generate nested layouts inside nested layout inside... from the command line.

Layouts take a `Frame`, a vector of `Frame`, along with some other information like the window count,
the multiplier factor and master count and return a vector of frame. You can make
a completely independant layout (without using other layouts) or use the one already made to generate new one.

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
- Simple layouts that do not require others should go in src/layout.rs
- Those that require others or call other functions should go in src/custom.rs
- Layouts **must** fill the given frame

## TO-DOs

- [ ] Provide more integration with river-options

- [ ] Cooler stuff when river's layout protocol gets updated

